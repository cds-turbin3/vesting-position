//! PDA derivations the tests need that `bundles_from_idl!` doesn't generate.
//!
//! `collection` is seeded on the `merkle_root` instruction arg, so the macro
//! demotes it to a bundle field the caller supplies. This helper is that
//! caller: it derives the collection address byte-identically to the program.
//! Everything downstream (`campaign`, the ATAs) is seeded only on accounts, so
//! the macro generates those helpers and they are not repeated here.

use crate::vesting_positions;
use anchor_lang::prelude::Pubkey;

pub const COLLECTION: &[u8] = b"collection";

/// `[COLLECTION, creator, mint, merkle_root]` under the vesting program.
pub fn collection_pda(creator: &Pubkey, mint: &Pubkey, merkle_root: &[u8; 32]) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            COLLECTION,
            creator.as_ref(),
            mint.as_ref(),
            merkle_root.as_ref(),
        ],
        &vesting_positions::ID,
    )
}
