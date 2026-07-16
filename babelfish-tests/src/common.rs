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

/// Read a fixture keypair into a frood `Actor` (no airdrop, no aliasing).
/// The label is the friendly [`actor_name`] for the file stem; registration
/// into a story is the seam functions' job ([`load_keypair`],
/// [`load_whitelist_user`]).
fn read_keypair(path: impl AsRef<Path>) -> Actor {
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

/// Load a non-whitelisted fixture keypair and register its wallet under the
/// roster name (Mallory). Wallet only, deliberately: a non-claimant's derived
/// asset/receipt would sit in the cast as rows for accounts that never exist;
/// a test that drives a refused claim names the trio with
/// `world.name_claimer` where the addresses actually appear.
pub fn load_keypair(world: &mut VestingWorld, path: impl AsRef<Path>) -> Actor {
    let actor = read_keypair(path);
    world.story.alias(actor.pubkey(), &actor.label);
    actor
}

pub fn load_whitelist_user(
    world: &mut VestingWorld,
    merkle: &MerkleTree,
    path: impl AsRef<Path>,
) -> WhitelistUser {
    let keypair = read_keypair(path);
    let (allocation, proofs) = get_proofs(merkle, &keypair.pubkey()).expect("user in merkle tree");
    // Register the claimant's whole trio under the roster name the Actor
    // already carries ([`actor_name`]): the wallet, the position NFT, and
    // the claim receipt. This is the seam where a claimant enters a story
    // (the merkle root pins their exact key, so `cast` cannot mint them),
    // and authored aliases win over auto-composition, so registering here,
    // before the first claim resolves anything, also names the composites
    // that embed the wallet (`userAta(Alice, token, Mint)`).
    world.name_claimer(&keypair.pubkey(), &keypair.label.clone());
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

/// A whitelisted fixture's allocation, read from the tree without touching a
/// world: for tests whose `CampaignConfig` depends on it before the world
/// exists (an exact-funded campaign), so the world can be built first and
/// `load_whitelist_user`'s alias registration still lands in it.
pub fn whitelist_allocation(merkle: &MerkleTree, path: impl AsRef<Path>) -> u64 {
    let keypair = read_keypair(path);
    get_proofs(merkle, &keypair.pubkey())
        .expect("user in merkle tree")
        .0
}
