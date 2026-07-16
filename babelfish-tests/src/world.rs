//! Test world for a vesting campaign on the source-free frood harness.
//!
//! Mirrors `litesvm-tests`' `TestCampaign` method-for-method, so the ported
//! suites read the same: the world holds the shared roots (creator, mint,
//! collection) and each verb builds a generated bundle from them, letting the
//! resolver derive the rest (campaign, ATAs, update-authority) from the IDL
//! seeds. What differs from the anchor-litesvm original is only the substrate:
//! a frood `Story` over the committed `.so` + Codama IDL instead of a compiled
//! program crate, and instructions built through `frood gen`'s typed mirrors.

use frood::{Actor, Outcome, Story};
use frood_idl::types::Value;
use solana_account_info::AccountInfo;
use solana_clock::Clock;
use solana_instruction::Instruction;
use solana_keypair::Keypair;
use solana_pubkey::Pubkey;

use mpl_core::accounts::{BaseAssetV1, BaseCollectionV1};
use mpl_core::fetch_plugin;
use mpl_core::instructions::TransferV1Builder;
use mpl_core::types::{PermanentFreezeDelegate, PluginType};

use crate::merkle::{MerkleTree, TOTAL_DEPOSIT};
use crate::pda::{asset_pda, campaign_pda, collection_pda, receipt_pda};
use crate::vesting_gen::{
    cancel_campaign, claim, clawback, clawback_unclaimed, close_campaign, close_receipt,
    exclude_asset, freeze_asset, freeze_collection, initialize, ClaimArgs, ClawbackUnclaimedArgs,
    InitializeArgs,
};

const SO: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/fixtures/vesting_positions.so");
const MPL_CORE_SO: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/fixtures/mpl_core.so");
const IDL: &str = include_str!("../idls/vesting_positions.codama.json");

const MPL_CORE_ID: Pubkey = Pubkey::from_str_const("CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d");
const COMPUTE_BUDGET_ID: Pubkey =
    Pubkey::from_str_const("ComputeBudget111111111111111111111111111111");
const SYSTEM_PROGRAM_ID: Pubkey = Pubkey::from_str_const("11111111111111111111111111111111");
const LAMPORTS: u64 = 100 * 1_000_000_000;

/// First claim mints the position NFT (an mpl-core CreateV2 CPI); the
/// full-allocation-plus-freeze path tops the 200k default cap, so raise it.
pub const FIRST_CLAIM_CU: u32 = 250_000;

/// Solana's per-transaction compute cap when no `ComputeBudget` ix is present.
pub const DEFAULT_TX_CU: u32 = 200_000;

/// Log consumed vs requested limit (run with `cargo test compute_units -- --nocapture`).
pub fn log_tx_cu(label: &str, consumed: u64, limit: u32) {
    let over_default = consumed > DEFAULT_TX_CU as u64;
    println!(
        "[CU] {label}: {consumed} consumed / {limit} limit (default cap {DEFAULT_TX_CU}){}",
        if over_default {
            " — exceeds default"
        } else {
            ""
        }
    );
}

/// Assert an outcome failed with a named error. Anchor logs the error variant
/// name (`AlreadyClaimed`, `Unauthorized`, ...); a pre-execution rejection puts
/// the reason on `err` instead, so both surfaces are checked.
pub fn assert_err(out: &Outcome, name: &str) {
    let matched = !out.success
        && (out.err.as_deref().unwrap_or("").contains(name)
            || out.logs.iter().any(|l| l.contains(name)));
    assert!(
        matched,
        "expected error containing `{name}`, got success={} err={:?}\nlogs:\n{}",
        out.success,
        out.err,
        out.logs.join("\n"),
    );
}

/// Build a `ComputeBudget::SetComputeUnitLimit` instruction by hand (variant tag
/// 2, then the u32 limit little-endian), so the crate needn't pull the
/// compute-budget interface just to raise the cap for the NFT-minting claim.
fn set_cu_limit_ix(limit: u32) -> Instruction {
    let mut data = Vec::with_capacity(5);
    data.push(2u8);
    data.extend_from_slice(&limit.to_le_bytes());
    Instruction {
        program_id: COMPUTE_BUDGET_ID,
        accounts: vec![],
        data,
    }
}

/// Clone an `Actor` (its keypair is not `Clone`) via a byte round-trip, so a
/// world method can sign as its own stored creator without borrowing `self`
/// while also mutating the SVM.
fn clone_actor(a: &Actor) -> Actor {
    Actor {
        keypair: Keypair::try_from(&a.keypair.to_bytes()[..]).expect("clone actor keypair"),
        label: a.label.clone(),
    }
}

/// Every claim carries the same throwaway NFT metadata; the schedule math and
/// authorization are what the tests exercise, not the name/uri.
pub fn claim_args(proofs: Option<Vec<[u8; 33]>>, allocation: Option<u64>) -> ClaimArgs {
    ClaimArgs {
        proofs: proofs.map(|ps| ps.into_iter().map(|p| p.to_vec()).collect()),
        allocation,
        name: "Test asset".to_string(),
        uri: "https://example.com".to_string(),
    }
}

#[derive(Clone, Copy)]
pub struct CampaignConfig {
    pub now: i64,
    pub start: i64,
    pub end: i64,
    pub cliff_duration: u64,
    pub cliff_release_bps: u16,
    pub is_transferable: bool,
    pub grace_period: u64,
    pub total_deposit: u64,
}

impl Default for CampaignConfig {
    fn default() -> Self {
        let now = 1_700_000_000;
        let start = now + 86_400;
        Self {
            now,
            start,
            end: start + 86_400 * 30,
            cliff_duration: 86_400,
            cliff_release_bps: 1_000,
            is_transferable: true,
            grace_period: 604_800,
            total_deposit: TOTAL_DEPOSIT,
        }
    }
}

/// The on-chain `Campaign`, decoded from frood's dynamic `Value` into the
/// scalars the tests read. A projection, not the full account: the schedule
/// math and the `is_transferable` flag are all the suites need.
pub struct CampaignView {
    pub creator: Pubkey,
    pub merkle_root: Vec<u8>,
    pub start: i64,
    pub end: i64,
    pub cliff_duration: u64,
    pub cliff_release_bps: u16,
    pub grace_period: u64,
    pub total_deposit: u64,
    pub collection: Pubkey,
    pub is_transferable: bool,
}

impl CampaignView {
    fn from_value(v: &Value) -> Self {
        let fields = match v {
            Value::Struct(f) => f,
            other => panic!("campaign: expected Struct, got {other:?}"),
        };
        let get = |name: &str| -> &Value {
            fields
                .iter()
                .find(|(n, _)| n == name)
                .map(|(_, val)| val)
                .unwrap_or_else(|| panic!("campaign field `{name}` missing"))
        };
        CampaignView {
            creator: as_pubkey(get("creator")),
            merkle_root: as_bytes(get("merkleRoot")),
            start: as_i64(get("start")),
            end: as_i64(get("end")),
            cliff_duration: as_u64(get("cliffDuration")),
            cliff_release_bps: as_u16(get("cliffReleaseBps")),
            grace_period: as_u64(get("gracePeriod")),
            total_deposit: as_u64(get("totalDeposit")),
            collection: as_pubkey(get("collection")),
            is_transferable: as_bool(get("isTransferable")),
        }
    }
}

fn as_i64(v: &Value) -> i64 {
    match v {
        Value::I64(n) => *n,
        other => panic!("expected i64, got {other:?}"),
    }
}
fn as_u64(v: &Value) -> u64 {
    match v {
        Value::U64(n) => *n,
        other => panic!("expected u64, got {other:?}"),
    }
}
fn as_u16(v: &Value) -> u16 {
    match v {
        Value::U16(n) => *n,
        other => panic!("expected u16, got {other:?}"),
    }
}
fn as_bool(v: &Value) -> bool {
    match v {
        Value::Bool(b) => *b,
        other => panic!("expected bool, got {other:?}"),
    }
}
fn as_pubkey(v: &Value) -> Pubkey {
    match v {
        Value::Pubkey(b) => Pubkey::new_from_array(*b),
        other => panic!("expected pubkey, got {other:?}"),
    }
}
fn as_bytes(v: &Value) -> Vec<u8> {
    match v {
        Value::Bytes(b) => b.clone(),
        Value::Seq(items) => items
            .iter()
            .map(|x| match x {
                Value::U8(n) => *n,
                other => panic!("expected byte, got {other:?}"),
            })
            .collect(),
        other => panic!("expected bytes, got {other:?}"),
    }
}

pub struct VestingWorld {
    pub story: Story,
    pub creator: Actor,
    pub mint: Pubkey,
    pub collection: Pubkey,
    pub config: CampaignConfig,
}

impl VestingWorld {
    /// Load the program (and mpl-core alongside it), warp the clock, cast the
    /// creator, mint + fund its deposit, and pin the collection PDA.
    fn base(config: CampaignConfig, merkle: &MerkleTree) -> Self {
        let mut story = Story::load(SO, IDL);
        story
            .svm
            .add_program_from_file(MPL_CORE_ID, MPL_CORE_SO)
            .expect("load mpl_core.so");
        set_clock(&mut story, config.now);

        let creator = story.cast("Creator");
        // cast airdrops 10 SOL; top up so the creator can pay the campaign,
        // vault, and collection rents the initialize CPI creates.
        story
            .svm
            .airdrop(&creator.pubkey(), LAMPORTS)
            .expect("airdrop creator");
        let mint = story.mint(&creator, 6);
        story.fund(&creator, &[(mint, config.total_deposit)]);

        let (collection, _) = collection_pda(&creator.pubkey(), &mint, &merkle.root);
        story.alias(collection, "Collection");

        Self {
            story,
            creator,
            mint,
            collection,
            config,
        }
    }

    /// Build the world, fund the creator, and run initialize: a live campaign.
    pub fn initialized(merkle: &MerkleTree, config: CampaignConfig) -> Self {
        let mut world = Self::base(config, merkle);
        world.run_initialize(merkle);
        world
    }

    /// Build the world with tokens funded but initialize not yet run.
    pub fn uninitialized(merkle: &MerkleTree, config: CampaignConfig) -> Self {
        Self::base(config, merkle)
    }

    /// Run the `initialize` instruction, returning its outcome so callers that
    /// profile compute can read the consumed units. Blockhash is expired after.
    pub fn run_initialize(&mut self, merkle: &MerkleTree) -> Outcome {
        let creator = clone_actor(&self.creator);
        let builder = initialize()
            .creator(creator.pubkey())
            .mint(self.mint)
            .collection(self.collection)
            .args(self.initialize_args(merkle.root));
        let ix = self.story.ix_typed(builder).instruction().clone();
        let out = self.story.run_instruction(ix, &[&creator]);
        assert!(out.success, "initialize failed:\n{}", out.logs.join("\n"));
        self.after_tx();
        out
    }

    fn initialize_args(&self, merkle_root: [u8; 32]) -> InitializeArgs {
        InitializeArgs {
            merkle_root: merkle_root.to_vec(),
            start: self.config.start,
            end: self.config.end,
            cliff_duration: self.config.cliff_duration,
            cliff_release_bps: self.config.cliff_release_bps,
            mint_to_distribute: self.mint,
            is_transferable: self.config.is_transferable,
            grace_period: self.config.grace_period,
            total_deposit: self.config.total_deposit,
            name: "Vesting campaign".to_string(),
            uri: "https://example.com/collection.json".to_string(),
        }
    }

    // --- schedule bounds --------------------------------------------------------

    pub fn campaign_address(&self) -> Pubkey {
        campaign_pda(&self.collection).0
    }

    /// The on-chain Campaign account, decoded.
    pub fn campaign(&self) -> CampaignView {
        let val = self.story.account_as("campaign", &self.campaign_address());
        CampaignView::from_value(&val)
    }

    pub fn start(&self) -> i64 {
        self.config.start
    }
    pub fn end(&self) -> i64 {
        self.config.end
    }
    pub fn grace_period(&self) -> u64 {
        self.config.grace_period
    }
    pub fn cliff_release_bps(&self) -> u16 {
        self.config.cliff_release_bps
    }

    /// The instant the cliff ends and linear vesting begins.
    pub fn cliff_end(&self) -> i64 {
        self.config.start + self.config.cliff_duration as i64
    }

    /// The timestamp `pct`% through the linear window (0 = cliff_end, 100 = end).
    pub fn linear_checkpoint(&self, pct: u64) -> i64 {
        assert!(pct <= 100);
        if pct == 100 {
            return self.config.end;
        }
        let window = (self.config.end - self.cliff_end()) as u64;
        self.cliff_end() + (window * pct / 100) as i64
    }

    /// The program's schedule math, mirrored so a test can assert the expected
    /// release independently.
    pub fn expected_claimable(&self, now: i64, allocation: u64, claimed_so_far: u64) -> u64 {
        crate::vesting::compute_claimable(&self.campaign(), now, allocation, claimed_so_far)
            .expect("schedule math overflow")
    }

    // --- addresses + balances ---------------------------------------------------

    /// The position-NFT address for `user`, seeded `[asset, campaign, user]`.
    pub fn asset_for(&self, user: &Pubkey) -> Pubkey {
        asset_pda(&self.campaign_address(), user).0
    }

    /// The campaign's token vault (its ATA on the distributed mint).
    pub fn campaign_ata(&self) -> Pubkey {
        self.story.ata(&self.campaign_address(), &self.mint)
    }

    /// The token balance held in the campaign vault.
    pub fn vault_balance(&self) -> u64 {
        self.story.token_balance(&self.campaign_ata())
    }

    /// The claimer's ATA balance for the distributed mint (0 if never funded).
    pub fn claimer_token_balance(&self, user: &Pubkey) -> u64 {
        self.story.token_balance(&self.story.ata(user, &self.mint))
    }

    /// The creator's ATA balance for the distributed mint.
    pub fn creator_token_balance(&self) -> u64 {
        self.story
            .token_balance(&self.story.ata(&self.creator.pubkey(), &self.mint))
    }

    /// Lamports at `address` (0 if the account is gone).
    pub fn lamports(&self, address: &Pubkey) -> u64 {
        self.story
            .svm
            .get_account(address)
            .map(|a| a.lamports)
            .unwrap_or(0)
    }

    /// The receipt PDA for `user`.
    pub fn receipt_address(&self, user: &Pubkey) -> Pubkey {
        receipt_pda(&self.campaign_address(), user).0
    }

    /// The claimer recorded in `user`'s receipt.
    pub fn receipt_claimer(&self, user: &Pubkey) -> Pubkey {
        let val = self
            .story
            .account_as("claimReceipt", &self.receipt_address(user));
        match &val {
            Value::Struct(f) => f
                .iter()
                .find(|(n, _)| n == "claimer")
                .map(|(_, v)| as_pubkey(v))
                .expect("claimReceipt.claimer missing"),
            other => panic!("claimReceipt: expected Struct, got {other:?}"),
        }
    }

    pub fn assert_receipt_claimer(&self, user: &Pubkey) {
        assert_eq!(self.receipt_claimer(user), *user);
    }

    // --- claim ------------------------------------------------------------------

    /// Build a raw `claim` instruction. `asset` is the NFT the claim targets;
    /// the receipt is always the signer's own PDA. The resolver fills the
    /// campaign, ATAs, update-authority, and programs from the IDL seeds.
    pub fn claim_ix(
        &self,
        user: &Pubkey,
        asset: Pubkey,
        proofs: Option<Vec<[u8; 33]>>,
        allocation: Option<u64>,
    ) -> Instruction {
        let receipt = receipt_pda(&self.campaign_address(), user).0;
        let builder = claim()
            .user(*user)
            .collection(self.collection)
            .mint(self.mint)
            .asset(asset)
            .claim_receipt(receipt)
            .args(claim_args(proofs, allocation));
        self.story.ix_typed(builder).instruction().clone()
    }

    /// A first claim (proofs + allocation), warping to `start` if the clock is
    /// still early. Prepends a raised CU limit for the NFT-minting CPI.
    pub fn first_claim_ok(
        &mut self,
        user: &Actor,
        proofs: Vec<[u8; 33]>,
        allocation: u64,
    ) -> Outcome {
        if self.now() < self.config.start {
            self.warp_to(self.config.start);
        }
        let asset = self.asset_for(&user.pubkey());
        let claim_ix = self.claim_ix(&user.pubkey(), asset, Some(proofs), Some(allocation));
        let budget_ix = set_cu_limit_ix(FIRST_CLAIM_CU);
        let out = self
            .story
            .run_instructions(vec![budget_ix, claim_ix], &[user]);
        assert!(out.success, "first claim failed:\n{}", out.logs.join("\n"));
        self.after_tx();
        out
    }

    /// A first claim expected to fail validation before the NFT mint (so the
    /// default CU cap suffices). The caller pins the clock; this never warps.
    pub fn first_claim_err(
        &mut self,
        user: &Actor,
        proofs: Vec<[u8; 33]>,
        allocation: u64,
        error: &str,
    ) -> Outcome {
        let asset = self.asset_for(&user.pubkey());
        let claim_ix = self.claim_ix(&user.pubkey(), asset, Some(proofs), Some(allocation));
        let out = self.story.run_instruction(claim_ix, &[user]);
        assert_err(&out, error);
        self.after_tx();
        out
    }

    /// A subsequent claim on an already-minted `asset` (no proofs).
    pub fn subsequent_claim_ok(&mut self, user: &Actor, asset: Pubkey) -> Outcome {
        let claim_ix = self.claim_ix(&user.pubkey(), asset, None, None);
        let out = self.story.run_instruction(claim_ix, &[user]);
        assert!(
            out.success,
            "subsequent claim failed:\n{}",
            out.logs.join("\n")
        );
        self.after_tx();
        out
    }

    /// A subsequent claim on `asset` expected to fail with `error`.
    pub fn subsequent_claim_err(&mut self, user: &Actor, asset: Pubkey, error: &str) -> Outcome {
        let claim_ix = self.claim_ix(&user.pubkey(), asset, None, None);
        let out = self.story.run_instruction(claim_ix, &[user]);
        assert_err(&out, error);
        self.after_tx();
        out
    }

    // --- clawback ---------------------------------------------------------------

    fn clawback_ix(&self, creator: Pubkey, asset: Pubkey) -> Instruction {
        let builder = clawback()
            .creator(creator)
            .collection(self.collection)
            .mint(self.mint)
            .asset(asset);
        self.story.ix_typed(builder).instruction().clone()
    }

    pub fn clawback(&mut self, asset: Pubkey) -> Outcome {
        let creator = clone_actor(&self.creator);
        let ix = self.clawback_ix(creator.pubkey(), asset);
        let out = self.story.run_instruction(ix, &[&creator]);
        assert!(out.success, "clawback failed:\n{}", out.logs.join("\n"));
        self.after_tx();
        out
    }

    pub fn clawback_err(&mut self, asset: Pubkey, error: &str) -> Outcome {
        let creator = clone_actor(&self.creator);
        let ix = self.clawback_ix(creator.pubkey(), asset);
        let out = self.story.run_instruction(ix, &[&creator]);
        assert_err(&out, error);
        self.after_tx();
        out
    }

    /// A clawback signed by `impostor` (whose key also fills the creator field).
    pub fn clawback_by(&mut self, impostor: &Actor, asset: Pubkey, error: &str) -> Outcome {
        let ix = self.clawback_ix(impostor.pubkey(), asset);
        let out = self.story.run_instruction(ix, &[impostor]);
        assert_err(&out, error);
        self.after_tx();
        out
    }

    fn clawback_unclaimed_ix(
        &self,
        creator: Pubkey,
        recipient: Pubkey,
        allocation: u64,
        proofs: Vec<[u8; 33]>,
    ) -> Instruction {
        let campaign = self.campaign_address();
        let builder = clawback_unclaimed()
            .creator(creator)
            .collection(self.collection)
            .mint(self.mint)
            .asset(asset_pda(&campaign, &recipient).0)
            .claim_receipt(receipt_pda(&campaign, &recipient).0)
            .args(ClawbackUnclaimedArgs {
                original_recipient: recipient,
                allocation,
                proofs: proofs.into_iter().map(|p| p.to_vec()).collect(),
            });
        self.story.ix_typed(builder).instruction().clone()
    }

    pub fn clawback_unclaimed_ok(
        &mut self,
        recipient: Pubkey,
        allocation: u64,
        proofs: Vec<[u8; 33]>,
    ) -> Outcome {
        let creator = clone_actor(&self.creator);
        let ix = self.clawback_unclaimed_ix(creator.pubkey(), recipient, allocation, proofs);
        let out = self.story.run_instruction(ix, &[&creator]);
        assert!(
            out.success,
            "clawback_unclaimed failed:\n{}",
            out.logs.join("\n")
        );
        self.after_tx();
        out
    }

    pub fn clawback_unclaimed_err(
        &mut self,
        recipient: Pubkey,
        allocation: u64,
        proofs: Vec<[u8; 33]>,
        error: &str,
    ) -> Outcome {
        let creator = clone_actor(&self.creator);
        let ix = self.clawback_unclaimed_ix(creator.pubkey(), recipient, allocation, proofs);
        let out = self.story.run_instruction(ix, &[&creator]);
        assert_err(&out, error);
        self.after_tx();
        out
    }

    // --- campaign lifecycle -----------------------------------------------------

    fn close_campaign_ix(&self, creator: Pubkey) -> Instruction {
        let builder = close_campaign()
            .creator(creator)
            .collection(self.collection)
            .mint(self.mint);
        self.story.ix_typed(builder).instruction().clone()
    }

    pub fn close_campaign_ok(&mut self) -> Outcome {
        let creator = clone_actor(&self.creator);
        let ix = self.close_campaign_ix(creator.pubkey());
        let out = self.story.run_instruction(ix, &[&creator]);
        assert!(
            out.success,
            "close_campaign failed:\n{}",
            out.logs.join("\n")
        );
        self.after_tx();
        out
    }

    pub fn close_campaign_err(&mut self, error: &str) -> Outcome {
        let creator = clone_actor(&self.creator);
        let ix = self.close_campaign_ix(creator.pubkey());
        let out = self.story.run_instruction(ix, &[&creator]);
        assert_err(&out, error);
        self.after_tx();
        out
    }

    fn cancel_campaign_ix(&self, creator: Pubkey) -> Instruction {
        let builder = cancel_campaign()
            .creator(creator)
            .collection(self.collection)
            .mint(self.mint);
        self.story.ix_typed(builder).instruction().clone()
    }

    pub fn cancel_campaign_ok(&mut self) -> Outcome {
        let creator = clone_actor(&self.creator);
        let ix = self.cancel_campaign_ix(creator.pubkey());
        let out = self.story.run_instruction(ix, &[&creator]);
        assert!(
            out.success,
            "cancel_campaign failed:\n{}",
            out.logs.join("\n")
        );
        self.after_tx();
        out
    }

    pub fn cancel_campaign_err(&mut self, error: &str) -> Outcome {
        let creator = clone_actor(&self.creator);
        let ix = self.cancel_campaign_ix(creator.pubkey());
        let out = self.story.run_instruction(ix, &[&creator]);
        assert_err(&out, error);
        self.after_tx();
        out
    }

    pub fn cancel_campaign_by(&mut self, impostor: &Actor, error: &str) -> Outcome {
        let ix = self.cancel_campaign_ix(impostor.pubkey());
        let out = self.story.run_instruction(ix, &[impostor]);
        assert_err(&out, error);
        self.after_tx();
        out
    }

    fn exclude_asset_ix(&self, creator: Pubkey, asset: Pubkey) -> Instruction {
        let builder = exclude_asset()
            .creator(creator)
            .collection(self.collection)
            .asset(asset)
            .mint(self.mint);
        self.story.ix_typed(builder).instruction().clone()
    }

    pub fn exclude_asset(&mut self, asset: Pubkey) -> Outcome {
        let creator = clone_actor(&self.creator);
        let ix = self.exclude_asset_ix(creator.pubkey(), asset);
        let out = self.story.run_instruction(ix, &[&creator]);
        assert!(
            out.success,
            "exclude_asset failed:\n{}",
            out.logs.join("\n")
        );
        self.after_tx();
        out
    }

    pub fn exclude_asset_err(&mut self, asset: Pubkey, error: &str) -> Outcome {
        let creator = clone_actor(&self.creator);
        let ix = self.exclude_asset_ix(creator.pubkey(), asset);
        let out = self.story.run_instruction(ix, &[&creator]);
        assert_err(&out, error);
        self.after_tx();
        out
    }

    // --- freeze -----------------------------------------------------------------

    fn freeze_asset_ix(&self, creator: Pubkey, asset: Pubkey, should_freeze: bool) -> Instruction {
        let builder = freeze_asset()
            .creator(creator)
            .collection(self.collection)
            .asset(asset)
            .should_freeze(should_freeze);
        self.story.ix_typed(builder).instruction().clone()
    }

    pub fn freeze_asset(&mut self, asset: Pubkey, should_freeze: bool) -> Outcome {
        let creator = clone_actor(&self.creator);
        let ix = self.freeze_asset_ix(creator.pubkey(), asset, should_freeze);
        let out = self.story.run_instruction(ix, &[&creator]);
        assert!(out.success, "freeze_asset failed:\n{}", out.logs.join("\n"));
        self.after_tx();
        out
    }

    pub fn freeze_asset_err(&mut self, asset: Pubkey, should_freeze: bool, error: &str) -> Outcome {
        let creator = clone_actor(&self.creator);
        let ix = self.freeze_asset_ix(creator.pubkey(), asset, should_freeze);
        let out = self.story.run_instruction(ix, &[&creator]);
        assert_err(&out, error);
        self.after_tx();
        out
    }

    pub fn freeze_asset_by(
        &mut self,
        impostor: &Actor,
        asset: Pubkey,
        should_freeze: bool,
        error: &str,
    ) -> Outcome {
        let ix = self.freeze_asset_ix(impostor.pubkey(), asset, should_freeze);
        let out = self.story.run_instruction(ix, &[impostor]);
        assert_err(&out, error);
        self.after_tx();
        out
    }

    pub fn freeze_collection(&mut self, should_freeze: bool) -> Outcome {
        let creator = clone_actor(&self.creator);
        let builder = freeze_collection()
            .creator(creator.pubkey())
            .collection(self.collection)
            .should_freeze(should_freeze);
        let ix = self.story.ix_typed(builder).instruction().clone();
        let out = self.story.run_instruction(ix, &[&creator]);
        assert!(
            out.success,
            "freeze_collection failed:\n{}",
            out.logs.join("\n")
        );
        self.after_tx();
        out
    }

    // --- close_receipt ----------------------------------------------------------

    pub fn close_receipt_ok(&mut self, user: &Actor) -> Outcome {
        let campaign = self.campaign_address();
        let builder = close_receipt()
            .user(user.pubkey())
            .campaign(campaign)
            .claim_receipt(receipt_pda(&campaign, &user.pubkey()).0);
        let ix = self.story.ix_typed(builder).instruction().clone();
        let out = self.story.run_instruction(ix, &[user]);
        assert!(
            out.success,
            "close_receipt failed:\n{}",
            out.logs.join("\n")
        );
        self.after_tx();
        out
    }

    pub fn close_receipt_err(&mut self, user: &Actor, error: &str) -> Outcome {
        let campaign = self.campaign_address();
        let builder = close_receipt()
            .user(user.pubkey())
            .campaign(campaign)
            .claim_receipt(receipt_pda(&campaign, &user.pubkey()).0);
        let ix = self.story.ix_typed(builder).instruction().clone();
        let out = self.story.run_instruction(ix, &[user]);
        assert_err(&out, error);
        self.after_tx();
        out
    }

    // --- asset / mpl-core inspection --------------------------------------------

    /// The current owner of an mpl-core `asset`.
    pub fn asset_owner(&self, asset: &Pubkey) -> Pubkey {
        let account = self.story.svm.get_account(asset).expect("asset account");
        BaseAssetV1::from_bytes(&account.data)
            .expect("asset data")
            .owner
    }

    /// True when `asset` exists and still deserializes as a live mpl-core asset.
    pub fn asset_is_valid_mpl_core(&self, asset: &Pubkey) -> bool {
        match self.story.svm.get_account(asset) {
            None => false,
            Some(account) if account.owner != MPL_CORE_ID => false,
            Some(account) => BaseAssetV1::from_bytes(&account.data).is_ok(),
        }
    }

    /// The asset's PermanentFreezeDelegate plugin, if it carries one.
    pub fn try_fetch_asset_freeze_delegate(
        &self,
        asset: &Pubkey,
    ) -> Option<PermanentFreezeDelegate> {
        let account = self.story.svm.get_account(asset).expect("asset account");
        let mut lamports = account.lamports;
        let mut data = account.data;
        let owner = account.owner;
        let info = AccountInfo::new(asset, false, false, &mut lamports, &mut data, &owner, false);
        fetch_plugin::<BaseAssetV1, PermanentFreezeDelegate>(
            &info,
            PluginType::PermanentFreezeDelegate,
        )
        .ok()
        .map(|(_, delegate, _)| delegate)
    }

    /// Whether the asset carries a PermanentFreezeDelegate plugin at all.
    pub fn asset_has_freeze_delegate(&self, asset: &Pubkey) -> bool {
        self.try_fetch_asset_freeze_delegate(asset).is_some()
    }

    /// The permanent-freeze-delegate plugin on an asset or the collection.
    pub fn fetch_permanent_freeze_delegate(&self, address: &Pubkey) -> PermanentFreezeDelegate {
        let account = self.story.svm.get_account(address).expect("account");
        let mut lamports = account.lamports;
        let mut data = account.data;
        let owner = account.owner;
        let info = AccountInfo::new(
            address,
            false,
            false,
            &mut lamports,
            &mut data,
            &owner,
            false,
        );
        // The collection and an asset store the plugin under different base
        // types, so dispatch on which one this address is.
        if *address == self.collection {
            fetch_plugin::<BaseCollectionV1, PermanentFreezeDelegate>(
                &info,
                PluginType::PermanentFreezeDelegate,
            )
            .expect("collection PermanentFreezeDelegate plugin")
            .1
        } else {
            fetch_plugin::<BaseAssetV1, PermanentFreezeDelegate>(
                &info,
                PluginType::PermanentFreezeDelegate,
            )
            .expect("asset PermanentFreezeDelegate plugin")
            .1
        }
    }

    /// An mpl-core transfer of `asset` from one holder to another.
    pub fn transfer_asset_ix(&self, from: &Pubkey, to: &Pubkey, asset: &Pubkey) -> Instruction {
        TransferV1Builder::new()
            .asset(*asset)
            .collection(Some(self.collection))
            .payer(*from)
            .authority(Some(*from))
            .new_owner(*to)
            .system_program(Some(SYSTEM_PROGRAM_ID))
            .instruction()
    }

    /// Attempt an mpl-core transfer of `asset` and report whether the owner
    /// actually changed. A frozen position leaves the owner untouched; the
    /// failed transfer is swallowed so the caller reads the outcome from state.
    pub fn transfer_changes_owner(&mut self, from: &Actor, to: &Pubkey, asset: &Pubkey) -> bool {
        let owner_before = self.asset_owner(asset);
        let ix = self.transfer_asset_ix(&from.pubkey(), to, asset);
        let _ = self.story.run_instruction(ix, &[from]);
        self.after_tx();
        self.asset_owner(asset) != owner_before
    }

    // --- clock + housekeeping ---------------------------------------------------

    /// The current on-chain unix timestamp.
    pub fn now(&self) -> i64 {
        let clock: Clock = self.story.svm.get_sysvar();
        clock.unix_timestamp
    }

    /// Move the clock to `timestamp`.
    pub fn warp_to(&mut self, timestamp: i64) {
        set_clock(&mut self.story, timestamp);
    }

    /// Warp just past the vesting end (one second in).
    pub fn warp_past_end(&mut self) {
        self.warp_to(self.config.end + 1);
    }

    /// Warp past the grace window's close (one second in).
    pub fn warp_past_grace(&mut self) {
        self.warp_to(self.config.end + self.config.grace_period as i64 + 1);
    }

    /// Expire the blockhash so back-to-back txs aren't deduplicated as replays.
    pub fn after_tx(&mut self) {
        self.story.svm.expire_blockhash();
    }
}

/// Set the SVM clock to `timestamp`, nudging the slot so the sysvar write takes.
fn set_clock(story: &mut Story, timestamp: i64) {
    let mut clock: Clock = story.svm.get_sysvar();
    clock.unix_timestamp = timestamp;
    clock.slot = clock.slot.saturating_add(1);
    story.svm.set_sysvar(&clock);
}
