//! Full vesting lifecycle on the source-free world.
//!
//! Definitions:
//!   nft_position / asset  mpl-core position NFT (PDA `[asset, campaign, user]`, minted on first claim)
//!   vested_tokens         the schedule's per-claim output
//!
//! Alice first-claims at cliff_end, then claims again without proofs;
//! positions change hands and the report tracks who can claim what.

use anchor_litesvm::{md_kv, Report, Signer};
use vesting_litesvm_tests::campaign::{CampaignConfig, TestCampaign};
use vesting_litesvm_tests::common::{
    fund_keypair, load_keypair, load_whitelist_user, LAMPORTS, NOT_WHITELISTED, WHITELISTED_1,
    WHITELISTED_2,
};
use vesting_litesvm_tests::format::format_tokens;
use vesting_litesvm_tests::merkle::{default_merkle, TOTAL_DEPOSIT};

#[test]
fn full_lifecycle() {
    let mut md = Report::new(
        "Full vesting lifecycle",
        "demonstrate whole vesting life cycle including position transfers",
    );

    let merkle = default_merkle();
    let mut world = TestCampaign::initialized(&merkle, CampaignConfig::default());
    let alice = load_whitelist_user(&merkle, WHITELISTED_1);
    let charlie = load_whitelist_user(&merkle, WHITELISTED_2);
    let bob = load_keypair(NOT_WHITELISTED);

    fund_keypair(&mut world.ctx, &alice.keypair, LAMPORTS);
    fund_keypair(&mut world.ctx, &charlie.keypair, LAMPORTS);
    fund_keypair(&mut world.ctx, &bob, LAMPORTS);

    let creator = world.creator.pubkey();
    let campaign_addr = world.campaign_address();
    let campaign_vault = world.campaign_ata();
    world.ctx.alias(alice.keypair.pubkey(), "Alice pubkey");
    world.ctx.alias(bob.pubkey(), "Bob pubkey");
    world.ctx.alias(charlie.keypair.pubkey(), "Charlie pubkey");
    world.ctx.alias(creator, "campaign creator pubkey");
    world.ctx.alias(campaign_addr, "Campaign pubkey");

    md.block(
        "actors pubkeys",
        md_kv! {
            "campaign creator" => creator,
            "alice"            => alice.keypair.pubkey(),
            "bob"              => bob.pubkey(),
            "charlie"          => charlie.keypair.pubkey(),
        },
    );

    md.block(
        "PDAs",
        md_kv! {
            "campaign"       => campaign_addr,
            "campaign vault" => campaign_vault,
        },
    );

    let campaign = world.campaign();
    let config = CampaignConfig::default();

    md.step("Campaign initialized");
    md.check("campaign creator match", campaign.creator == creator, true);
    md.check(
        "campaign schedule match",
        campaign.start == config.start
            && campaign.end == config.end
            && campaign.cliff_duration == config.cliff_duration
            && campaign.cliff_release_bps == config.cliff_release_bps
            && campaign.grace_period == config.grace_period,
        true,
    );
    md.check(
        "merkle root and deposit match",
        campaign.merkle_root == merkle.root && campaign.total_deposit == TOTAL_DEPOSIT,
        true,
    );
    md.block(
        "campaign settings",
        md_kv! {
            "creator"              => world.ctx.label(&campaign.creator),
            "start (unix)"         => campaign.start,
            "end (unix)"           => campaign.end,
            "grace period (sec)"   => campaign.grace_period,
            "cliff duration (sec)" => campaign.cliff_duration,
            "cliff release (bps)"  => campaign.cliff_release_bps,
            "total deposit"        => format_tokens(campaign.total_deposit),
            "transferable"         => campaign.is_transferable,
        },
    );

    // --- Alice claims her vesting tokens at end of cliff period -----------------
    md.step("Alice first claim");
    world.warp_to(world.cliff_end());
    let alice_position = world.asset_for(&alice.keypair.pubkey());
    let charlie_position = world.asset_for(&charlie.keypair.pubkey());
    world.ctx.alias(alice_position, "Alice position NFT");
    world.ctx.alias(charlie_position, "Charlie position NFT");
    world
        .ctx
        .alias(campaign.collection, "NFT Collection for this campaign");

    md.block(
        "Collection & Assets",
        md_kv! {
            world.ctx.label(&campaign.collection) => campaign.collection,
            world.ctx.label(&alice_position)      => alice_position,
            world.ctx.label(&charlie_position)    => charlie_position,
        },
    );

    md.check(
        "alice and charlie have different allocations",
        alice.allocation != charlie.allocation,
        true,
    );

    let first_claim = world.first_claim_ok(&alice.keypair, alice.proofs.clone(), alice.allocation);
    world.record_execution("Act 1 — alice first claim", &first_claim);

    let alice_balance_after_cliff = world.claimer_token_balance(&alice.keypair.pubkey());
    let expected_cliff = world.expected_claimable(world.cliff_end(), alice.allocation, 0);

    md.transition(
        "Alice's token balance, after the cliff claim",
        format_tokens(0),
        format_tokens(expected_cliff),
        format_tokens(alice_balance_after_cliff),
        format!(
            "Alice received {}% of her allocation",
            campaign.cliff_release_bps / 100
        ),
    );

    md.check(
        "alice position nft exists",
        true,
        world.ctx.account_exists(&alice_position),
    );
    md.check(
        "alice position nft owner is alice",
        alice.keypair.pubkey(),
        world.asset_owner(&alice_position),
    );
    world.assert_receipt_claimer(&alice.keypair.pubkey());

    // --- Alice claims for elapsed time (50% of claim window) --------------------
    md.step("Alice subsequent claim without providing any proof");
    let mid = world.linear_checkpoint(50);
    world.warp_to(mid);
    let balance_before = world.claimer_token_balance(&alice.keypair.pubkey());
    let expected_mid = world.expected_claimable(mid, alice.allocation, balance_before);
    md.block(
        "subsequent claim",
        md_kv! {
            "claimant"            => world.ctx.label(&alice.keypair.pubkey()),
            "auth"                => "position NFT (no proofs)",
            "asset"               => world.ctx.label(&alice_position),
            "linear %"            => 50,
            "timestamp (unix)"    => mid,
            "balance before"      => format_tokens(balance_before),
            "incremental release" => format_tokens(expected_mid),
        },
    );

    let sub_claim = world.subsequent_claim_ok(&alice.keypair, alice_position);
    world.record_execution("Act 2 — alice subsequent claim", &sub_claim);

    md.transition(
        "Alice balance after second claim at 50% elapsed time",
        format_tokens(balance_before),
        format_tokens(balance_before + expected_mid),
        format_tokens(world.claimer_token_balance(&alice.keypair.pubkey())),
        "Alice received the expected amount for 50% elapsed time",
    );

    // --- Charlie first claim at 50% elapsed (parallel position) ----------------
    md.step("Charlie first claim at 50% elapsed");
    md.note("Charlie opens a separate position with his own merkle leaf and allocation");
    md.block(
        "first claim",
        md_kv! {
            "claimant"         => world.ctx.label(&charlie.keypair.pubkey()),
            "auth"             => "merkle proofs + allocation",
            "asset"            => world.ctx.label(&charlie_position),
            "allocation"       => format_tokens(charlie.allocation),
            "timestamp (unix)" => mid,
        },
    );

    let charlie_first_claim =
        world.first_claim_ok(&charlie.keypair, charlie.proofs.clone(), charlie.allocation);
    world.record_execution("Act 3 — charlie first claim", &charlie_first_claim);

    let expected_charlie_first = world.expected_claimable(mid, charlie.allocation, 0);
    let charlie_balance_after_first = world.claimer_token_balance(&charlie.keypair.pubkey());

    md.transition(
        "Charlie's token balance after first claim",
        format_tokens(0),
        format_tokens(expected_charlie_first),
        format_tokens(charlie_balance_after_first),
        "Charlie received cliff + linear vesting through 50% of the window",
    );

    md.check(
        "charlie position nft exists",
        true,
        world.ctx.account_exists(&charlie_position),
    );
    md.check(
        "charlie position nft owner is charlie",
        charlie.keypair.pubkey(),
        world.asset_owner(&charlie_position),
    );
    world.assert_receipt_claimer(&charlie.keypair.pubkey());

    // --- Alice transfers her vesting position to Bob ---------------------------
    md.step("NFT transfer to Bob");
    md.block(
        "transfer",
        md_kv! {
            "from"                  => world.ctx.label(&alice.keypair.pubkey()),
            "to"                    => world.ctx.label(&bob.pubkey()),
            "recipient whitelisted" => false,
            "asset"                 => world.ctx.label(&alice_position),
        },
    );
    let owner_before_transfer = world.asset_owner(&alice_position);
    let transfer_ix =
        world.transfer_asset_ix(&alice.keypair.pubkey(), &bob.pubkey(), &alice_position);
    world.ctx.tx(&[&alice.keypair]).ix(transfer_ix).send_ok();
    world.after_tx();
    let owner_after_transfer = world.asset_owner(&alice_position);

    md.transition(
        "Alice's position ownership",
        world.ctx.label(&owner_before_transfer),
        world.ctx.label(&bob.pubkey()),
        world.ctx.label(&owner_after_transfer),
        "Bob is now the owner of Alice's vesting position",
    );

    // --- Alice replays merkle proofs after transferring the NFT ----------------
    md.step("Alice replays merkle proofs after transferring the NFT");
    md.note("The program should reject minting a second vesting position for the same proofs");
    md.block(
        "attack",
        md_kv! {
            "vector"         => "first claim with valid merkle proofs after transfer",
            "goal"           => "mint a second NFT / reclaim allocation (infinite money trick)",
            "alice owns nft" => false,
            "expected error" => "AlreadyClaimed",
        },
    );
    md.check(
        "alice no longer owns the nft",
        alice.keypair.pubkey() != world.asset_owner(&alice_position),
        true,
    );

    let replay = world.first_claim_err(
        &alice.keypair,
        alice.proofs.clone(),
        alice.allocation,
        "AlreadyClaimed",
    );
    world.record_execution("Act 4 — merkle replay rejected", &replay);
    md.check(
        "claim receipt still bound to alice",
        alice.keypair.pubkey(),
        world.receipt_claimer(&alice.keypair.pubkey()),
    );

    // --- Alice tries to claim on her position after transferring it to Bob -----
    md.step("Alice subsequent claim after transferring her position");
    md.note(
        "Former owners cannot claim via their old receipt or whitelist status — NFT ownership is required",
    );
    md.block(
        "attack",
        md_kv! {
            "vector"         => "subsequent claim on transferred position",
            "claimant"       => world.ctx.label(&alice.keypair.pubkey()),
            "asset owner"    => world.ctx.label(&bob.pubkey()),
            "auth attempted" => "no proofs (former owner)",
            "expected error" => "NotAssetOwner",
        },
    );

    let alice_after_transfer =
        world.subsequent_claim_err(&alice.keypair, alice_position, "NotAssetOwner");
    world.record_execution("Act 5 — alice former owner rejected", &alice_after_transfer);

    // --- Bob claims tokens using Alice's position ------------------------------
    md.step("Bob claims via NFT ownership");
    let alice_claimed = world.claimer_token_balance(&alice.keypair.pubkey());
    let later = world.linear_checkpoint(75);
    world.warp_to(later);
    let bob_expected = world.expected_claimable(later, alice.allocation, alice_claimed);

    md.block(
        "subsequent claim",
        md_kv! {
            "claimant"         => world.ctx.label(&bob.pubkey()),
            "whitelisted"      => false,
            "asset"            => world.ctx.label(&alice_position),
            "auth"             => "NFT ownership only (no merkle proofs)",
            "linear %"         => 75,
            "timestamp (unix)" => later,
            "claimed so far"   => format_tokens(alice_claimed),
            "expected release" => format_tokens(bob_expected),
        },
    );

    md.note("Bob was never whitelisted; NFT ownership alone authorizes the subsequent claim");
    let bob_claim = world.subsequent_claim_ok(&bob, alice_position);
    world.record_execution("Act 6 — bob claim via nft ownership", &bob_claim);

    md.transition(
        "Bob's token balance after claiming with Alice's position",
        format_tokens(0),
        format_tokens(bob_expected),
        format_tokens(world.claimer_token_balance(&bob.pubkey())),
        "Bob claims vested tokens from Alice's allocation because he owns her position",
    );

    md.check(
        "alice position nft owner is still bob",
        bob.pubkey(),
        world.asset_owner(&alice_position),
    );

    // --- Charlie subsequent claim before transferring to Alice -----------------
    md.step("Charlie partial claim before transferring to Alice");
    let charlie_balance_before_sub = world.claimer_token_balance(&charlie.keypair.pubkey());
    let charlie_sub_expected =
        world.expected_claimable(later, charlie.allocation, charlie_balance_before_sub);

    md.block(
        "subsequent claim",
        md_kv! {
            "claimant"            => world.ctx.label(&charlie.keypair.pubkey()),
            "asset"               => world.ctx.label(&charlie_position),
            "auth"                => "position NFT (no proofs)",
            "linear %"            => 75,
            "timestamp (unix)"    => later,
            "balance before"      => format_tokens(charlie_balance_before_sub),
            "incremental release" => format_tokens(charlie_sub_expected),
        },
    );

    let charlie_sub_claim = world.subsequent_claim_ok(&charlie.keypair, charlie_position);
    world.record_execution("Act 7 — charlie subsequent claim", &charlie_sub_claim);

    let charlie_claimed_so_far = world.claimer_token_balance(&charlie.keypair.pubkey());
    md.transition(
        "Charlie's token balance after partial claim",
        format_tokens(charlie_balance_before_sub),
        format_tokens(charlie_balance_before_sub + charlie_sub_expected),
        format_tokens(charlie_claimed_so_far),
        "Charlie claimed more vested tokens before transferring the NFT",
    );

    // --- Charlie transfers his position to Alice -------------------------------
    md.step("Charlie transfers position to Alice");
    md.block(
        "transfer",
        md_kv! {
            "from"                  => world.ctx.label(&charlie.keypair.pubkey()),
            "to"                    => world.ctx.label(&alice.keypair.pubkey()),
            "recipient whitelisted" => true,
            "asset"                 => world.ctx.label(&charlie_position),
            "allocation on nft"     => format_tokens(charlie.allocation),
        },
    );
    md.note("Asset PDA and original_recipient stay bound to Charlie; only mpl-core owner changes");

    let charlie_owner_before = world.asset_owner(&charlie_position);
    let charlie_to_alice_ix = world.transfer_asset_ix(
        &charlie.keypair.pubkey(),
        &alice.keypair.pubkey(),
        &charlie_position,
    );
    world
        .ctx
        .tx(&[&charlie.keypair])
        .ix(charlie_to_alice_ix)
        .send_ok();
    world.after_tx();

    md.transition(
        "Charlie's position ownership",
        world.ctx.label(&charlie_owner_before),
        world.ctx.label(&alice.keypair.pubkey()),
        world.ctx.label(&world.asset_owner(&charlie_position)),
        "Alice now holds Charlie's vesting position NFT",
    );

    md.check(
        "alice and charlie positions are distinct nfts",
        true,
        alice_position != charlie_position,
    );

    // --- Charlie tries to claim after transferring his position to Alice -------
    md.step("Charlie subsequent claim after transferring his position");
    md.block(
        "attack",
        md_kv! {
            "vector"         => "subsequent claim on transferred position",
            "claimant"       => world.ctx.label(&charlie.keypair.pubkey()),
            "asset owner"    => world.ctx.label(&alice.keypair.pubkey()),
            "auth attempted" => "no proofs (former owner, still whitelisted)",
            "expected error" => "NotAssetOwner",
        },
    );

    let charlie_after_transfer =
        world.subsequent_claim_err(&charlie.keypair, charlie_position, "NotAssetOwner");
    world.record_execution(
        "Act 8 — charlie former owner rejected",
        &charlie_after_transfer,
    );

    // --- Alice claims remaining vesting on Charlie's position ------------------
    md.step("Alice claims Charlie's allocation via NFT ownership");
    let ninety = world.linear_checkpoint(90);
    world.warp_to(ninety);
    let alice_balance_before_charlie_claim = world.claimer_token_balance(&alice.keypair.pubkey());
    let alice_on_charlie_expected =
        world.expected_claimable(ninety, charlie.allocation, charlie_claimed_so_far);

    md.block(
        "subsequent claim",
        md_kv! {
            "claimant"                 => world.ctx.label(&alice.keypair.pubkey()),
            "whitelisted for own leaf" => true,
            "asset"                    => world.ctx.label(&charlie_position),
            "auth"                     => "NFT ownership (Charlie's position, not Alice's leaf)",
            "allocation on nft"        => format_tokens(charlie.allocation),
            "alice own allocation"     => format_tokens(alice.allocation),
            "linear %"                 => 90,
            "timestamp (unix)"         => ninety,
            "charlie claimed so far"   => format_tokens(charlie_claimed_so_far),
            "expected release"         => format_tokens(alice_on_charlie_expected),
        },
    );

    md.note(
        "Alice is whitelisted for her own leaf, but this claim uses Charlie's \
         allocation stored on his NFT — not Alice's merkle proofs",
    );

    let alice_on_charlie_claim = world.subsequent_claim_ok(&alice.keypair, charlie_position);
    world.record_execution(
        "Act 9 — alice claim on charlie position",
        &alice_on_charlie_claim,
    );

    let alice_balance_after_charlie_claim = world.claimer_token_balance(&alice.keypair.pubkey());
    let alice_increment = alice_balance_after_charlie_claim - alice_balance_before_charlie_claim;

    md.transition(
        "Alice token balance increment from Charlie's position",
        format_tokens(alice_balance_before_charlie_claim),
        format_tokens(alice_balance_before_charlie_claim + alice_on_charlie_expected),
        format_tokens(alice_balance_after_charlie_claim),
        "Alice received Charlie's vested tokens, not a replay of her own allocation",
    );

    md.check(
        "increment matches charlie allocation math",
        format_tokens(alice_on_charlie_expected),
        format_tokens(alice_increment),
    );
    let alice_own_expected_at_ninety =
        world.expected_claimable(ninety, alice.allocation, alice_claimed);
    md.check(
        "increment uses charlie allocation not alice own leaf",
        true,
        alice_increment != alice_own_expected_at_ninety,
    );
    md.check(
        "charlie's position owner is alice",
        alice.keypair.pubkey(),
        world.asset_owner(&charlie_position),
    );
    md.check(
        "Alice's original position owner is bob",
        bob.pubkey(),
        world.asset_owner(&alice_position),
    );

    world.report_execution(&mut md);
}
