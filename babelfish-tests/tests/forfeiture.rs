//! A narrative demonstration, not a defect.
//!
//! The program forfeits a recipient's *vested but unclaimed* tokens to the
//! creator once the grace window past `end` lapses. This test reproduces that
//! exactly and passes: the behaviour is the design, working as written.
//! `VestingWorld`'s `Reporter` derive captures every recorded transaction
//! automatically; no inline rendering needed here.

use frood_idl::types::Value;
use vesting_babelfish_tests::common::{fund_keypair, load_whitelist_user, LAMPORTS, WHITELISTED_1};
use vesting_babelfish_tests::merkle::default_merkle;
use vesting_babelfish_tests::pda::PROGRAM_ID;
use vesting_babelfish_tests::world::{CampaignConfig, VestingWorld};

// Report: ../test-report/vested_tokens_are_forfeited_past_the_grace_window.md
#[test]
fn vested_tokens_are_forfeited_past_the_grace_window() {
    let merkle = default_merkle();
    let mut world = VestingWorld::initialized(&merkle, CampaignConfig::default());
    let alice = load_whitelist_user(&merkle, WHITELISTED_1);
    fund_keypair(&mut world, &alice.keypair, LAMPORTS);

    // Aliases so the rendered diagrams read as narrative, not base58.
    world.story.alias(alice.keypair.pubkey(), "Alice");
    world.story.alias(world.campaign_address(), "Campaign");
    world.story.alias(PROGRAM_ID, "Vesting");
    world.story.narrate(
        "A recipient's vested-but-unclaimed tokens are swept to the creator once the grace \
         window past `end` lapses: this is the design, not a defect.",
    );
    world
        .story
        .given("Alice is whitelisted in a live campaign with a grace window past end");

    // --- Alice claims her cliff, then walks away -------------------------------
    world.warp_to(world.cliff_end());
    let asset = world.asset_for(&alice.keypair.pubkey());
    world.story.alias(asset, "Alice's position NFT");
    world.first_claim_ok(&alice.keypair, alice.proofs.clone(), alice.allocation);

    let cliff = world.claimer_token_balance(&alice.keypair.pubkey());
    let expected_cliff = world.expected_claimable(world.cliff_end(), alice.allocation, 0);
    world.story.then_holds(
        "the cliff unlock reached Alice",
        cliff == expected_cliff && cliff > 0,
    );

    // --- Campaign end: Alice is 100% vested -----------------------------------
    world.warp_past_end();
    let owed = world.expected_claimable(world.end(), alice.allocation, cliff);
    world.story.then_holds(
        "at end, Alice's vested-but-unclaimed remainder is the whole rest of her allocation",
        alice.allocation - cliff == owed,
    );

    // --- The grace window lapses; the creator claws back ----------------------
    world.warp_past_grace();
    let creator_before = world.creator_token_balance();
    world.clawback(asset);
    let recovered = world.creator_token_balance() - creator_before;
    world.story.then_holds(
        "the grace window let the creator reclaim Alice's fully-vested remainder",
        recovered == owed,
    );
    world.story.transition(
        "Alice's remainder",
        Value::U64(owed),
        Value::U64(0),
        "forfeited, never delivered",
    );

    // Alice's own claim is now refused outright.
    world.subsequent_claim_err(&alice.keypair, asset, "ClaimWindowClosed");

    // --- The forfeiture, as a state fact --------------------------------------
    let alice_final = world.claimer_token_balance(&alice.keypair.pubkey());
    world.story.then_holds(
        "Alice's balance never moved across the clawback: her remainder was forfeited, not delivered",
        alice_final == cliff,
    );
}
