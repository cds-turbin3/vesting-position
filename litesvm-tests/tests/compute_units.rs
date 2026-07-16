//! Compute-unit profile: each path stays under its cap. The first claim mints
//! the position NFT (mpl-core CreateV2 CPI) and, on the full-allocation path,
//! also freezes it; that path needs FIRST_CLAIM_CU, everything else fits the
//! 200k default.

use anchor_litesvm::Signer;
use vesting_litesvm_tests::campaign::{
    claim_args, log_tx_cu, CampaignConfig, TestCampaign, DEFAULT_TX_CU, FIRST_CLAIM_CU,
};
use vesting_litesvm_tests::common::{fund_keypair, load_whitelist_user, LAMPORTS, WHITELISTED_1};
use vesting_litesvm_tests::merkle::default_merkle;

#[test]
fn compute_units_profile() {
    let merkle = default_merkle();
    let config = CampaignConfig::default();
    let alice = load_whitelist_user(&merkle, WHITELISTED_1);

    // World A: profile initialize, the mint-only first claim, and a subsequent
    // claim, all under the default cap (no ComputeBudget ix).
    let mut world = TestCampaign::uninitialized(&merkle, config);
    let init = world.run_initialize(&merkle);
    log_tx_cu("initialize", init.compute_units(), DEFAULT_TX_CU);

    fund_keypair(&mut world.ctx, &alice.keypair, LAMPORTS);

    // First claim at start: mints the NFT, releases zero tokens (before cliff).
    world.warp_to(world.start());
    let mint_bundle = world.claim_bundle(&alice.keypair.pubkey(), None);
    let mint_ix = world.ctx.program().build_ix(
        mint_bundle,
        claim_args(Some(alice.proofs.clone()), Some(alice.allocation)),
    );
    let first_mint = world
        .ctx
        .execute_instructions(vec![mint_ix], &[&alice.keypair])
        .expect("first claim (mint only)")
        .assert_success();
    log_tx_cu(
        "first_claim (mint only)",
        first_mint.compute_units(),
        DEFAULT_TX_CU,
    );
    world.after_tx();

    // Subsequent claim on the same position (still before cliff, zero released).
    let sub_bundle = world.claim_bundle(&alice.keypair.pubkey(), None);
    let sub_ix = world
        .ctx
        .program()
        .build_ix(sub_bundle, claim_args(None, None));
    let sub = world
        .ctx
        .execute_instructions(vec![sub_ix], &[&alice.keypair])
        .expect("subsequent claim")
        .assert_success();
    log_tx_cu("subsequent_claim", sub.compute_units(), DEFAULT_TX_CU);

    // World B: the heaviest path (first claim at end+1 releases the full
    // allocation and freezes the badge), which needs the raised limit.
    let mut heavy = TestCampaign::uninitialized(&merkle, config);
    heavy.run_initialize(&merkle);
    fund_keypair(&mut heavy.ctx, &alice.keypair, LAMPORTS);
    heavy.warp_to(heavy.end() + 1);
    let first_full = heavy.first_claim_ok(&alice.keypair, alice.proofs, alice.allocation);
    log_tx_cu(
        "first_claim (full allocation)",
        first_full.compute_units(),
        FIRST_CLAIM_CU,
    );

    assert!(
        first_full.compute_units() <= FIRST_CLAIM_CU as u64,
        "first claim (full allocation) exceeds FIRST_CLAIM_CU"
    );
    assert!(
        init.compute_units() <= DEFAULT_TX_CU as u64,
        "initialize exceeds default cap"
    );
    assert!(
        first_mint.compute_units() <= DEFAULT_TX_CU as u64,
        "first claim (mint only) exceeds default cap"
    );
    assert!(
        sub.compute_units() <= DEFAULT_TX_CU as u64,
        "subsequent claim exceeds default cap"
    );
}
