//! PDA derivations the tests need that `bundles_from_idl!` doesn't generate.
//!
//! `collection` is seeded on the `merkle_root` instruction arg, and
//! `claim_receipt` derives differently across `claim`/`close_receipt`, so both
//! demote to bundle fields the caller supplies; the world derives them here,
//! byte-identically to the program. `asset` is the position NFT address, seeded
//! only on accounts but not itself an account in every instruction, so it too is
//! carried by hand. Everything else (campaign, ATAs) the macro generates.

use crate::vesting_positions;
use anchor_lang::prelude::Pubkey;

pub const COLLECTION: &[u8] = b"collection";
pub const CAMPAIGN: &[u8] = b"campaign";
pub const CLAIM: &[u8] = b"claim";
pub const ASSET: &[u8] = b"asset";

fn id() -> Pubkey {
    vesting_positions::ID
}

/// `[COLLECTION, creator, mint, merkle_root]`.
pub fn collection_pda(creator: &Pubkey, mint: &Pubkey, merkle_root: &[u8; 32]) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            COLLECTION,
            creator.as_ref(),
            mint.as_ref(),
            merkle_root.as_ref(),
        ],
        &id(),
    )
}

/// `[CAMPAIGN, collection]`.
pub fn campaign_pda(collection: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[CAMPAIGN, collection.as_ref()], &id())
}

/// `[CLAIM, campaign, user]`.
pub fn receipt_pda(campaign: &Pubkey, user: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[CLAIM, campaign.as_ref(), user.as_ref()], &id())
}

/// `[ASSET, campaign, user]`: the position NFT.
pub fn asset_pda(campaign: &Pubkey, user: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[ASSET, campaign.as_ref(), user.as_ref()], &id())
}
