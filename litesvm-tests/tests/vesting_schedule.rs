//! Vesting-schedule tests: assert each claim releases exactly what the schedule
//! math predicts, across the cliff and the linear tail.

use anchor_litesvm::{AssertionHelpers, MarkdownBlock, Report, Signer};
use vesting_litesvm_tests::campaign::{CampaignConfig, TestCampaign};
use vesting_litesvm_tests::common::{fund_keypair, load_whitelist_user, LAMPORTS, WHITELISTED_1};
use vesting_litesvm_tests::format::format_tokens;
use vesting_litesvm_tests::merkle::{default_merkle, MerkleTree};

/// Percentages through the linear vesting window (0 = at cliff_end, 100 = at end).
const LINEAR_CHECKPOINTS: &[u64] = &[0, 1, 7, 13, 20, 33, 45, 50, 57, 67, 75, 83, 91, 99, 100];

/// Default merkle fixture + an initialized campaign under `config`.
fn setup(config: CampaignConfig) -> (MerkleTree, TestCampaign) {
    let merkle = default_merkle();
    let world = TestCampaign::initialized(&merkle, config);
    (merkle, world)
}

/// Before cliff_end nothing is claimable; the position NFT still mints.
#[test]
fn before_cliff_transfers_zero() {
    let (merkle, mut world) = setup(CampaignConfig::default());
    let alice = load_whitelist_user(&merkle, WHITELISTED_1);

    world.warp_to(world.cliff_end() - 1);
    assert_eq!(
        world.expected_claimable(world.cliff_end() - 1, alice.allocation, 0),
        0
    );

    fund_keypair(&mut world.ctx, &alice.keypair, LAMPORTS);
    let asset = world.asset_for(&alice.keypair.pubkey());
    world.first_claim_ok(&alice.keypair, alice.proofs.clone(), alice.allocation);
    world.ctx.svm.assert_account_exists(&asset);
    assert_eq!(world.claimer_token_balance(&alice.keypair.pubkey()), 0);
}

/// Default campaign: 10% released at cliff, linear tail after.
#[test]
fn at_cliff_releases_cliff_bps() {
    let (merkle, mut world) = setup(CampaignConfig::default());
    let alice = load_whitelist_user(&merkle, WHITELISTED_1);

    world.warp_to(world.cliff_end());
    let expected = world.expected_claimable(world.cliff_end(), alice.allocation, 0);
    assert_eq!(expected, alice.allocation * 1_000 / 10_000);

    fund_keypair(&mut world.ctx, &alice.keypair, LAMPORTS);
    world.first_claim_ok(&alice.keypair, alice.proofs.clone(), alice.allocation);
    assert_eq!(
        world.claimer_token_balance(&alice.keypair.pubkey()),
        expected
    );
}

/// Halfway through the linear window: cliff slice + 50% of the linear remainder.
#[test]
fn mid_schedule_cliff_plus_linear() {
    let (merkle, mut world) = setup(CampaignConfig::default());
    let alice = load_whitelist_user(&merkle, WHITELISTED_1);

    let mid = world.linear_checkpoint(50);
    world.warp_to(mid);

    let expected = world.expected_claimable(mid, alice.allocation, 0);
    let cliff_amount = alice.allocation * world.cliff_release_bps() as u64 / 10_000;
    let linear_amount = alice.allocation - cliff_amount;
    assert_eq!(expected, cliff_amount + linear_amount / 2);

    fund_keypair(&mut world.ctx, &alice.keypair, LAMPORTS);
    world.first_claim_ok(&alice.keypair, alice.proofs.clone(), alice.allocation);
    assert_eq!(
        world.claimer_token_balance(&alice.keypair.pubkey()),
        world.expected_claimable(mid, alice.allocation, 0)
    );
}

/// After end, the full allocation is claimable in one shot.
#[test]
fn at_end_releases_full_allocation() {
    let (merkle, mut world) = setup(CampaignConfig::default());
    let alice = load_whitelist_user(&merkle, WHITELISTED_1);

    world.warp_past_end();
    assert_eq!(
        world.expected_claimable(world.end() + 1, alice.allocation, 0),
        alice.allocation
    );

    fund_keypair(&mut world.ctx, &alice.keypair, LAMPORTS);
    world.first_claim_ok(&alice.keypair, alice.proofs.clone(), alice.allocation);
    assert_eq!(
        world.claimer_token_balance(&alice.keypair.pubkey()),
        alice.allocation
    );
}

/// cliff_release_bps = 0 → pure linear from cliff_end to end.
#[test]
fn pure_linear_starts_at_cliff_end() {
    let (merkle, mut world) = setup(CampaignConfig {
        cliff_release_bps: 0,
        ..Default::default()
    });
    let alice = load_whitelist_user(&merkle, WHITELISTED_1);

    world.warp_to(world.cliff_end());
    assert_eq!(
        world.expected_claimable(world.cliff_end(), alice.allocation, 0),
        0
    );

    fund_keypair(&mut world.ctx, &alice.keypair, LAMPORTS);
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
#[test]
fn full_release_at_cliff() {
    let (merkle, mut world) = setup(CampaignConfig {
        cliff_release_bps: 10_000,
        ..Default::default()
    });
    let alice = load_whitelist_user(&merkle, WHITELISTED_1);

    world.warp_to(world.cliff_end());
    assert_eq!(
        world.expected_claimable(world.cliff_end(), alice.allocation, 0),
        alice.allocation
    );

    fund_keypair(&mut world.ctx, &alice.keypair, LAMPORTS);
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
#[test]
fn claims_at_linear_checkpoints() {
    let (merkle, mut world) = setup(CampaignConfig::default());
    let alice = load_whitelist_user(&merkle, WHITELISTED_1);
    let mut md = Report::new(
        "Linear checkpoint vesting",
        "incremental claims at each point through the linear window; \
         10% cliff at cliff_end, remainder vests linearly to end",
    );

    fund_keypair(&mut world.ctx, &alice.keypair, LAMPORTS);
    world.ctx.alias(alice.keypair.pubkey(), "Alice");
    let asset = world.asset_for(&alice.keypair.pubkey());

    md.step("claim at each linear checkpoint");
    let mut table_rows: Vec<Vec<String>> = Vec::new();

    for (i, &pct) in LINEAR_CHECKPOINTS.iter().enumerate() {
        let now = world.linear_checkpoint(pct);
        world.warp_to(now);
        let balance_before = world.claimer_token_balance(&alice.keypair.pubkey());

        if i == 0 {
            world.first_claim_ok(&alice.keypair, alice.proofs.clone(), alice.allocation);
        } else {
            world.subsequent_claim_ok(&alice.keypair, asset);
        }

        let cumulative = world.claimer_token_balance(&alice.keypair.pubkey());
        let incremental = cumulative - balance_before;
        let expected = world.expected_claimable(now, alice.allocation, 0);
        md.check(
            format!("cumulative at {pct}% linear"),
            format_tokens(expected),
            format_tokens(cumulative),
        );
        table_rows.push(vec![
            pct.to_string(),
            now.to_string(),
            format_tokens(incremental),
            format_tokens(cumulative),
            format_tokens(expected),
        ]);
    }

    md.block(
        "vested / released at linear checkpoints",
        MarkdownBlock::Table {
            headers: vec![
                "linear %".into(),
                "timestamp (unix)".into(),
                "incremental release".into(),
                "cumulative released".into(),
                "expected cumulative".into(),
            ],
            rows: table_rows,
        },
    );
    md.check(
        "full allocation released at end",
        format_tokens(alice.allocation),
        format_tokens(world.claimer_token_balance(&alice.keypair.pubkey())),
    );
}
