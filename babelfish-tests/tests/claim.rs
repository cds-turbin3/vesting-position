//! Claim integration tests, on the source-free frood harness.
//!
//! Definitions:
//!   nft_position / asset  mpl-core position NFT (PDA `[asset, campaign, user]`, minted on first claim)
//!   vested_tokens         the program's per-claim schedule output
//!
//! First claim:  signer = [user], proofs = Some(..), allocation = Some(..)
//! Subsequent:   signer = [user], proofs = None,     allocation = None
//!
//! The world owns the verbs (`first_claim_ok`/`subsequent_claim_ok`/
//! `subsequent_claim_err`), so each scenario reads as the sequence of claims it
//! actually is.

use vesting_babelfish_tests::common::{
    fund_keypair, load_keypair, load_whitelist_user, LAMPORTS, MOCK_ALLOC, NOT_WHITELISTED,
    WHITELISTED_1, WHITELISTED_2,
};
use vesting_babelfish_tests::merkle::{
    default_merkle, leaf_hash, random_proofs, verify, MerkleTree,
};
use vesting_babelfish_tests::world::{Action, CampaignConfig, VestingWorld};

/// Default merkle fixture + an initialized campaign.
#[track_caller]
fn setup() -> (MerkleTree, VestingWorld) {
    let merkle = default_merkle();
    let world = VestingWorld::initialized(&merkle, CampaignConfig::default());
    (merkle, world)
}

// ---------------------------------------------------------------------------
// Happy path
// ---------------------------------------------------------------------------

/// Scenario 1: Alice mints her position, then claims vested tokens again.
// Report: ../test-report/scenario_1_alice_first_and_subsequent_claim.md
#[test]
fn scenario_1_alice_first_and_subsequent_claim() {
    let (merkle, mut world) = setup();
    let alice = load_whitelist_user(&merkle, WHITELISTED_1);

    assert!(verify(
        leaf_hash(&alice.keypair.pubkey(), alice.allocation),
        &alice.proofs,
        &merkle.root,
    ));

    fund_keypair(&mut world, &alice.keypair, LAMPORTS);

    let asset = world.asset_for(&alice.keypair.pubkey());
    world.first_claim_ok(&alice.keypair, alice.proofs.clone(), alice.allocation);
    assert!(world.story.exists(&asset));
    world.assert_receipt_claimer(&alice.keypair.pubkey());

    world.subsequent_claim_ok(&alice.keypair, asset);
}

/// Scenario 2: Alice & Bob each mint; Bob claims both after buying Alice's position.
// Report: ../test-report/scenario_2_two_users_transfer_and_bob_claims_both.md
#[test]
fn scenario_2_two_users_transfer_and_bob_claims_both() {
    let (merkle, mut world) = setup();
    let alice = load_whitelist_user(&merkle, WHITELISTED_1);
    let bob = load_whitelist_user(&merkle, WHITELISTED_2);

    fund_keypair(&mut world, &alice.keypair, LAMPORTS);
    fund_keypair(&mut world, &bob.keypair, LAMPORTS);

    let asset_alice = world.asset_for(&alice.keypair.pubkey());
    let asset_bob = world.asset_for(&bob.keypair.pubkey());

    world.first_claim_ok(&alice.keypair, alice.proofs, alice.allocation);
    world.first_claim_ok(&bob.keypair, bob.proofs, bob.allocation);

    let transfer_ix =
        world.transfer_asset_ix(&alice.keypair.pubkey(), &bob.keypair.pubkey(), &asset_alice);
    world
        .story
        .run_instruction_as(
            Action::TransferPosition.label(),
            transfer_ix,
            &[&alice.keypair],
        )
        .expect_success();
    world.after_tx();

    world.subsequent_claim_ok(&bob.keypair, asset_alice);
    world.subsequent_claim_ok(&bob.keypair, asset_bob);
}

/// Scenario 3: Alice → Bob → Alice buyback; Alice claims again on the same NFT.
// Report: ../test-report/scenario_3_buyback_alice_claims_again.md
#[test]
fn scenario_3_buyback_alice_claims_again() {
    let (merkle, mut world) = setup();
    let alice = load_whitelist_user(&merkle, WHITELISTED_1);
    let bob = load_whitelist_user(&merkle, WHITELISTED_2);

    fund_keypair(&mut world, &alice.keypair, LAMPORTS);
    fund_keypair(&mut world, &bob.keypair, LAMPORTS);

    let asset = world.asset_for(&alice.keypair.pubkey());
    world.first_claim_ok(&alice.keypair, alice.proofs, alice.allocation);
    world.subsequent_claim_ok(&alice.keypair, asset);

    let alice_to_bob =
        world.transfer_asset_ix(&alice.keypair.pubkey(), &bob.keypair.pubkey(), &asset);
    world
        .story
        .run_instruction_as(
            Action::TransferPosition.label(),
            alice_to_bob,
            &[&alice.keypair],
        )
        .expect_success();
    world.after_tx();
    world.subsequent_claim_ok(&bob.keypair, asset);

    let bob_to_alice =
        world.transfer_asset_ix(&bob.keypair.pubkey(), &alice.keypair.pubkey(), &asset);
    world
        .story
        .run_instruction_as(
            Action::TransferPosition.label(),
            bob_to_alice,
            &[&bob.keypair],
        )
        .expect_success();
    world.after_tx();
    world.subsequent_claim_ok(&alice.keypair, asset);
}

// ---------------------------------------------------------------------------
// Error path
// ---------------------------------------------------------------------------

/// Scenario 4: Replay first claim with same proofs → AlreadyClaimed.
// Report: ../test-report/scenario_4_replay_first_claim_fails.md
#[test]
fn scenario_4_replay_first_claim_fails() {
    let (merkle, mut world) = setup();
    let alice = load_whitelist_user(&merkle, WHITELISTED_1);
    fund_keypair(&mut world, &alice.keypair, LAMPORTS);

    world.first_claim_ok(&alice.keypair, alice.proofs.clone(), alice.allocation);
    world.first_claim_err(
        &alice.keypair,
        alice.proofs,
        alice.allocation,
        "AlreadyClaimed",
    );
}

/// Scenario 5: Carol (not whitelisted) cannot first-claim.
// Report: ../test-report/scenario_5_unwhitelisted_user_fails.md
#[test]
fn scenario_5_unwhitelisted_user_fails() {
    let (_merkle, mut world) = setup();
    let carol = load_keypair(NOT_WHITELISTED);
    fund_keypair(&mut world, &carol, LAMPORTS);
    world.warp_to(world.start());

    world.first_claim_err(&carol, random_proofs(), MOCK_ALLOC, "InvalidProofs");
}

/// Scenario 6: Alice cannot subsequent-claim on Bob's NFT.
// Report: ../test-report/scenario_6_not_owner_subsequent_claim_fails.md
#[test]
fn scenario_6_not_owner_subsequent_claim_fails() {
    let (merkle, mut world) = setup();
    let alice = load_whitelist_user(&merkle, WHITELISTED_1);
    let bob = load_whitelist_user(&merkle, WHITELISTED_2);

    fund_keypair(&mut world, &alice.keypair, LAMPORTS);
    fund_keypair(&mut world, &bob.keypair, LAMPORTS);

    let asset_bob = world.asset_for(&bob.keypair.pubkey());
    world.first_claim_ok(&bob.keypair, bob.proofs, bob.allocation);

    world.subsequent_claim_err(&alice.keypair, asset_bob, "NotAssetOwner");
}

/// Scenario 7: Fully claimed position → frozen, and further claims fail.
// Report: ../test-report/scenario_7_fully_claimed_position_frozen.md
#[test]
fn scenario_7_fully_claimed_position_frozen() {
    let (merkle, mut world) = setup();
    let alice = load_whitelist_user(&merkle, WHITELISTED_1);
    fund_keypair(&mut world, &alice.keypair, LAMPORTS);

    let asset = world.asset_for(&alice.keypair.pubkey());
    world.first_claim_ok(&alice.keypair, alice.proofs, alice.allocation);
    assert!(
        !world.fetch_permanent_freeze_delegate(&asset).frozen,
        "position must stay transferable until fully claimed"
    );

    world.warp_past_end();
    world.subsequent_claim_ok(&alice.keypair, asset);

    assert_eq!(
        world.claimer_token_balance(&alice.keypair.pubkey()),
        alice.allocation
    );
    assert!(
        world.fetch_permanent_freeze_delegate(&asset).frozen,
        "loyalty badge must be permanently frozen after full claim"
    );

    world.subsequent_claim_err(&alice.keypair, asset, "AlreadyFullyClaimed");
}

/// Scenario 8: Subsequent claim with wrong asset address → InvalidAsset.
// Report: ../test-report/scenario_8_wrong_asset_subsequent_claim_fails.md
#[test]
fn scenario_8_wrong_asset_subsequent_claim_fails() {
    use solana_pubkey::Pubkey;

    let (_merkle, mut world) = setup();
    let bob = load_keypair(NOT_WHITELISTED);
    fund_keypair(&mut world, &bob, LAMPORTS);
    world.warp_to(world.start());

    let ghost_asset = Pubkey::new_unique();
    world.subsequent_claim_err(&bob, ghost_asset, "InvalidAsset");
}

/// Scenario 9: Claim works until the last second of the grace window, then
/// closes with ClaimWindowClosed (clawback takes over from there).
// Report: ../test-report/scenario_9_claim_window_closes_after_grace.md
#[test]
fn scenario_9_claim_window_closes_after_grace() {
    let (merkle, mut world) = setup();
    let alice = load_whitelist_user(&merkle, WHITELISTED_1);
    fund_keypair(&mut world, &alice.keypair, LAMPORTS);

    let asset = world.asset_for(&alice.keypair.pubkey());
    world.first_claim_ok(&alice.keypair, alice.proofs.clone(), alice.allocation);

    // Last moment inside the window → full allocation still claimable.
    world.warp_to(world.end() + world.grace_period() as i64 - 1);
    world.subsequent_claim_ok(&alice.keypair, asset);
    assert_eq!(
        world.claimer_token_balance(&alice.keypair.pubkey()),
        alice.allocation
    );

    // At end + grace_period the window is closed, even for a first claim.
    let bob = load_whitelist_user(&merkle, WHITELISTED_2);
    fund_keypair(&mut world, &bob.keypair, LAMPORTS);
    world.warp_to(world.end() + world.grace_period() as i64);
    world.first_claim_err(
        &bob.keypair,
        bob.proofs.clone(),
        bob.allocation,
        "ClaimWindowClosed",
    );
}

/// Scenario 10: No claims before `start`, even with valid proofs.
// Report: ../test-report/scenario_10_claim_before_start_fails.md
#[test]
fn scenario_10_claim_before_start_fails() {
    let (merkle, mut world) = setup();
    let alice = load_whitelist_user(&merkle, WHITELISTED_1);
    fund_keypair(&mut world, &alice.keypair, LAMPORTS);

    // Clock is still at campaign-setup time (before start).
    world.first_claim_err(
        &alice.keypair,
        alice.proofs.clone(),
        alice.allocation,
        "CampaignNotStarted",
    );
}
