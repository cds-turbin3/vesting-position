//! Vesting-schedule tests: assert each claim releases exactly what the schedule
//! math predicts, across the cliff and the linear tail.
//!
//! The anchor-litesvm original wove a markdown table report through
//! `claims_at_linear_checkpoints`; that presentation is handled separately by
//! frood's native report vocabulary, so here the checkpoint walk keeps only the
//! behavioral assertions (cumulative == schedule math at each point).

use frood_idl::types::Value;
use vesting_babelfish_tests::common::{fund_keypair, load_whitelist_user, LAMPORTS, WHITELISTED_1};
use vesting_babelfish_tests::merkle::{default_merkle, MerkleTree};
use vesting_babelfish_tests::world::{CampaignConfig, VestingWorld};

/// Percentages through the linear vesting window (0 = at cliff_end, 100 = at end).
const LINEAR_CHECKPOINTS: &[u64] = &[0, 1, 7, 13, 20, 33, 45, 50, 57, 67, 75, 83, 91, 99, 100];

/// Default merkle fixture + an initialized campaign under `config`.
#[track_caller]
fn setup(config: CampaignConfig) -> (MerkleTree, VestingWorld) {
    let merkle = default_merkle();
    let world = VestingWorld::initialized(&merkle, config);
    (merkle, world)
}

/// Before cliff_end nothing is claimable; the position NFT still mints.
// Report: ../test-report/before_cliff_transfers_zero.md
#[test]
fn before_cliff_transfers_zero() {
    let (merkle, mut world) = setup(CampaignConfig::default());
    let alice = load_whitelist_user(&mut world, &merkle, WHITELISTED_1);

    world.warp_to(world.cliff_end() - 1);
    assert_eq!(
        world.expected_claimable(world.cliff_end() - 1, alice.allocation, 0),
        0
    );

    fund_keypair(&mut world, &alice.keypair, LAMPORTS);
    let asset = world.asset_for(&alice.keypair.pubkey());
    world.first_claim_ok(&alice.keypair, alice.proofs.clone(), alice.allocation);
    assert!(world.story.exists(&asset));
    assert_eq!(world.claimer_token_balance(&alice.keypair.pubkey()), 0);
}

/// Default campaign: 10% released at cliff, linear tail after.
// Report: ../test-report/at_cliff_releases_cliff_bps.md
#[test]
fn at_cliff_releases_cliff_bps() {
    let (merkle, mut world) = setup(CampaignConfig::default());
    let alice = load_whitelist_user(&mut world, &merkle, WHITELISTED_1);

    world.warp_to(world.cliff_end());
    let expected = world.expected_claimable(world.cliff_end(), alice.allocation, 0);
    assert_eq!(expected, alice.allocation * 1_000 / 10_000);

    fund_keypair(&mut world, &alice.keypair, LAMPORTS);
    world.first_claim_ok(&alice.keypair, alice.proofs.clone(), alice.allocation);
    assert_eq!(
        world.claimer_token_balance(&alice.keypair.pubkey()),
        expected
    );
}

/// Halfway through the linear window: cliff slice + 50% of the linear remainder.
// Report: ../test-report/mid_schedule_cliff_plus_linear.md
#[test]
fn mid_schedule_cliff_plus_linear() {
    let (merkle, mut world) = setup(CampaignConfig::default());
    let alice = load_whitelist_user(&mut world, &merkle, WHITELISTED_1);
    world.story.narrate(
        "Halfway through the linear window, a claim releases the cliff slice plus half the \
         linear remainder: cliff_amount + linear_amount/2.",
    );

    let mid = world.linear_checkpoint(50);
    world.warp_to(mid);

    let expected = world.expected_claimable(mid, alice.allocation, 0);
    let cliff_amount = alice.allocation * world.cliff_release_bps() as u64 / 10_000;
    let linear_amount = alice.allocation - cliff_amount;
    assert_eq!(expected, cliff_amount + linear_amount / 2);

    fund_keypair(&mut world, &alice.keypair, LAMPORTS);
    let before = world.claimer_token_balance(&alice.keypair.pubkey());
    world.first_claim_ok(&alice.keypair, alice.proofs.clone(), alice.allocation);
    let after = world.claimer_token_balance(&alice.keypair.pubkey());
    assert_eq!(after, world.expected_claimable(mid, alice.allocation, 0));
    world.story.transition(
        "Alice's balance",
        Value::U64(before),
        Value::U64(after),
        "mid-schedule claim: cliff slice + half the linear remainder",
    );
}

/// After end, the full allocation is claimable in one shot.
// Report: ../test-report/at_end_releases_full_allocation.md
#[test]
fn at_end_releases_full_allocation() {
    let (merkle, mut world) = setup(CampaignConfig::default());
    let alice = load_whitelist_user(&mut world, &merkle, WHITELISTED_1);

    world.warp_past_end();
    assert_eq!(
        world.expected_claimable(world.end() + 1, alice.allocation, 0),
        alice.allocation
    );

    fund_keypair(&mut world, &alice.keypair, LAMPORTS);
    world.first_claim_ok(&alice.keypair, alice.proofs.clone(), alice.allocation);
    assert_eq!(
        world.claimer_token_balance(&alice.keypair.pubkey()),
        alice.allocation
    );
}

/// cliff_release_bps = 0 → pure linear from cliff_end to end.
// Report: ../test-report/pure_linear_starts_at_cliff_end.md
#[test]
fn pure_linear_starts_at_cliff_end() {
    let (merkle, mut world) = setup(CampaignConfig {
        cliff_release_bps: 0,
        ..Default::default()
    });
    let alice = load_whitelist_user(&mut world, &merkle, WHITELISTED_1);

    world.warp_to(world.cliff_end());
    assert_eq!(
        world.expected_claimable(world.cliff_end(), alice.allocation, 0),
        0
    );

    fund_keypair(&mut world, &alice.keypair, LAMPORTS);
    let asset = world.asset_for(&alice.keypair.pubkey());
    world.first_claim_ok(&alice.keypair, alice.proofs.clone(), alice.allocation);
    assert_eq!(world.claimer_token_balance(&alice.keypair.pubkey()), 0);

    world.warp_past_end();
    world.subsequent_claim_ok(&alice.keypair, asset);
    assert_eq!(
        world.claimer_token_balance(&alice.keypair.pubkey()),
        alice.allocation
    );
}

/// cliff_release_bps = 10000 → 100% at cliff, no linear tail.
// Report: ../test-report/full_release_at_cliff.md
#[test]
fn full_release_at_cliff() {
    let (merkle, mut world) = setup(CampaignConfig {
        cliff_release_bps: 10_000,
        ..Default::default()
    });
    let alice = load_whitelist_user(&mut world, &merkle, WHITELISTED_1);

    world.warp_to(world.cliff_end());
    assert_eq!(
        world.expected_claimable(world.cliff_end(), alice.allocation, 0),
        alice.allocation
    );

    fund_keypair(&mut world, &alice.keypair, LAMPORTS);
    let asset = world.asset_for(&alice.keypair.pubkey());
    world.first_claim_ok(&alice.keypair, alice.proofs.clone(), alice.allocation);
    assert_eq!(
        world.claimer_token_balance(&alice.keypair.pubkey()),
        alice.allocation
    );
    assert!(
        world.fetch_permanent_freeze_delegate(&asset).frozen,
        "loyalty badge must be permanently frozen after full claim"
    );

    world.warp_past_end();
    world.subsequent_claim_err(&alice.keypair, asset, "AlreadyFullyClaimed");
}

/// Incremental claims at several points through the linear window.
// Report: ../test-report/claims_at_linear_checkpoints.md
#[test]
fn claims_at_linear_checkpoints() {
    let (merkle, mut world) = setup(CampaignConfig::default());
    let alice = load_whitelist_user(&mut world, &merkle, WHITELISTED_1);

    fund_keypair(&mut world, &alice.keypair, LAMPORTS);
    let asset = world.asset_for(&alice.keypair.pubkey());

    // Show vesting happening with no transaction: the schedule ceiling is a
    // pure function of the clock, so sampling it across a plain warp (no
    // claim in between) proves the report can show the schedule moving on
    // its own, not only at claim time.
    let claimable = world.observe_claimable("Alice claimable (schedule ceiling)", alice.allocation);
    world.story.sample(claimable); // before the cliff: nothing vested yet
    world.warp_to(world.cliff_end());
    world.story.sample(claimable); // the cliff unlocked — no transaction ran

    // "Claimed only ever grows": Alice's ATA balance is exactly what she has
    // claimed, so its own observation held `monotonic` is the law.
    let alice_balance = world.observe_balance(&alice.keypair);
    world.story.monotonic(alice_balance);

    for (i, &pct) in LINEAR_CHECKPOINTS.iter().enumerate() {
        let now = world.linear_checkpoint(pct);
        world.warp_to(now);

        if i == 0 {
            world.first_claim_ok(&alice.keypair, alice.proofs.clone(), alice.allocation);
        } else {
            world.subsequent_claim_ok(&alice.keypair, asset);
        }

        let cumulative = world.claimer_token_balance(&alice.keypair.pubkey());
        let expected = world.expected_claimable(now, alice.allocation, 0);
        assert_eq!(
            cumulative, expected,
            "cumulative at {pct}% linear must match the schedule math"
        );
    }

    assert_eq!(
        world.claimer_token_balance(&alice.keypair.pubkey()),
        alice.allocation,
        "full allocation released at end"
    );
}
