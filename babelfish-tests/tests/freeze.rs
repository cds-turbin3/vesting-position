//! Freeze / transfer-gating tests on the source-free world: collection freeze,
//! per-asset freeze, the loyalty-badge freeze on full claim, and exclude_asset.

use solana_pubkey::Pubkey;
use vesting_babelfish_tests::common::{
    fund_keypair, load_whitelist_user, WhitelistUser, LAMPORTS, WHITELISTED_1, WHITELISTED_2,
};
use vesting_babelfish_tests::merkle::{default_merkle, MerkleTree};
use vesting_babelfish_tests::world::{CampaignConfig, VestingWorld};

#[track_caller]
fn setup(config: CampaignConfig) -> (MerkleTree, VestingWorld) {
    let merkle = default_merkle();
    let world = VestingWorld::initialized(&merkle, config);
    (merkle, world)
}

fn mint_alice_position(merkle: &MerkleTree, world: &mut VestingWorld) -> (WhitelistUser, Pubkey) {
    let alice = load_whitelist_user(merkle, WHITELISTED_1);
    fund_keypair(world, &alice.keypair, LAMPORTS);
    let asset = world.asset_for(&alice.keypair.pubkey());
    world.first_claim_ok(&alice.keypair, alice.proofs.clone(), alice.allocation);
    (alice, asset)
}

/// Transferable campaign: asset minted unfrozen → transfer succeeds.
#[test]
fn transfer_when_collection_unfrozen_on_transferable_campaign() {
    let (merkle, mut world) = setup(CampaignConfig::default());
    let bob = load_whitelist_user(&merkle, WHITELISTED_2);
    fund_keypair(&mut world, &bob.keypair, LAMPORTS);

    let (alice, asset) = mint_alice_position(&merkle, &mut world);

    let collection = world.collection;
    assert!(!world.fetch_permanent_freeze_delegate(&collection).frozen);
    assert!(
        !world
            .try_fetch_asset_freeze_delegate(&asset)
            .expect("transferable mint has PermanentFreezeDelegate")
            .frozen
    );

    assert!(world.transfer_changes_owner(&alice.keypair, &bob.keypair.pubkey(), &asset));
    assert_eq!(world.asset_owner(&asset), bob.keypair.pubkey());
}

/// Non-transferable campaign: collection frozen, no asset freeze plugin → transfer blocked.
#[test]
fn transfer_blocked_when_collection_frozen() {
    let (merkle, mut world) = setup(CampaignConfig {
        is_transferable: false,
        ..Default::default()
    });
    let bob = load_whitelist_user(&merkle, WHITELISTED_2);
    fund_keypair(&mut world, &bob.keypair, LAMPORTS);

    let (alice, asset) = mint_alice_position(&merkle, &mut world);

    let collection = world.collection;
    assert!(world.fetch_permanent_freeze_delegate(&collection).frozen);
    assert!(!world.asset_has_freeze_delegate(&asset));

    assert!(!world.transfer_changes_owner(&alice.keypair, &bob.keypair.pubkey(), &asset));
    assert_eq!(world.asset_owner(&asset), alice.keypair.pubkey());
}

/// MPL Core precedence: asset `frozen: false` overrides collection freeze on existing positions.
#[test]
fn collection_freeze_does_not_block_existing_transferable_positions() {
    let (merkle, mut world) = setup(CampaignConfig::default());
    let bob = load_whitelist_user(&merkle, WHITELISTED_2);
    fund_keypair(&mut world, &bob.keypair, LAMPORTS);

    let (alice, asset) = mint_alice_position(&merkle, &mut world);

    world.freeze_collection(true);

    let collection = world.collection;
    assert!(world.fetch_permanent_freeze_delegate(&collection).frozen);
    assert!(
        !world
            .try_fetch_asset_freeze_delegate(&asset)
            .expect("transferable mint has PermanentFreezeDelegate")
            .frozen
    );

    assert!(world.transfer_changes_owner(&alice.keypair, &bob.keypair.pubkey(), &asset));
}

/// Unfreezing a non-transferable collection restores transfer (no asset plugin to clear).
#[test]
fn unfreeze_collection_restores_transfer_for_non_transferable_campaign() {
    let (merkle, mut world) = setup(CampaignConfig {
        is_transferable: false,
        ..Default::default()
    });
    let bob = load_whitelist_user(&merkle, WHITELISTED_2);
    fund_keypair(&mut world, &bob.keypair, LAMPORTS);

    let (alice, asset) = mint_alice_position(&merkle, &mut world);

    assert!(!world.transfer_changes_owner(&alice.keypair, &bob.keypair.pubkey(), &asset));

    world.freeze_collection(false);

    assert!(world.campaign().is_transferable);
    let collection = world.collection;
    assert!(!world.fetch_permanent_freeze_delegate(&collection).frozen);
    assert!(!world.asset_has_freeze_delegate(&asset));

    assert!(world.transfer_changes_owner(&alice.keypair, &bob.keypair.pubkey(), &asset));
    assert_eq!(world.asset_owner(&asset), bob.keypair.pubkey());
}

/// Fully claimed position on transferable campaign is frozen (loyalty badge).
#[test]
fn fully_claimed_loyalty_badge_is_permanently_frozen() {
    let (merkle, mut world) = setup(CampaignConfig::default());
    let bob = load_whitelist_user(&merkle, WHITELISTED_2);
    fund_keypair(&mut world, &bob.keypair, LAMPORTS);

    let (alice, asset) = mint_alice_position(&merkle, &mut world);

    world.warp_past_end();
    world.subsequent_claim_ok(&alice.keypair, asset);

    assert_eq!(
        world.claimer_token_balance(&alice.keypair.pubkey()),
        alice.allocation
    );
    let collection = world.collection;
    assert!(!world.fetch_permanent_freeze_delegate(&collection).frozen);
    assert!(world.fetch_permanent_freeze_delegate(&asset).frozen);

    assert!(!world.transfer_changes_owner(&alice.keypair, &bob.keypair.pubkey(), &asset));
}

/// Fully claimed position on non-transferable campaign has no asset freeze plugin.
#[test]
fn fully_claimed_non_transferable_has_no_asset_freeze() {
    let (merkle, mut world) = setup(CampaignConfig {
        is_transferable: false,
        ..Default::default()
    });

    let (alice, asset) = mint_alice_position(&merkle, &mut world);

    world.warp_past_end();
    world.subsequent_claim_ok(&alice.keypair, asset);

    assert_eq!(
        world.claimer_token_balance(&alice.keypair.pubkey()),
        alice.allocation
    );
    let collection = world.collection;
    assert!(world.fetch_permanent_freeze_delegate(&collection).frozen);
    assert!(!world.asset_has_freeze_delegate(&asset));
}

// ---------------------------------------------------------------------------
// exclude_asset
// ---------------------------------------------------------------------------

/// Creator can burn a partially-vested position; the unclaimed remainder
/// returns to the creator and subsequent claims fail.
#[test]
fn exclude_asset_blocks_subsequent_claims() {
    let (merkle, mut world) = setup(CampaignConfig::default());
    let (alice, asset) = mint_alice_position(&merkle, &mut world);

    world.warp_to(world.linear_checkpoint(50));
    world.subsequent_claim_ok(&alice.keypair, asset);

    let balance_after_partial = world.claimer_token_balance(&alice.keypair.pubkey());
    assert!(balance_after_partial > 0);
    assert!(balance_after_partial < alice.allocation);

    let creator_before = world.creator_token_balance();
    world.exclude_asset(asset);

    assert_eq!(
        world.creator_token_balance(),
        creator_before + (alice.allocation - balance_after_partial),
        "unclaimed remainder must return to creator"
    );

    world.warp_past_end();
    world.subsequent_claim_err(&alice.keypair, asset, "InvalidAsset");

    assert_eq!(
        world.claimer_token_balance(&alice.keypair.pubkey()),
        balance_after_partial
    );
}

/// Cannot exclude a fully claimed loyalty badge.
#[test]
fn exclude_asset_fails_when_fully_claimed() {
    let (merkle, mut world) = setup(CampaignConfig::default());
    let (alice, asset) = mint_alice_position(&merkle, &mut world);

    world.warp_past_end();
    world.subsequent_claim_ok(&alice.keypair, asset);

    world.exclude_asset_err(asset, "AlreadyFullyClaimed");
}

/// Confirms mpl-core BurnV1: asset invalidated, transfer blocked, creator refunded rent.
#[test]
fn exclude_asset_burns_position() {
    let (merkle, mut world) = setup(CampaignConfig::default());
    let bob = load_whitelist_user(&merkle, WHITELISTED_2);
    fund_keypair(&mut world, &bob.keypair, LAMPORTS);

    let (alice, asset) = mint_alice_position(&merkle, &mut world);

    assert!(world.asset_is_valid_mpl_core(&asset));
    assert_eq!(world.asset_owner(&asset), alice.keypair.pubkey());

    let creator = world.creator.pubkey();
    let creator_lamports_before = world.lamports(&creator);

    world.exclude_asset(asset);

    assert!(
        !world.asset_is_valid_mpl_core(&asset),
        "burned asset must not deserialize as BaseAssetV1"
    );

    let transfer_ix =
        world.transfer_asset_ix(&alice.keypair.pubkey(), &bob.keypair.pubkey(), &asset);
    let out = world.story.run_instruction(transfer_ix, &[&alice.keypair]);
    assert!(!out.success, "transfer of a burned asset must fail");
    world.after_tx();

    assert!(
        world.lamports(&creator) > creator_lamports_before,
        "burn payer should receive rent refund"
    );
}

// ---------------------------------------------------------------------------
// freeze_asset (per-asset admin pause)
// ---------------------------------------------------------------------------

/// Creator can pause transfers of a single position and later restore them.
#[test]
fn freeze_asset_blocks_transfer_and_unfreeze_restores() {
    let (merkle, mut world) = setup(CampaignConfig::default());
    let bob = load_whitelist_user(&merkle, WHITELISTED_2);
    fund_keypair(&mut world, &bob.keypair, LAMPORTS);

    let (alice, asset) = mint_alice_position(&merkle, &mut world);

    world.freeze_asset(asset, true);
    assert!(world.fetch_permanent_freeze_delegate(&asset).frozen);
    assert!(!world.transfer_changes_owner(&alice.keypair, &bob.keypair.pubkey(), &asset));

    world.freeze_asset(asset, false);
    assert!(!world.fetch_permanent_freeze_delegate(&asset).frozen);
    assert!(world.transfer_changes_owner(&alice.keypair, &bob.keypair.pubkey(), &asset));
}

/// Positions on non-transferable campaigns carry no freeze plugin and are rejected.
#[test]
fn freeze_asset_fails_without_freeze_plugin() {
    let (merkle, mut world) = setup(CampaignConfig {
        is_transferable: false,
        ..Default::default()
    });
    let (_alice, asset) = mint_alice_position(&merkle, &mut world);

    world.freeze_asset_err(asset, true, "FreezePluginMissing");
}

/// Only the campaign creator can toggle a per-asset freeze.
#[test]
fn freeze_asset_requires_creator() {
    let (merkle, mut world) = setup(CampaignConfig::default());
    let (alice, asset) = mint_alice_position(&merkle, &mut world);

    world.freeze_asset_by(&alice.keypair, asset, true, "Unauthorized");
}

// ---------------------------------------------------------------------------
// freeze_collection toggle vs final claim (regression)
// ---------------------------------------------------------------------------

/// Asset minted on a non-transferable campaign (no freeze plugin); admin later
/// unfreezes the collection (is_transferable becomes true). The final claim
/// must not attempt the loyalty-badge freeze on the missing plugin.
#[test]
fn final_claim_succeeds_after_collection_unfreeze_toggle() {
    let (merkle, mut world) = setup(CampaignConfig {
        is_transferable: false,
        ..Default::default()
    });
    let (alice, asset) = mint_alice_position(&merkle, &mut world);

    world.freeze_collection(false);
    assert!(world.campaign().is_transferable);

    world.warp_past_end();
    world.subsequent_claim_ok(&alice.keypair, asset);

    assert_eq!(
        world.claimer_token_balance(&alice.keypair.pubkey()),
        alice.allocation
    );
    assert!(!world.asset_has_freeze_delegate(&asset));
}

/// Mirror case: asset minted with the plugin (transferable campaign); admin
/// freezes the collection afterwards. The badge freeze still goes through.
#[test]
fn final_claim_freezes_badge_even_when_collection_frozen() {
    let (merkle, mut world) = setup(CampaignConfig::default());
    let (alice, asset) = mint_alice_position(&merkle, &mut world);

    world.freeze_collection(true);
    assert!(!world.campaign().is_transferable);

    world.warp_past_end();
    world.subsequent_claim_ok(&alice.keypair, asset);

    assert_eq!(
        world.claimer_token_balance(&alice.keypair.pubkey()),
        alice.allocation
    );
    assert!(world.fetch_permanent_freeze_delegate(&asset).frozen);
}
