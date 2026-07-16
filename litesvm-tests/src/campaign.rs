//! Test world for a vesting campaign, rebuilt on the source-free harness.
//!
//! The old world wrapped one hand-written `VestingBundle` holding every account.
//! Here the world holds the shared roots (creator, mint, collection) and builds
//! each instruction's generated bundle from them; the macro auto-derives the
//! rest (campaign, ATAs) from the IDL seeds. This is the `initialize` vertical
//! slice; per-instruction builders and fetches grow it as tests move over.

use anchor_lang::prelude::{AccountInfo, Pubkey};
use anchor_lang::solana_program::{instruction::Instruction, system_program};
use anchor_litesvm::{
    AnchorContext, AnchorLiteSVM, Keypair, Signer, TestHelpers, TransactionResult,
};
use mpl_core::accounts::{BaseAssetV1, BaseCollectionV1};
use mpl_core::fetch_plugin;
use mpl_core::instructions::TransferV1Builder;
use mpl_core::types::{Attributes, PermanentFreezeDelegate, PluginType};
use solana_compute_budget_interface::ComputeBudgetInstruction;
use spl_associated_token_account::get_associated_token_address;

use crate::merkle::{MerkleTree, TOTAL_DEPOSIT};
use crate::pda::{asset_pda, campaign_pda, collection_pda, receipt_pda};
use crate::vesting_positions::{self, accounts::Campaign};
use crate::{ClaimBundle, InitializeBundle};

const MPL_CORE_ID: Pubkey = Pubkey::from_str_const("CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d");
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

/// Every claim carries the same throwaway NFT metadata; the schedule math and
/// authorization are what the tests exercise, not the name/uri.
pub fn claim_args(
    proofs: Option<Vec<[u8; 33]>>,
    allocation: Option<u64>,
) -> vesting_positions::client::args::Claim {
    vesting_positions::client::args::Claim {
        proofs,
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

impl CampaignConfig {
    pub fn initialize_args(
        &self,
        merkle_root: [u8; 32],
        mint: Pubkey,
    ) -> vesting_positions::client::args::Initialize {
        vesting_positions::client::args::Initialize {
            merkle_root,
            start: self.start,
            end: self.end,
            cliff_duration: self.cliff_duration,
            cliff_release_bps: self.cliff_release_bps,
            mint_to_distribute: mint,
            is_transferable: self.is_transferable,
            grace_period: self.grace_period,
            total_deposit: self.total_deposit,
            name: "Vesting campaign".to_string(),
            uri: "https://example.com/collection.json".to_string(),
        }
    }
}

pub struct TestCampaign {
    pub ctx: AnchorContext,
    pub creator: Keypair,
    pub mint: Pubkey,
    pub collection: Pubkey,
    pub config: CampaignConfig,
}

impl TestCampaign {
    /// Build the world, fund the creator, and run initialize: a live campaign.
    pub fn initialized(tree: &MerkleTree, config: CampaignConfig) -> Self {
        let mut world = Self::uninitialized(tree, config);
        world.run_initialize(tree);
        world
    }

    /// Run the `initialize` instruction, returning its result so callers that
    /// profile compute can read the consumed units. Blockhash is expired after.
    pub fn run_initialize(&mut self, tree: &MerkleTree) -> TransactionResult {
        let bundle = InitializeBundle {
            creator: self.creator.pubkey(),
            mint: self.mint,
            collection: self.collection,
            ..Default::default()
        };
        let args = self.config.initialize_args(tree.root, self.mint);
        let result = self.ctx.tx(&[&self.creator]).build(bundle, args).send_ok();
        self.after_tx();
        result
    }

    /// Build the world with tokens funded but initialize not yet run. Creator
    /// and mint are cast as deterministic aliased identities, so every PDA the
    /// program derives downstream is pinned and reports stay reproducible.
    pub fn uninitialized(tree: &MerkleTree, config: CampaignConfig) -> Self {
        let mut ctx = build_ctx();
        ctx.svm.warp_to_timestamp(config.now);
        let creator = ctx.cast_actor_with_sol("creator", LAMPORTS);
        let mint = ctx.cast_mint("mint", &creator, 6);
        let (collection, _) = collection_pda(&creator.pubkey(), &mint, &tree.root);

        let creator_ata = ctx
            .svm
            .create_associated_token_account(&mint, &creator)
            .unwrap();
        ctx.svm
            .mint_to(&mint, &creator_ata, &creator, config.total_deposit)
            .unwrap();

        Self {
            ctx,
            creator,
            mint,
            collection,
            config,
        }
    }

    /// The campaign PDA, seeded on the collection (generated helper).
    pub fn campaign_address(&self) -> Pubkey {
        campaign_pda(&self.collection).0
    }

    /// The on-chain Campaign account, deserialized.
    pub fn campaign(&self) -> Campaign {
        self.ctx.load(&self.campaign_address())
    }

    // Schedule bounds live on `config`; expose the three the claim tests read so
    // call sites stay `world.start()` rather than reaching through `world.config`.
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
    /// release independently (see [`crate::vesting::compute_claimable`]).
    pub fn expected_claimable(&self, now: i64, allocation: u64, claimed_so_far: u64) -> u64 {
        crate::vesting::compute_claimable(&self.campaign(), now, allocation, claimed_so_far)
            .expect("schedule math overflow")
    }

    /// The position-NFT address for `user`, seeded `[asset, campaign, user]`.
    pub fn asset_for(&self, user: &Pubkey) -> Pubkey {
        asset_pda(&self.campaign_address(), user).0
    }

    /// The claimer's ATA balance for the distributed mint (0 if never funded).
    pub fn claimer_token_balance(&self, user: &Pubkey) -> u64 {
        let ata = get_associated_token_address(user, &self.mint);
        self.ctx.svm.token_balance(&ata).unwrap_or(0)
    }

    /// The generated claim bundle for `user`. `asset` is the NFT the claim
    /// targets; `None` uses the user's own first-claim PDA. The macro derives
    /// the campaign, ATAs, and update-authority; `collection`/`claim_receipt`
    /// demoted to fields (arg-seeded / cross-instruction-divergent), so the
    /// world supplies them from the extracted derivations.
    pub fn claim_bundle(&self, user: &Pubkey, asset: Option<Pubkey>) -> ClaimBundle {
        let campaign = self.campaign_address();
        ClaimBundle {
            user: *user,
            collection: self.collection,
            mint: self.mint,
            asset: asset.unwrap_or_else(|| asset_pda(&campaign, user).0),
            claim_receipt: receipt_pda(&campaign, user).0,
            ..Default::default()
        }
    }

    /// A first claim (proofs + allocation), warping to `start` if the clock is
    /// still early. Prepends a raised CU limit for the NFT-minting CPI.
    pub fn first_claim_ok(
        &mut self,
        user: &Keypair,
        proofs: Vec<[u8; 33]>,
        allocation: u64,
    ) -> TransactionResult {
        if self.ctx.svm.get_unix_timestamp() < self.config.start {
            self.warp_to(self.config.start);
        }
        let bundle = self.claim_bundle(&user.pubkey(), None);
        let claim_ix = self
            .ctx
            .program()
            .build_ix(bundle, claim_args(Some(proofs), Some(allocation)));
        let budget_ix = ComputeBudgetInstruction::set_compute_unit_limit(FIRST_CLAIM_CU);
        let result = self
            .ctx
            .execute_instructions(vec![budget_ix, claim_ix], &[user])
            .expect("first claim")
            .with_aliases(self.ctx.aliases.clone())
            .assert_success();
        self.after_tx();
        result
    }

    /// A first claim expected to fail validation before the NFT mint (so the
    /// default CU cap suffices). The caller pins the clock; this never warps.
    pub fn first_claim_err(
        &mut self,
        user: &Keypair,
        proofs: Vec<[u8; 33]>,
        allocation: u64,
        error: &str,
    ) {
        let bundle = self.claim_bundle(&user.pubkey(), None);
        self.ctx
            .tx(&[user])
            .build(bundle, claim_args(Some(proofs), Some(allocation)))
            .send_err_named(error);
        self.after_tx();
    }

    /// A subsequent claim on an already-minted `asset` (no proofs).
    pub fn subsequent_claim_ok(&mut self, user: &Keypair, asset: Pubkey) {
        let bundle = self.claim_bundle(&user.pubkey(), Some(asset));
        self.ctx
            .tx(&[user])
            .build(bundle, claim_args(None, None))
            .send_ok();
        self.after_tx();
    }

    /// A subsequent claim on `asset` expected to fail with `error`.
    pub fn subsequent_claim_err(&mut self, user: &Keypair, asset: Pubkey, error: &str) {
        let bundle = self.claim_bundle(&user.pubkey(), Some(asset));
        self.ctx
            .tx(&[user])
            .build(bundle, claim_args(None, None))
            .send_err_named(error);
        self.after_tx();
    }

    /// An mpl-core transfer of `asset` from one holder to another.
    pub fn transfer_asset_ix(&self, from: &Pubkey, to: &Pubkey, asset: &Pubkey) -> Instruction {
        TransferV1Builder::new()
            .asset(*asset)
            .collection(Some(self.collection))
            .payer(*from)
            .authority(Some(*from))
            .new_owner(*to)
            .system_program(Some(system_program::ID))
            .instruction()
    }

    /// The permanent-freeze-delegate plugin on an asset or the collection.
    pub fn fetch_permanent_freeze_delegate(&self, address: &Pubkey) -> PermanentFreezeDelegate {
        let account = self.ctx.svm.get_account(address).expect("account");
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

    /// Move the clock to `timestamp`.
    pub fn warp_to(&mut self, timestamp: i64) {
        self.ctx.svm.warp_to_timestamp(timestamp);
    }

    /// Warp just past the vesting end (one second in).
    pub fn warp_past_end(&mut self) {
        self.warp_to(self.config.end + 1);
    }

    /// Warp past the grace window's close (one second in).
    pub fn warp_past_grace(&mut self) {
        self.warp_to(self.config.end + self.config.grace_period as i64 + 1);
    }

    /// Expire the blockhash so back-to-back claims aren't deduplicated as
    /// replays of one transaction.
    pub fn after_tx(&mut self) {
        self.ctx.svm.expire_blockhash();
    }

    /// The claim receipt for `user`, asserting it records them as the claimer.
    pub fn assert_receipt_claimer(&self, user: &Pubkey) {
        let receipt: crate::vesting_positions::accounts::ClaimReceipt = self
            .ctx
            .load(&receipt_pda(&self.campaign_address(), user).0);
        assert_eq!(receipt.claimer, *user);
    }

    /// The schedule Attributes the program stored on the mpl-core collection.
    pub fn fetch_collection_attributes(&self) -> Attributes {
        let account = self
            .ctx
            .svm
            .get_account(&self.collection)
            .expect("collection account");
        let mut lamports = account.lamports;
        let mut data = account.data;
        let owner = account.owner;
        let info = AccountInfo::new(
            &self.collection,
            false,
            false,
            &mut lamports,
            &mut data,
            &owner,
            false,
        );
        let (_, attrs, _) =
            fetch_plugin::<BaseCollectionV1, Attributes>(&info, PluginType::Attributes)
                .expect("collection Attributes plugin");
        attrs
    }
}

fn build_ctx() -> AnchorContext {
    AnchorLiteSVM::build_with_programs(&[
        (
            vesting_positions::ID,
            "vesting_positions",
            include_bytes!("../tests/fixtures/vesting_positions.so"),
        ),
        (
            MPL_CORE_ID,
            "mpl_core",
            include_bytes!("../tests/fixtures/mpl_core.so"),
        ),
    ])
}
