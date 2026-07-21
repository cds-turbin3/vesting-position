//! Clawback, cancel, close, and receipt-lifecycle tests on the source-free world.

use solana_pubkey::Pubkey;
use vesting_babelfish_tests::common::{
    fund_keypair, load_whitelist_user, whitelist_allocation, WhitelistUser, LAMPORTS,
    WHITELISTED_1, WHITELISTED_2,
};
use vesting_babelfish_tests::merkle::{default_merkle, random_proofs, MerkleTree};
use vesting_babelfish_tests::world::{Action, CampaignConfig, VestingWorld};

#[track_caller]
fn setup() -> (MerkleTree, VestingWorld) {
    let merkle = default_merkle();
    let world = VestingWorld::initialized(&merkle, CampaignConfig::default());
    (merkle, world)
}

fn mint_alice_position(merkle: &MerkleTree, world: &mut VestingWorld) -> (WhitelistUser, Pubkey) {
    let alice = load_whitelist_user(world, merkle, WHITELISTED_1);
    fund_keypair(world, &alice.keypair, LAMPORTS);
    let asset = world.asset_for(&alice.keypair.pubkey());
    world.first_claim_ok(&alice.keypair, alice.proofs.clone(), alice.allocation);
    (alice, asset)
}

/// Claim at ~50% of the linear window so the position is partially vested.
fn partial_claim(world: &mut VestingWorld, user: &WhitelistUser, asset: Pubkey) {
    world.warp_to(world.linear_checkpoint(50));
    world.subsequent_claim_ok(&user.keypair, asset);
}

// --- clawback (existing position) ------------------------------------------

/// Burns the asset and returns the unclaimed remainder to the creator.
// Report: ../test-report/clawback_after_grace_burns_asset_and_recovers_remainder.md
#[test]
fn clawback_after_grace_burns_asset_and_recovers_remainder() {
    let (merkle, mut world) = setup();
    let (alice, asset) = mint_alice_position(&merkle, &mut world);

    partial_claim(&mut world, &alice, asset);
    let claimed = world.claimer_token_balance(&alice.keypair.pubkey());
    assert!(claimed > 0 && claimed < alice.allocation);

    world.warp_past_grace();
    let creator_before = world.creator_token_balance();
    world.clawback(asset);

    assert!(
        !world.asset_is_valid_mpl_core(&asset),
        "asset must be burned"
    );
    assert_eq!(
        world.creator_token_balance(),
        creator_before + (alice.allocation - claimed)
    );

    // Past the grace window the claim instruction is closed entirely.
    world.subsequent_claim_err(&alice.keypair, asset, "ClaimWindowClosed");
}

/// Cannot clawback before end + grace_period.
// Report: ../test-report/clawback_before_grace_fails.md
#[test]
fn clawback_before_grace_fails() {
    let (merkle, mut world) = setup();
    let (_alice, asset) = mint_alice_position(&merkle, &mut world);

    world.warp_past_end(); // past end, but inside the grace window
    world.clawback_err(asset, "GracePeriodNotOver");
}

/// Fully claimed loyalty badges are inviolable.
// Report: ../test-report/clawback_fully_claimed_badge_fails.md
#[test]
fn clawback_fully_claimed_badge_fails() {
    let (merkle, mut world) = setup();
    let (alice, asset) = mint_alice_position(&merkle, &mut world);

    // Fully claim inside the window, then move past grace for the clawback.
    world.warp_past_end();
    world.subsequent_claim_ok(&alice.keypair, asset);

    world.warp_past_grace();
    world.clawback_err(asset, "AlreadyFullyClaimed");
}

/// Only the campaign creator can clawback.
// Report: ../test-report/clawback_requires_creator.md
#[test]
fn clawback_requires_creator() {
    let (merkle, mut world) = setup();
    let (alice, asset) = mint_alice_position(&merkle, &mut world);

    world.warp_past_grace();
    world.clawback_by(&alice.keypair, asset, "Unauthorized");
}

// --- clawback_unclaimed (never-claimed allocation) --------------------------

/// Recovers the full allocation and permanently blocks a late first claim.
// Report: ../test-report/clawback_unclaimed_recovers_allocation_and_blocks_claim.md
#[test]
fn clawback_unclaimed_recovers_allocation_and_blocks_claim() {
    let (merkle, mut world) = setup();
    let bob = load_whitelist_user(&mut world, &merkle, WHITELISTED_2);
    fund_keypair(&mut world, &bob.keypair, LAMPORTS);

    world.warp_past_grace();
    let creator_before = world.creator_token_balance();
    world.clawback_unclaimed_ok(bob.keypair.pubkey(), bob.allocation, bob.proofs.clone());

    assert_eq!(
        world.creator_token_balance(),
        creator_before + bob.allocation
    );

    // Bob's late first claim is blocked: the window is closed (and the
    // receipt created by the clawback blocks it as defense in depth).
    world.first_claim_err(
        &bob.keypair,
        bob.proofs.clone(),
        bob.allocation,
        "ClaimWindowClosed",
    );
}

/// Invalid merkle proof is rejected.
// Report: ../test-report/clawback_unclaimed_rejects_invalid_proof.md
#[test]
fn clawback_unclaimed_rejects_invalid_proof() {
    let (merkle, mut world) = setup();
    let bob = load_whitelist_user(&mut world, &merkle, WHITELISTED_2);

    world.warp_past_grace();
    world.clawback_unclaimed_err(
        bob.keypair.pubkey(),
        bob.allocation,
        random_proofs(),
        "InvalidProofs",
    );
}

/// Cannot clawback_unclaimed a recipient who already claimed (receipt exists).
// Report: ../test-report/clawback_unclaimed_fails_if_already_claimed.md
#[test]
fn clawback_unclaimed_fails_if_already_claimed() {
    let (merkle, mut world) = setup();
    let (alice, _asset) = mint_alice_position(&merkle, &mut world);

    world.warp_past_grace();
    world.clawback_unclaimed_err(
        alice.keypair.pubkey(),
        alice.allocation,
        alice.proofs.clone(),
        "AlreadyClaimed", // receipt records a real first claim
    );
}

/// A buyer's subsequent claim creates a zeroed receipt at the buyer's PDA.
/// That must not strand the buyer's own never-claimed allocation
/// (regression: `init` on the receipt would fail forever).
// Report: ../test-report/clawback_unclaimed_succeeds_despite_buyers_zeroed_receipt.md
#[test]
fn clawback_unclaimed_succeeds_despite_buyers_zeroed_receipt() {
    let (merkle, mut world) = setup();
    let (alice, asset) = mint_alice_position(&merkle, &mut world);

    // Bob is whitelisted but never first-claims. He buys Alice's position
    // and does a subsequent claim on it, creating a zeroed receipt for him.
    let bob = load_whitelist_user(&mut world, &merkle, WHITELISTED_2);
    fund_keypair(&mut world, &bob.keypair, LAMPORTS);

    let transfer_ix =
        world.transfer_asset_ix(&alice.keypair.pubkey(), &bob.keypair.pubkey(), &asset);
    world
        .story
        .run_instruction_as(
            Action::TransferPosition.label(),
            transfer_ix,
            &[&alice.keypair],
        )
        .expect_success();
    world.after_tx();

    world.warp_to(world.linear_checkpoint(50));
    world.subsequent_claim_ok(&bob.keypair, asset);

    world.warp_past_grace();
    let creator_before = world.creator_token_balance();
    world.clawback_unclaimed_ok(bob.keypair.pubkey(), bob.allocation, bob.proofs.clone());

    assert_eq!(
        world.creator_token_balance(),
        creator_before + bob.allocation
    );
}

/// Cannot clawback_unclaimed before end + grace_period.
// Report: ../test-report/clawback_unclaimed_before_grace_fails.md
#[test]
fn clawback_unclaimed_before_grace_fails() {
    let (merkle, mut world) = setup();
    let bob = load_whitelist_user(&mut world, &merkle, WHITELISTED_2);

    world.warp_past_end();
    world.clawback_unclaimed_err(
        bob.keypair.pubkey(),
        bob.allocation,
        bob.proofs.clone(),
        "GracePeriodNotOver",
    );
}

// --- close_campaign ---------------------------------------------------------

/// Vault must be empty before closing.
// Report: ../test-report/close_campaign_fails_when_vault_not_empty.md
#[test]
fn close_campaign_fails_when_vault_not_empty() {
    let (_merkle, mut world) = setup();
    assert!(world.vault_balance() > 0);
    world.close_campaign_err("VaultNotEmpty");
}

/// Empty vault → Campaign PDA and ATA closed, rent returned to creator.
// Report: ../test-report/close_campaign_succeeds_when_vault_empty.md
#[test]
fn close_campaign_succeeds_when_vault_empty() {
    // Fund the campaign with exactly alice's allocation so a full claim
    // drains the vault to zero.
    let merkle = default_merkle();
    let allocation = whitelist_allocation(&merkle, WHITELISTED_1);
    let mut world = VestingWorld::initialized(
        &merkle,
        CampaignConfig {
            total_deposit: allocation,
            ..Default::default()
        },
    );
    let alice = load_whitelist_user(&mut world, &merkle, WHITELISTED_1);

    fund_keypair(&mut world, &alice.keypair, LAMPORTS);
    let asset = world.asset_for(&alice.keypair.pubkey());
    world.first_claim_ok(&alice.keypair, alice.proofs.clone(), alice.allocation);

    world.warp_past_end();
    world.subsequent_claim_ok(&alice.keypair, asset);
    assert_eq!(world.vault_balance(), 0);

    let creator = world.creator.pubkey();
    let campaign = world.campaign_address();
    let campaign_ata = world.campaign_ata();
    let creator_lamports_before = world.lamports(&creator);
    world.close_campaign_ok();

    assert!(
        world.lamports(&campaign) == 0,
        "campaign PDA must be closed"
    );
    assert!(
        world.lamports(&campaign_ata) == 0,
        "campaign ATA must be closed"
    );
    assert!(
        world.lamports(&creator) > creator_lamports_before,
        "creator should receive rent refunds"
    );
}

// --- cancel_campaign --------------------------------------------------------

/// Mistake safeguard: full deposit returned, campaign + ATA + collection closed.
// Report: ../test-report/cancel_campaign_returns_deposit_and_closes_accounts.md
#[test]
fn cancel_campaign_returns_deposit_and_closes_accounts() {
    let (_merkle, mut world) = setup();
    let deposit = world.vault_balance();
    assert!(deposit > 0);

    let creator = world.creator.pubkey();
    let campaign = world.campaign_address();
    let campaign_ata = world.campaign_ata();
    let collection = world.collection;
    let creator_tokens_before = world.creator_token_balance();
    let creator_lamports_before = world.lamports(&creator);

    world.cancel_campaign_ok();

    assert_eq!(
        world.creator_token_balance(),
        creator_tokens_before + deposit,
        "full deposit must return to creator"
    );
    assert_eq!(world.lamports(&campaign), 0, "campaign PDA must be closed");
    assert_eq!(
        world.lamports(&campaign_ata),
        0,
        "campaign ATA must be closed"
    );
    // mpl-core burn leaves a 1-byte uninitialized marker, not a closed account.
    let collection_data = world
        .story
        .svm
        .get_account(&collection)
        .map(|a| a.data)
        .unwrap_or_default();
    assert!(
        collection_data.len() <= 1,
        "collection must be burned (got {} bytes)",
        collection_data.len()
    );
    assert!(
        world.lamports(&creator) > creator_lamports_before,
        "creator should receive rent refunds"
    );
}

/// Once any position was minted, cancel is no longer possible.
// Report: ../test-report/cancel_campaign_fails_after_first_claim.md
#[test]
fn cancel_campaign_fails_after_first_claim() {
    let (merkle, mut world) = setup();
    let (_alice, _asset) = mint_alice_position(&merkle, &mut world);
    world.cancel_campaign_err("CampaignHasPositions");
}

/// Cancel remains blocked even after the minted position was burned
/// (num_minted, not current_size, is the guard).
// Report: ../test-report/cancel_campaign_fails_after_position_burned.md
#[test]
fn cancel_campaign_fails_after_position_burned() {
    let (merkle, mut world) = setup();
    let (_alice, asset) = mint_alice_position(&merkle, &mut world);

    world.exclude_asset(asset);
    world.cancel_campaign_err("CampaignHasPositions");
}

/// Only the campaign creator can cancel.
// Report: ../test-report/cancel_campaign_requires_creator.md
#[test]
fn cancel_campaign_requires_creator() {
    let (merkle, mut world) = setup();
    let alice = load_whitelist_user(&mut world, &merkle, WHITELISTED_1);
    fund_keypair(&mut world, &alice.keypair, LAMPORTS);
    world.cancel_campaign_by(&alice.keypair, "Unauthorized");
}

// --- close_receipt ----------------------------------------------------------

/// Receipts must persist while the campaign exists.
// Report: ../test-report/close_receipt_fails_while_campaign_active.md
#[test]
fn close_receipt_fails_while_campaign_active() {
    let (merkle, mut world) = setup();
    let (alice, _asset) = mint_alice_position(&merkle, &mut world);
    world.close_receipt_err(&alice.keypair, "CampaignStillActive");
}

/// Once the campaign PDA is closed, claimers reclaim their receipt rent.
// Report: ../test-report/close_receipt_returns_rent_after_close_campaign.md
#[test]
fn close_receipt_returns_rent_after_close_campaign() {
    // Exact-funded campaign so a full claim empties the vault.
    let merkle = default_merkle();
    let allocation = whitelist_allocation(&merkle, WHITELISTED_1);
    let mut world = VestingWorld::initialized(
        &merkle,
        CampaignConfig {
            total_deposit: allocation,
            ..Default::default()
        },
    );
    let alice = load_whitelist_user(&mut world, &merkle, WHITELISTED_1);

    fund_keypair(&mut world, &alice.keypair, LAMPORTS);
    let asset = world.asset_for(&alice.keypair.pubkey());
    world.first_claim_ok(&alice.keypair, alice.proofs.clone(), alice.allocation);

    world.warp_past_end();
    world.subsequent_claim_ok(&alice.keypair, asset);

    world.close_campaign_ok();

    let receipt = world.receipt_address(&alice.keypair.pubkey());
    let alice_lamports_before = world.lamports(&alice.keypair.pubkey());
    assert!(world.lamports(&receipt) > 0);

    world.close_receipt_ok(&alice.keypair);

    assert_eq!(world.lamports(&receipt), 0, "receipt must be closed");
    assert!(
        world.lamports(&alice.keypair.pubkey()) > alice_lamports_before,
        "claimer should receive the receipt rent"
    );
}
