//! Test world for a vesting campaign, rebuilt on the source-free harness.
//!
//! The old world wrapped one hand-written `VestingBundle` holding every account.
//! Here the world holds the shared roots (creator, mint, collection) and builds
//! each instruction's generated bundle from them; the macro auto-derives the
//! rest (campaign, ATAs) from the IDL seeds. This is the `initialize` vertical
//! slice; per-instruction builders and fetches grow it as tests move over.

use anchor_lang::prelude::{AccountInfo, Pubkey};
use anchor_litesvm::{AnchorContext, AnchorLiteSVM, Keypair, Signer, TestHelpers};
use mpl_core::accounts::BaseCollectionV1;
use mpl_core::fetch_plugin;
use mpl_core::types::{Attributes, PluginType};

use crate::merkle::{MerkleTree, TOTAL_DEPOSIT};
use crate::pda::collection_pda;
use crate::vesting_positions::{self, accounts::Campaign};
use crate::{campaign_pda, InitializeBundle};

const MPL_CORE_ID: Pubkey = Pubkey::from_str_const("CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d");
const LAMPORTS: u64 = 100 * 1_000_000_000;

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
        let bundle = InitializeBundle {
            creator: world.creator.pubkey(),
            mint: world.mint,
            collection: world.collection,
            ..Default::default()
        };
        world
            .ctx
            .tx(&[&world.creator])
            .build(bundle, config.initialize_args(tree.root, world.mint))
            .send_ok();
        world
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
