//! Test world for a vesting campaign on the source-free frood harness.
//!
//! Mirrors `litesvm-tests`' `TestCampaign` method-for-method, so the ported
//! suites read the same: the world holds the shared roots (creator, mint,
//! collection) and each verb builds a generated bundle from them, letting the
//! resolver derive the rest (campaign, ATAs, update-authority) from the IDL
//! seeds. What differs from the anchor-litesvm original is only the substrate:
//! a frood `Story` over the committed `.so` + Codama IDL instead of a compiled
//! program crate, and instructions built through `frood gen`'s typed mirrors.
//!
//! The world no longer builds report `Block`s itself: `Story` mints a
//! `Moment` per transaction and samples every registered `observe`ation into
//! it, so all a send leaves behind is housekeeping (expire the blockhash,
//! register the signer's balance observation). The four private send helpers
//! (`creator_send_ok` and friends, below) own that tail, and every verb is a
//! builder plus one helper call. Rendering is `Story::project`'s job, run
//! once at `Drop` — see `VestingWorld::drop`.

use std::collections::HashMap;

use frood::{Actor, IntoBundle, Obs, Outcome, ReportState, Reporter, Story};
use frood_idl::types::Value;
use frood_idl::FromValue;
use solana_account::Account;
use solana_account_info::AccountInfo;
use solana_instruction::Instruction;
use solana_pubkey::Pubkey;

use mpl_core::accounts::{BaseAssetV1, BaseCollectionV1};
use mpl_core::instructions::TransferV1Builder;
use mpl_core::types::{PermanentFreezeDelegate, PluginType};
use mpl_core::{fetch_plugin, DataBlob, SolanaAccount};

use crate::merkle::{MerkleTree, TOTAL_DEPOSIT};
use crate::pda::{asset_pda, campaign_pda, collection_pda, receipt_pda};
use crate::vesting_gen::{
    cancel_campaign, claim, clawback, clawback_unclaimed, close_campaign, close_receipt,
    exclude_asset, freeze_asset, freeze_collection, initialize, Claim, ClaimArgs,
    ClawbackUnclaimedArgs, InitializeArgs,
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

/// The vesting world's closed action vocabulary — one variant per named
/// beat, most sent through `Story::when`. `IntoStaticStr` (PascalCase)
/// gives every label and every `History::count_actions` lookup the SAME
/// string by construction, so the typo class that a hand-typed `"Clm"`
/// literal would open never exists.
///
/// Two variants name beats the `IntoBundle` vocabulary cannot express,
/// sent through the labeled raw path (`Story::run_instructions_as`):
/// `FirstClaim`, whose bundle prepends a raw `ComputeBudget` instruction,
/// and `TransferPosition`, an mpl-core transfer (another program's
/// instruction entirely). `FirstClaim` is deliberately NOT `Claim`: the
/// full_lifecycle finally counts Claim-labeled transactions and means
/// only the subsequent kind, so the two claim shapes stay countable
/// apart.
#[derive(Clone, Copy, Debug, strum::IntoStaticStr)]
#[strum(serialize_all = "PascalCase")]
pub enum Action {
    Initialize,
    Claim,
    FirstClaim,
    Clawback,
    ClawbackUnclaimed,
    CloseCampaign,
    CancelCampaign,
    ExcludeAsset,
    FreezeAsset,
    FreezeCollection,
    CloseReceipt,
    TransferPosition,
}

impl Action {
    /// The label `Story::when` records and `History::count_actions` matches
    /// against. A thin wrapper over the `IntoStaticStr` projection so a call
    /// site reads `Action::Claim.label()` instead of the less obvious
    /// `<&str>::from(Action::Claim)`.
    pub fn label(self) -> &'static str {
        self.into()
    }
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

/// Read `address`'s `PermanentFreezeDelegate` plugin out of a fetched
/// account, if it carries one. `B` picks the mpl-core base the plugin
/// registry hangs off: `BaseAssetV1` for an asset, `BaseCollectionV1` for
/// the collection. `fetch_plugin` wants an `AccountInfo`, so the account's
/// fields are rebuilt into one on the stack — the one place that ritual
/// still lives.
fn permanent_freeze<B: DataBlob + SolanaAccount>(
    address: &Pubkey,
    account: Account,
) -> Option<PermanentFreezeDelegate> {
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
    fetch_plugin::<B, PermanentFreezeDelegate>(&info, PluginType::PermanentFreezeDelegate)
        .ok()
        .map(|(_, delegate, _)| delegate)
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
/// math and the `is_transferable` flag are all the suites need. The derive
/// reads each field under its camelCase IDL spelling (`merkle_root` reads
/// `merkleRoot`).
#[derive(FromValue)]
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

#[derive(Reporter)]
pub struct VestingWorld {
    pub story: Story,
    pub creator: Actor,
    pub mint: Pubkey,
    pub collection: Pubkey,
    pub config: CampaignConfig,
    report: ReportState,
    /// The campaign vault's token balance, observed at construction (before
    /// `initialize` even runs — `token_balance` reads 0 for an absent
    /// account) so it samples at every moment from T0 on.
    vault_obs: Obs,
    /// The creator's ATA balance, observed at construction alongside the
    /// vault: the pair a conservation law (`claimed + vault == deposit`)
    /// reads.
    creator_obs: Obs,
    /// Every actor's ATA balance a send has registered so far, keyed by
    /// pubkey so a repeat actor (the same claimer across several calls)
    /// reuses its handle instead of re-registering — replaces the old
    /// `StateRoster`: the trajectory samples every registered observation at
    /// every moment on its own, so there is nothing left to snapshot/diff by
    /// hand.
    balance_obs: HashMap<Pubkey, Obs>,
}

impl Drop for VestingWorld {
    fn drop(&mut self) {
        // The lifecycle's last safety net (see `Story::conclusion`'s doc):
        // a law that broke and that nothing ever asserted on must still fail
        // the test, even on a plain `cargo test` run with no report being
        // written. Idempotent (cached), so `project`'s own internal call
        // below is a no-op repeat, not a second evaluation. Guarded against
        // firing a second time while some OTHER assertion is already
        // unwinding this very drop (see `conclusion`'s doc).
        self.story.conclusion();
        if self.report.enabled() {
            let config = self.report.config();
            let arc = self.story.project(&config);
            self.report.set_arc(arc);
        }
        self.finish();
    }
}

impl VestingWorld {
    /// Load the program (and mpl-core alongside it), warp the clock, cast the
    /// creator, mint + fund its deposit, and pin the collection PDA. Marked
    /// `#[track_caller]` so the `ReportState` captured below chains back
    /// through `initialized`/`uninitialized` to the test's own call site.
    #[track_caller]
    fn base(config: CampaignConfig, merkle: &MerkleTree) -> Self {
        let mut story = Story::load(SO, IDL);
        story
            .svm
            .add_program_from_file(MPL_CORE_ID, MPL_CORE_SO)
            .expect("load mpl_core.so");
        story.warp_to(config.now);

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

        // Seed the vault/creator observations before initialize ever runs:
        // the vault doesn't exist yet, but its ATA address is deterministic
        // (PDA + mint), and `token_balance` reads 0 for an absent account, so
        // the very first moment already carries a meaningful sample.
        let campaign_address = campaign_pda(&collection).0;
        let vault_ata = story.ata(&campaign_address, &mint);
        let vault_obs = story.observe("Vault balance", move |s| {
            Value::U64(s.token_balance(&vault_ata))
        });
        let creator_pk = creator.pubkey();
        let creator_ata = story.ata(&creator_pk, &mint);
        let creator_obs = story.observe("Creator balance", move |s| {
            Value::U64(s.token_balance(&creator_ata))
        });
        let mut balance_obs = HashMap::new();
        balance_obs.insert(creator_pk, creator_obs);

        Self {
            story,
            creator,
            mint,
            collection,
            config,
            report: ReportState::from_manifest_toml(None),
            vault_obs,
            creator_obs,
            balance_obs,
        }
    }

    /// Build the world, fund the creator, and run initialize: a live campaign.
    #[track_caller]
    pub fn initialized(merkle: &MerkleTree, config: CampaignConfig) -> Self {
        let mut world = Self::base(config, merkle);
        world.run_initialize(merkle);
        world
    }

    /// Build the world with tokens funded but initialize not yet run.
    #[track_caller]
    pub fn uninitialized(merkle: &MerkleTree, config: CampaignConfig) -> Self {
        Self::base(config, merkle)
    }

    /// Run the `initialize` instruction, returning its outcome so callers that
    /// profile compute can read the consumed units. Blockhash is expired after.
    pub fn run_initialize(&mut self, merkle: &MerkleTree) -> Outcome {
        let builder = initialize()
            .creator(self.creator.pubkey())
            .mint(self.mint)
            .collection(self.collection)
            .args(self.initialize_args(merkle.root));
        self.creator_send_ok(Action::Initialize, builder)
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

    // --- the send helpers: every verb is a builder plus one of these ------------

    /// Run a creator-signed action and assert it succeeded: `Story::when_ok`
    /// plus the housekeeping every send shares (expire the blockhash, keep
    /// the creator's balance observed).
    #[track_caller]
    fn creator_send_ok(&mut self, action: Action, builder: impl IntoBundle) -> Outcome {
        let out = self
            .story
            .when_ok(action.label(), builder, &[&self.creator]);
        self.creator_housekeeping();
        out
    }

    /// Run a creator-signed action and assert it was refused with `error`.
    #[track_caller]
    fn creator_send_err(
        &mut self,
        action: Action,
        builder: impl IntoBundle,
        error: &str,
    ) -> Outcome {
        let out = self
            .story
            .when_err(action.label(), builder, &[&self.creator], error);
        self.creator_housekeeping();
        out
    }

    /// Run a `user`-signed action and assert it succeeded.
    #[track_caller]
    fn send_ok(&mut self, action: Action, builder: impl IntoBundle, user: &Actor) -> Outcome {
        let out = self.story.when_ok(action.label(), builder, &[user]);
        self.after_tx();
        self.observe_balance(user);
        out
    }

    /// Run a `user`-signed action and assert it was refused with `error`.
    #[track_caller]
    fn send_err(
        &mut self,
        action: Action,
        builder: impl IntoBundle,
        user: &Actor,
        error: &str,
    ) -> Outcome {
        let out = self.story.when_err(action.label(), builder, &[user], error);
        self.after_tx();
        self.observe_balance(user);
        out
    }

    fn creator_housekeeping(&mut self) {
        self.after_tx();
        let (pk, label) = (self.creator.pubkey(), self.creator.label.clone());
        self.observe_balance_at(pk, &label);
    }

    // --- schedule bounds --------------------------------------------------------

    pub fn campaign_address(&self) -> Pubkey {
        campaign_pda(&self.collection).0
    }

    /// The on-chain Campaign account, decoded.
    pub fn campaign(&self) -> CampaignView {
        let val = self.story.account_as("campaign", &self.campaign_address());
        CampaignView::from_value(&val).expect("campaign decodes as CampaignView")
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
        let claimer = val
            .at("claimer")
            .unwrap_or_else(|e| panic!("claimReceipt: {e}"));
        Pubkey::from_value(claimer).expect("claimReceipt.claimer is a pubkey")
    }

    pub fn assert_receipt_claimer(&self, user: &Pubkey) {
        assert_eq!(self.receipt_claimer(user), *user);
    }

    // --- the trajectory: observations, laws --------------------------------------

    /// The campaign vault's balance observation, registered at construction —
    /// the handle a conservation `law` or a `finally` over the vault reads.
    pub fn vault_obs(&self) -> Obs {
        self.vault_obs
    }

    /// The creator's balance observation, registered at construction.
    pub fn creator_obs(&self) -> Obs {
        self.creator_obs
    }

    /// Register (idempotently) an observation of `actor`'s token balance —
    /// what the send helpers use to watch a touched actor, and what a test
    /// hands straight to `Story::monotonic`/`constant`/`finally` ("claimed
    /// only ever grows" is exactly this observation's timeline). Reuses the
    /// SAME `Obs` across repeat calls for the same pubkey rather than
    /// re-registering, since a law/finally handle must stay stable.
    pub fn observe_balance(&mut self, actor: &Actor) -> Obs {
        self.observe_balance_at(actor.pubkey(), &actor.label)
    }

    fn observe_balance_at(&mut self, pk: Pubkey, fallback_label: &str) -> Obs {
        if let Some(obs) = self.balance_obs.get(&pk) {
            return *obs;
        }
        // Prefer the story's narrative alias: a test may cast a key against
        // its own plot (full_lifecycle names the NFT transferee "Bob", the
        // sympathetic inheritor, not the adversary its default label calls
        // "Mallory"), so the observation's rendered label reads as that same
        // cast. Fall back to the actor's default cast name.
        let label = self
            .story
            .aliases
            .get(&pk)
            .cloned()
            .unwrap_or_else(|| fallback_label.to_string());
        let ata = self.story.ata(&pk, &self.mint);
        let obs = self.story.observe(&format!("{label} balance"), move |s| {
            Value::U64(s.token_balance(&ata))
        });
        self.balance_obs.insert(pk, obs);
        obs
    }

    /// Register an observation of `allocation`'s claimable amount per the
    /// schedule, from a zero baseline (ignoring anything already claimed) —
    /// a pure function of the clock (the program's release math takes no
    /// per-user input beyond the allocation and what's claimed so far, see
    /// `vesting::compute_claimable`). `Story::sample`ing it across a
    /// `warp_to` with no transaction in between is what proves vesting
    /// moving on its own: the flagship case the whole trajectory design
    /// exists to show (see `Story::sample`'s doc). `label` names the
    /// observation for rendering (e.g. "Alice claimable (schedule ceiling)").
    pub fn observe_claimable(&mut self, label: &str, allocation: u64) -> Obs {
        let campaign_address = self.campaign_address();
        self.story.observe(label, move |s| {
            let val = s.account_as("campaign", &campaign_address);
            let campaign =
                CampaignView::from_value(&val).expect("campaign decodes as CampaignView");
            let claimable =
                crate::vesting::compute_claimable(&campaign, s.now(), allocation, 0).unwrap_or(0);
            Value::U64(claimable)
        })
    }

    /// Register an observation of whether `asset` currently carries a frozen
    /// `PermanentFreezeDelegate` — the loyalty-badge latch: once the full
    /// claim freezes a position, it must never come back unfrozen.
    pub fn observe_frozen(&mut self, asset: Pubkey) -> Obs {
        self.story.observe("asset frozen", move |s| {
            let frozen = s
                .svm
                .get_account(&asset)
                .and_then(|account| permanent_freeze::<BaseAssetV1>(&asset, account))
                .map(|delegate| delegate.frozen)
                .unwrap_or(false);
            Value::Bool(frozen)
        })
    }

    /// Register an observation of `user`'s claim-receipt claimer field — "the
    /// receipt stays bound to its claimer" is exactly this observation held
    /// `constant`. Must be registered AFTER the receipt exists (the first
    /// claim creates it); an earlier registration would auto-sample a
    /// missing account at every moment before that.
    pub fn observe_receipt_claimer(&mut self, user: Pubkey) -> Obs {
        let receipt = self.receipt_address(&user);
        self.story.observe("receipt claimer", move |s| {
            let val = s.account_as("claimReceipt", &receipt);
            val.at("claimer")
                .unwrap_or_else(|e| panic!("claimReceipt: {e}"))
                .clone()
        })
    }

    // --- claim ------------------------------------------------------------------

    /// The typed `claim` builder for `user` on `asset`. `asset` is the NFT
    /// the claim targets; the receipt is always the signer's own PDA. The
    /// resolver fills the campaign, ATAs, update-authority, and programs
    /// from the IDL seeds. Returns the concrete generated type because
    /// `claim_ix` lowers it through `ix_typed` (a `TypedIx`) while the send
    /// helpers take it as an `IntoBundle`; it is both.
    fn claim_builder(&self, user: Pubkey, asset: Pubkey, args: ClaimArgs) -> Claim {
        claim()
            .user(user)
            .collection(self.collection)
            .mint(self.mint)
            .asset(asset)
            .claim_receipt(receipt_pda(&self.campaign_address(), &user).0)
            .args(args)
    }

    /// Build a raw `claim` instruction, for the ComputeBudget-prefixed
    /// bundles `first_claim_ok`/`first_claim_err` assemble by hand.
    pub fn claim_ix(
        &self,
        user: &Pubkey,
        asset: Pubkey,
        proofs: Option<Vec<[u8; 33]>>,
        allocation: Option<u64>,
    ) -> Instruction {
        let builder = self.claim_builder(*user, asset, claim_args(proofs, allocation));
        self.story.ix_typed(builder).instruction().clone()
    }

    /// A first claim (proofs + allocation), warping to `start` if the clock is
    /// still early. Prepends a raised CU limit for the NFT-minting CPI: the
    /// `ComputeBudget` program's instruction has no `frood gen` typed mirror
    /// (it's not part of `vestingPositions`'s IDL), so this bundle can't go
    /// through `Story::when`'s `IntoBundle` vocabulary — it runs through the
    /// labeled raw path instead, named `FirstClaim` (see `Action`'s doc for
    /// why not `Claim`).
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
            .run_instructions_as(
                Action::FirstClaim.label(),
                vec![budget_ix, claim_ix],
                &[user],
            )
            .expect_success();
        self.after_tx();
        self.observe_balance(user);
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
        let out = self
            .story
            .run_instruction_as(Action::FirstClaim.label(), claim_ix, &[user]);
        self.story.then_err(&out, error);
        self.after_tx();
        self.observe_balance(user);
        out
    }

    /// A subsequent claim on an already-minted `asset` (no proofs).
    pub fn subsequent_claim_ok(&mut self, user: &Actor, asset: Pubkey) -> Outcome {
        let builder = self.claim_builder(user.pubkey(), asset, claim_args(None, None));
        self.send_ok(Action::Claim, builder, user)
    }

    /// A subsequent claim on `asset` expected to fail with `error`.
    pub fn subsequent_claim_err(&mut self, user: &Actor, asset: Pubkey, error: &str) -> Outcome {
        let builder = self.claim_builder(user.pubkey(), asset, claim_args(None, None));
        self.send_err(Action::Claim, builder, user, error)
    }

    // --- clawback ---------------------------------------------------------------

    /// `creator` fills the builder's creator field so the impostor variant
    /// (`clawback_by`) shares it; the honest paths pass the world's own.
    fn clawback_builder(&self, creator: Pubkey, asset: Pubkey) -> impl IntoBundle {
        clawback()
            .creator(creator)
            .collection(self.collection)
            .mint(self.mint)
            .asset(asset)
    }

    pub fn clawback(&mut self, asset: Pubkey) -> Outcome {
        let builder = self.clawback_builder(self.creator.pubkey(), asset);
        self.creator_send_ok(Action::Clawback, builder)
    }

    pub fn clawback_err(&mut self, asset: Pubkey, error: &str) -> Outcome {
        let builder = self.clawback_builder(self.creator.pubkey(), asset);
        self.creator_send_err(Action::Clawback, builder, error)
    }

    /// A clawback signed by `impostor` (whose key also fills the creator field).
    pub fn clawback_by(&mut self, impostor: &Actor, asset: Pubkey, error: &str) -> Outcome {
        let builder = self.clawback_builder(impostor.pubkey(), asset);
        self.send_err(Action::Clawback, builder, impostor, error)
    }

    fn clawback_unclaimed_builder(
        &self,
        creator: Pubkey,
        recipient: Pubkey,
        allocation: u64,
        proofs: Vec<[u8; 33]>,
    ) -> impl IntoBundle {
        let campaign = self.campaign_address();
        clawback_unclaimed()
            .creator(creator)
            .collection(self.collection)
            .mint(self.mint)
            .asset(asset_pda(&campaign, &recipient).0)
            .claim_receipt(receipt_pda(&campaign, &recipient).0)
            .args(ClawbackUnclaimedArgs {
                original_recipient: recipient,
                allocation,
                proofs: proofs.into_iter().map(|p| p.to_vec()).collect(),
            })
    }

    pub fn clawback_unclaimed_ok(
        &mut self,
        recipient: Pubkey,
        allocation: u64,
        proofs: Vec<[u8; 33]>,
    ) -> Outcome {
        let builder =
            self.clawback_unclaimed_builder(self.creator.pubkey(), recipient, allocation, proofs);
        self.creator_send_ok(Action::ClawbackUnclaimed, builder)
    }

    pub fn clawback_unclaimed_err(
        &mut self,
        recipient: Pubkey,
        allocation: u64,
        proofs: Vec<[u8; 33]>,
        error: &str,
    ) -> Outcome {
        let builder =
            self.clawback_unclaimed_builder(self.creator.pubkey(), recipient, allocation, proofs);
        self.creator_send_err(Action::ClawbackUnclaimed, builder, error)
    }

    // --- campaign lifecycle -----------------------------------------------------

    fn close_campaign_builder(&self, creator: Pubkey) -> impl IntoBundle {
        close_campaign()
            .creator(creator)
            .collection(self.collection)
            .mint(self.mint)
    }

    pub fn close_campaign_ok(&mut self) -> Outcome {
        let builder = self.close_campaign_builder(self.creator.pubkey());
        self.creator_send_ok(Action::CloseCampaign, builder)
    }

    pub fn close_campaign_err(&mut self, error: &str) -> Outcome {
        let builder = self.close_campaign_builder(self.creator.pubkey());
        self.creator_send_err(Action::CloseCampaign, builder, error)
    }

    fn cancel_campaign_builder(&self, creator: Pubkey) -> impl IntoBundle {
        cancel_campaign()
            .creator(creator)
            .collection(self.collection)
            .mint(self.mint)
    }

    pub fn cancel_campaign_ok(&mut self) -> Outcome {
        let builder = self.cancel_campaign_builder(self.creator.pubkey());
        self.creator_send_ok(Action::CancelCampaign, builder)
    }

    pub fn cancel_campaign_err(&mut self, error: &str) -> Outcome {
        let builder = self.cancel_campaign_builder(self.creator.pubkey());
        self.creator_send_err(Action::CancelCampaign, builder, error)
    }

    pub fn cancel_campaign_by(&mut self, impostor: &Actor, error: &str) -> Outcome {
        let builder = self.cancel_campaign_builder(impostor.pubkey());
        self.send_err(Action::CancelCampaign, builder, impostor, error)
    }

    fn exclude_asset_builder(&self, asset: Pubkey) -> impl IntoBundle {
        exclude_asset()
            .creator(self.creator.pubkey())
            .collection(self.collection)
            .asset(asset)
            .mint(self.mint)
    }

    pub fn exclude_asset(&mut self, asset: Pubkey) -> Outcome {
        let builder = self.exclude_asset_builder(asset);
        self.creator_send_ok(Action::ExcludeAsset, builder)
    }

    pub fn exclude_asset_err(&mut self, asset: Pubkey, error: &str) -> Outcome {
        let builder = self.exclude_asset_builder(asset);
        self.creator_send_err(Action::ExcludeAsset, builder, error)
    }

    // --- freeze -----------------------------------------------------------------

    fn freeze_asset_builder(
        &self,
        creator: Pubkey,
        asset: Pubkey,
        should_freeze: bool,
    ) -> impl IntoBundle {
        freeze_asset()
            .creator(creator)
            .collection(self.collection)
            .asset(asset)
            .should_freeze(should_freeze)
    }

    pub fn freeze_asset(&mut self, asset: Pubkey, should_freeze: bool) -> Outcome {
        let builder = self.freeze_asset_builder(self.creator.pubkey(), asset, should_freeze);
        self.creator_send_ok(Action::FreezeAsset, builder)
    }

    pub fn freeze_asset_err(&mut self, asset: Pubkey, should_freeze: bool, error: &str) -> Outcome {
        let builder = self.freeze_asset_builder(self.creator.pubkey(), asset, should_freeze);
        self.creator_send_err(Action::FreezeAsset, builder, error)
    }

    pub fn freeze_asset_by(
        &mut self,
        impostor: &Actor,
        asset: Pubkey,
        should_freeze: bool,
        error: &str,
    ) -> Outcome {
        let builder = self.freeze_asset_builder(impostor.pubkey(), asset, should_freeze);
        self.send_err(Action::FreezeAsset, builder, impostor, error)
    }

    pub fn freeze_collection(&mut self, should_freeze: bool) -> Outcome {
        let builder = freeze_collection()
            .creator(self.creator.pubkey())
            .collection(self.collection)
            .should_freeze(should_freeze);
        self.creator_send_ok(Action::FreezeCollection, builder)
    }

    // --- close_receipt ----------------------------------------------------------

    fn close_receipt_builder(&self, user: Pubkey) -> impl IntoBundle {
        let campaign = self.campaign_address();
        close_receipt()
            .user(user)
            .campaign(campaign)
            .claim_receipt(receipt_pda(&campaign, &user).0)
    }

    pub fn close_receipt_ok(&mut self, user: &Actor) -> Outcome {
        let builder = self.close_receipt_builder(user.pubkey());
        self.send_ok(Action::CloseReceipt, builder, user)
    }

    pub fn close_receipt_err(&mut self, user: &Actor, error: &str) -> Outcome {
        let builder = self.close_receipt_builder(user.pubkey());
        self.send_err(Action::CloseReceipt, builder, user, error)
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

    /// The asset's PermanentFreezeDelegate plugin, if it carries one. A
    /// missing ACCOUNT still panics (that is a test wiring bug, not a state
    /// this reads); only a missing PLUGIN answers `None`.
    pub fn try_fetch_asset_freeze_delegate(
        &self,
        asset: &Pubkey,
    ) -> Option<PermanentFreezeDelegate> {
        let account = self.story.svm.get_account(asset).expect("asset account");
        permanent_freeze::<BaseAssetV1>(asset, account)
    }

    /// Whether the asset carries a PermanentFreezeDelegate plugin at all.
    pub fn asset_has_freeze_delegate(&self, asset: &Pubkey) -> bool {
        self.try_fetch_asset_freeze_delegate(asset).is_some()
    }

    /// The permanent-freeze-delegate plugin on an asset or the collection.
    pub fn fetch_permanent_freeze_delegate(&self, address: &Pubkey) -> PermanentFreezeDelegate {
        let account = self.story.svm.get_account(address).expect("account");
        // The collection and an asset store the plugin under different base
        // types, so dispatch on which one this address is.
        if *address == self.collection {
            permanent_freeze::<BaseCollectionV1>(address, account)
                .expect("collection PermanentFreezeDelegate plugin")
        } else {
            permanent_freeze::<BaseAssetV1>(address, account)
                .expect("asset PermanentFreezeDelegate plugin")
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
        self.story
            .run_instruction_as(Action::TransferPosition.label(), ix, &[from]);
        self.after_tx();
        self.observe_balance(from);
        self.asset_owner(asset) != owner_before
    }

    // --- clock + housekeeping ---------------------------------------------------

    /// The current on-chain unix timestamp.
    pub fn now(&self) -> i64 {
        self.story.now()
    }

    /// Move the clock to `timestamp`.
    pub fn warp_to(&mut self, timestamp: i64) {
        self.story.warp_to(timestamp);
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
