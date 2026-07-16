//! Fixture-keypair loading + whitelist users, ported from the old
//! `tests/common/keypair.rs`. The whitelisted keypairs are the ones baked into
//! the committed merkle fixture, so a loaded user carries its allocation and
//! proofs straight from the tree.

use std::fs;
use std::path::Path;

use anchor_litesvm::{AnchorContext, Keypair, Signer};

use crate::merkle::{get_proofs, MerkleTree};

pub const WHITELISTED_1: &str = "tests/fixtures/keypairs/whitelisted_1.json";
pub const WHITELISTED_2: &str = "tests/fixtures/keypairs/whitelisted_2.json";
pub const NOT_WHITELISTED: &str = "tests/fixtures/keypairs/not_whitelisted.json";

pub const LAMPORTS: u64 = 100 * 1_000_000_000;
pub const MOCK_ALLOC: u64 = crate::merkle::MOCK_ALLOC;

pub fn load_keypair(path: impl AsRef<Path>) -> Keypair {
    let bytes: Vec<u8> = serde_json::from_str(&fs::read_to_string(path.as_ref()).unwrap()).unwrap();
    Keypair::try_from(bytes.as_slice()).expect("valid keypair bytes")
}

pub struct WhitelistUser {
    pub keypair: Keypair,
    pub allocation: u64,
    pub proofs: Vec<[u8; 33]>,
}

pub fn load_whitelist_user(merkle: &MerkleTree, path: impl AsRef<Path>) -> WhitelistUser {
    let keypair = load_keypair(path);
    let (allocation, proofs) = get_proofs(merkle, &keypair.pubkey()).expect("user in merkle tree");
    WhitelistUser {
        keypair,
        allocation,
        proofs,
    }
}

pub fn fund_keypair(ctx: &mut AnchorContext, keypair: &Keypair, lamports: u64) {
    ctx.svm
        .airdrop(&keypair.pubkey(), lamports)
        .expect("airdrop");
}
