//! Source-free litesvm test harness for the `vesting_positions` program.
//!
//! The program (anchor 0.31 / solana 2.x) is driven through its committed `.so`
//! + IDL, so this crate stays on the modern anchor-litesvm stack without the
//! program ever entering its dependency graph. The generated client (from
//! `declare_program!`) and per-instruction bundles (from `bundles_from_idl!`)
//! live here at crate root so the world builder and every test share them; the
//! non-IDL program helpers (merkle crypto, attribute readers) are lifted out of
//! the program crate rather than imported from it.
#![allow(unexpected_cfgs)]

// `self` binds the crate name so declare_program!'s generated modules can reach
// `anchor_lang` via `super::`.
use anchor_lang::{self};

anchor_lang::declare_program!(vesting_positions);
anchor_litesvm::bundles_from_idl!(vesting_positions);

pub mod asset;
pub mod campaign;
pub mod common;
pub mod merkle;
pub mod pda;

#[cfg(test)]
mod tests {
    use crate::merkle::{default_merkle, leaf_hash, verify};
    use anchor_lang::prelude::Pubkey;

    #[test]
    fn merkle_fixture_loads_and_crypto_roundtrips() {
        let tree = default_merkle();
        assert_ne!(tree.root, [0u8; 32]);

        let who = Pubkey::new_unique();
        let leaf = leaf_hash(&who, 1_000);
        assert_eq!(leaf, leaf_hash(&who, 1_000));
        assert!(verify(leaf, &[], &leaf));
    }
}
