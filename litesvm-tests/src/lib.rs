//! Source-free litesvm test harness for the `vesting_positions` program.
//!
//! The program (anchor 0.31 / solana 2.x) is driven through its committed `.so`
//! + IDL, so this crate stays on the modern anchor-litesvm stack without the
//! program ever entering its dependency graph. It carries the program helpers
//! the IDL can't provide (merkle crypto, collection-attribute readers), lifted
//! out of the program crate rather than imported from it.

pub mod asset;
pub mod merkle;

#[cfg(test)]
mod tests {
    use crate::merkle::{default_merkle, leaf_hash, verify};
    use anchor_lang::prelude::Pubkey;

    #[test]
    fn merkle_fixture_loads_and_crypto_roundtrips() {
        // The committed fixture parses into a non-zero root.
        let tree = default_merkle();
        assert_ne!(tree.root, [0u8; 32]);

        // leaf_hash is deterministic; an empty proof leaves the leaf equal to
        // the root.
        let who = Pubkey::new_unique();
        let leaf = leaf_hash(&who, 1_000);
        assert_eq!(leaf, leaf_hash(&who, 1_000));
        assert!(verify(leaf, &[], &leaf));
    }
}
