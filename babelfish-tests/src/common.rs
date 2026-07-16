//! Fixture-keypair loading + whitelist users. The whitelisted keypairs are the
//! ones baked into the committed merkle fixture, so a loaded user carries its
//! allocation and proofs straight from the tree.
//!
//! A loaded keypair becomes a frood [`Actor`] (the thing `Story::when` signs
//! with), wrapping the real ed25519 key so the pubkey still matches its merkle
//! leaf. Unlike `Story::cast`, loading does not airdrop; `fund_keypair` does
//! that separately, mirroring the anchor-litesvm flow (load, then fund).

use std::fs;
use std::path::Path;

use frood::Actor;
use solana_keypair::Keypair;

use crate::merkle::{get_proofs, MerkleTree};
use crate::world::VestingWorld;

pub const WHITELISTED_1: &str = "tests/fixtures/keypairs/whitelisted_1.json";
pub const WHITELISTED_2: &str = "tests/fixtures/keypairs/whitelisted_2.json";
pub const NOT_WHITELISTED: &str = "tests/fixtures/keypairs/not_whitelisted.json";

pub const LAMPORTS: u64 = 100 * 1_000_000_000;
pub const MOCK_ALLOC: u64 = crate::merkle::MOCK_ALLOC;

/// Map a fixture keypair's file stem to a crypto-convention actor name, so
/// reports read as a cast (Alice claims, Mallory is turned away) rather than
/// a roster of `whitelisted_1` stubs. The roster keys on the pubkey, so one
/// stable name per identity reads consistently across every report, even where
/// a given test locally nicknames the same key differently. Alice and Bob are
/// the two whitelisted claimants; Mallory is the non-whitelisted adversary.
fn actor_name(stem: &str) -> &str {
    match stem {
        "whitelisted_1" => "Alice",
        "whitelisted_2" => "Bob",
        "not_whitelisted" => "Mallory",
        other => other,
    }
}

/// Load a fixture keypair into a frood `Actor` (no airdrop). The label is the
/// friendly [`actor_name`] for the file stem, so reports name a cast.
pub fn load_keypair(path: impl AsRef<Path>) -> Actor {
    let path = path.as_ref();
    let bytes: Vec<u8> = serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap();
    let keypair = Keypair::try_from(bytes.as_slice()).expect("valid keypair bytes");
    let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("user");
    let label = actor_name(stem).to_string();
    Actor { keypair, label }
}

pub struct WhitelistUser {
    pub keypair: Actor,
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

/// Airdrop SOL to a loaded keypair so it can pay for its own claims.
pub fn fund_keypair(world: &mut VestingWorld, actor: &Actor, lamports: u64) {
    world
        .story
        .svm
        .airdrop(&actor.pubkey(), lamports)
        .expect("airdrop");
}
