//! Full vesting lifecycle on the source-free world.
//!
//! Definitions:
//!   nft_position / asset  mpl-core position NFT (PDA `[asset, campaign, user]`, minted on first claim)
//!   vested_tokens         the schedule's per-claim output
//!
//! Alice first-claims at cliff_end, then claims again without proofs; positions
//! change hands and the arc tracks who can claim what. The most interesting
//! transaction (Bob claiming via Alice's transferred NFT, with no merkle
//! proofs, never having been whitelisted) is captured automatically by
//! `VestingWorld`'s `Reporter` derive; no inline rendering needed here.

use vesting_babelfish_tests::common::{
    fund_keypair, load_keypair, load_whitelist_user, LAMPORTS, NOT_WHITELISTED, WHITELISTED_1,
    WHITELISTED_2,
};
use vesting_babelfish_tests::merkle::{default_merkle, TOTAL_DEPOSIT};
use vesting_babelfish_tests::pda::PROGRAM_ID;
use vesting_babelfish_tests::world::{CampaignConfig, VestingWorld};

// Report: ../test-report/full_lifecycle.md
#[test]
fn full_lifecycle() {
    let merkle = default_merkle();
    let mut world = VestingWorld::initialized(&merkle, CampaignConfig::default());
    let alice = load_whitelist_user(&merkle, WHITELISTED_1);
    let charlie = load_whitelist_user(&merkle, WHITELISTED_2);
    let bob = load_keypair(NOT_WHITELISTED);

    fund_keypair(&mut world, &alice.keypair, LAMPORTS);
    fund_keypair(&mut world, &charlie.keypair, LAMPORTS);
    fund_keypair(&mut world, &bob, LAMPORTS);

    let creator = world.creator.pubkey();
    world.story.alias(alice.keypair.pubkey(), "Alice");
    world.story.alias(bob.pubkey(), "Bob");
    world.story.alias(charlie.keypair.pubkey(), "Charlie");
    world.story.alias(creator, "Creator");
    world.story.alias(world.campaign_address(), "Campaign");
    world.story.alias(PROGRAM_ID, "Vesting");
    world
        .story
        .given("a live campaign; Alice and Charlie whitelisted, Bob not");

    // --- Campaign initialized: the on-chain state matches the config -----------
    let campaign = world.campaign();
    let config = CampaignConfig::default();
    assert_eq!(campaign.creator, creator, "campaign creator match");
    assert!(
        campaign.start == config.start
            && campaign.end == config.end
            && campaign.cliff_duration == config.cliff_duration
            && campaign.cliff_release_bps == config.cliff_release_bps
            && campaign.grace_period == config.grace_period,
        "campaign schedule match"
    );
    assert!(
        campaign.merkle_root.as_slice() == merkle.root.as_slice()
            && campaign.total_deposit == TOTAL_DEPOSIT,
        "merkle root and deposit match"
    );
    assert_ne!(
        alice.allocation, charlie.allocation,
        "alice and charlie have different allocations"
    );

    // --- Alice claims her vesting tokens at end of cliff period -----------------
    world.warp_to(world.cliff_end());
    let alice_position = world.asset_for(&alice.keypair.pubkey());
    let charlie_position = world.asset_for(&charlie.keypair.pubkey());
    world.story.alias(alice_position, "Alice position NFT");
    world.story.alias(charlie_position, "Charlie position NFT");
    world.story.alias(campaign.collection, "Collection");

    world.first_claim_ok(&alice.keypair, alice.proofs.clone(), alice.allocation);
    let alice_balance_after_cliff = world.claimer_token_balance(&alice.keypair.pubkey());
    let expected_cliff = world.expected_claimable(world.cliff_end(), alice.allocation, 0);
    assert_eq!(
        alice_balance_after_cliff, expected_cliff,
        "Alice's balance after the cliff claim"
    );
    assert!(
        world.story.exists(&alice_position),
        "alice position nft exists"
    );
    assert_eq!(
        world.asset_owner(&alice_position),
        alice.keypair.pubkey(),
        "alice position nft owner is alice"
    );
    world.assert_receipt_claimer(&alice.keypair.pubkey());

    // --- Alice claims for elapsed time (50% of claim window) --------------------
    let mid = world.linear_checkpoint(50);
    world.warp_to(mid);
    let balance_before = world.claimer_token_balance(&alice.keypair.pubkey());
    let expected_mid = world.expected_claimable(mid, alice.allocation, balance_before);
    world.subsequent_claim_ok(&alice.keypair, alice_position);
    assert_eq!(
        world.claimer_token_balance(&alice.keypair.pubkey()),
        balance_before + expected_mid,
        "Alice balance after second claim at 50% elapsed"
    );

    // --- Charlie first claim at 50% elapsed (parallel position) ----------------
    world.first_claim_ok(&charlie.keypair, charlie.proofs.clone(), charlie.allocation);
    let expected_charlie_first = world.expected_claimable(mid, charlie.allocation, 0);
    assert_eq!(
        world.claimer_token_balance(&charlie.keypair.pubkey()),
        expected_charlie_first,
        "Charlie's balance after first claim"
    );
    assert!(
        world.story.exists(&charlie_position),
        "charlie position nft exists"
    );
    assert_eq!(
        world.asset_owner(&charlie_position),
        charlie.keypair.pubkey(),
        "charlie position nft owner is charlie"
    );
    world.assert_receipt_claimer(&charlie.keypair.pubkey());

    // --- Alice transfers her vesting position to Bob ---------------------------
    let owner_before_transfer = world.asset_owner(&alice_position);
    let transfer_ix =
        world.transfer_asset_ix(&alice.keypair.pubkey(), &bob.pubkey(), &alice_position);
    world
        .story
        .run_instruction(transfer_ix, &[&alice.keypair])
        .expect_success();
    world.after_tx();
    assert_ne!(owner_before_transfer, bob.pubkey());
    assert_eq!(
        world.asset_owner(&alice_position),
        bob.pubkey(),
        "Bob is now the owner of Alice's vesting position"
    );

    // --- Alice replays merkle proofs after transferring the NFT ----------------
    // The infinite-money trick: a second first-claim with the same proofs must
    // be rejected (AlreadyClaimed), and the receipt stays bound to Alice.
    world.first_claim_err(
        &alice.keypair,
        alice.proofs.clone(),
        alice.allocation,
        "AlreadyClaimed",
    );
    assert_eq!(
        world.receipt_claimer(&alice.keypair.pubkey()),
        alice.keypair.pubkey(),
        "claim receipt still bound to alice"
    );
    assert_ne!(
        alice.keypair.pubkey(),
        world.asset_owner(&alice_position),
        "alice no longer owns the nft"
    );

    // --- Alice tries to claim on her position after transferring it to Bob -----
    // Former owners cannot claim via their old receipt or whitelist status.
    world.subsequent_claim_err(&alice.keypair, alice_position, "NotAssetOwner");

    // --- Bob claims tokens using Alice's position -------------------------------
    let alice_claimed = world.claimer_token_balance(&alice.keypair.pubkey());
    let later = world.linear_checkpoint(75);
    world.warp_to(later);
    let bob_expected = world.expected_claimable(later, alice.allocation, alice_claimed);
    world
        .story
        .given("Bob, never whitelisted, now holds Alice's position NFT");
    world.subsequent_claim_ok(&bob, alice_position);
    assert_eq!(
        world.claimer_token_balance(&bob.pubkey()),
        bob_expected,
        "Bob claims vested tokens from Alice's allocation because he owns her position"
    );
    assert_eq!(
        world.asset_owner(&alice_position),
        bob.pubkey(),
        "alice position nft owner is still bob"
    );

    // --- Charlie subsequent claim before transferring to Alice -----------------
    let charlie_balance_before_sub = world.claimer_token_balance(&charlie.keypair.pubkey());
    let charlie_sub_expected =
        world.expected_claimable(later, charlie.allocation, charlie_balance_before_sub);
    world.subsequent_claim_ok(&charlie.keypair, charlie_position);
    let charlie_claimed_so_far = world.claimer_token_balance(&charlie.keypair.pubkey());
    assert_eq!(
        charlie_claimed_so_far,
        charlie_balance_before_sub + charlie_sub_expected,
        "Charlie claimed more vested tokens before transferring the NFT"
    );

    // --- Charlie transfers his position to Alice -------------------------------
    let charlie_owner_before = world.asset_owner(&charlie_position);
    let charlie_to_alice_ix = world.transfer_asset_ix(
        &charlie.keypair.pubkey(),
        &alice.keypair.pubkey(),
        &charlie_position,
    );
    world
        .story
        .run_instruction(charlie_to_alice_ix, &[&charlie.keypair])
        .expect_success();
    world.after_tx();
    assert_ne!(charlie_owner_before, alice.keypair.pubkey());
    assert_eq!(
        world.asset_owner(&charlie_position),
        alice.keypair.pubkey(),
        "Alice now holds Charlie's vesting position NFT"
    );
    assert_ne!(
        alice_position, charlie_position,
        "alice and charlie positions are distinct nfts"
    );

    // --- Charlie tries to claim after transferring his position to Alice -------
    world.subsequent_claim_err(&charlie.keypair, charlie_position, "NotAssetOwner");

    // --- Alice claims remaining vesting on Charlie's position ------------------
    // The release uses Charlie's allocation stored on his NFT, not Alice's leaf.
    let ninety = world.linear_checkpoint(90);
    world.warp_to(ninety);
    let alice_balance_before_charlie_claim = world.claimer_token_balance(&alice.keypair.pubkey());
    let alice_on_charlie_expected =
        world.expected_claimable(ninety, charlie.allocation, charlie_claimed_so_far);
    world.subsequent_claim_ok(&alice.keypair, charlie_position);
    let alice_balance_after_charlie_claim = world.claimer_token_balance(&alice.keypair.pubkey());
    let alice_increment = alice_balance_after_charlie_claim - alice_balance_before_charlie_claim;
    assert_eq!(
        alice_increment, alice_on_charlie_expected,
        "increment matches charlie allocation math"
    );
    let alice_own_expected_at_ninety =
        world.expected_claimable(ninety, alice.allocation, alice_claimed);
    assert_ne!(
        alice_increment, alice_own_expected_at_ninety,
        "increment uses charlie allocation not alice's own leaf"
    );
    assert_eq!(
        world.asset_owner(&charlie_position),
        alice.keypair.pubkey(),
        "charlie's position owner is alice"
    );
    assert_eq!(
        world.asset_owner(&alice_position),
        bob.pubkey(),
        "Alice's original position owner is bob"
    );
}
