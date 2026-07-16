//! PDA derivations the tests need but the resolver doesn't hand back by name.
//!
//! Derived byte-identically to the program (same seeds, same program id). The
//! IDL is the source of the seed templates; here they're spelled out so a test
//! can name a receipt or asset address before any instruction runs. The program
//! id is the IDL's declared address, so nothing pulls in the anchor-0.31 crate.

use solana_pubkey::Pubkey;

/// The `vesting_positions` program id (the IDL's declared address).
pub const PROGRAM_ID: Pubkey =
    Pubkey::from_str_const("7DkU9TQhcN87f2djZDd2MjjPZoXLfnZZj8HhybeZswX1");

pub const COLLECTION: &[u8] = b"collection";
pub const CAMPAIGN: &[u8] = b"campaign";
pub const CLAIM: &[u8] = b"claim";
pub const ASSET: &[u8] = b"asset";

/// `[COLLECTION, creator, mint, merkle_root]`.
pub fn collection_pda(creator: &Pubkey, mint: &Pubkey, merkle_root: &[u8; 32]) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            COLLECTION,
            creator.as_ref(),
            mint.as_ref(),
            merkle_root.as_ref(),
        ],
        &PROGRAM_ID,
    )
}

/// `[CAMPAIGN, collection]`.
pub fn campaign_pda(collection: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[CAMPAIGN, collection.as_ref()], &PROGRAM_ID)
}

/// `[CLAIM, campaign, user]`.
pub fn receipt_pda(campaign: &Pubkey, user: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[CLAIM, campaign.as_ref(), user.as_ref()], &PROGRAM_ID)
}

/// `[ASSET, campaign, user]`: the position NFT.
pub fn asset_pda(campaign: &Pubkey, user: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[ASSET, campaign.as_ref(), user.as_ref()], &PROGRAM_ID)
}
