//! Merkle helpers the tests need but the IDL can't provide: the program's leaf
//! hashing / proof verification (keccak256, ported from
//! `vesting_positions::utils::merkletree`) and the test-side tree loader
//! (ported from the old `tests/common/merkle.rs`). Neither depends on the
//! program crate: `leaf_hash`/`verify` are plain crypto over `Pubkey` and
//! bytes, so they live here rather than importing the 0.31 program.

use std::collections::HashMap;
use std::fs;
use std::path::Path;

use anchor_lang::prelude::Pubkey;
use serde::Deserialize;
use serde_json::Value;
use sha3::{Digest, Keccak256};

pub const GENERATED_MERKLE: &str = "tests/fixtures/merkle_proofs.json";
pub const MOCK_ALLOC: u64 = 1_000_000_000_000;
pub const TOTAL_DEPOSIT: u64 = 10 * MOCK_ALLOC;

/// A claimer's leaf: keccak256 of the lowercase base58 address concatenated
/// with the amount. Matches the program byte for byte.
pub fn leaf_hash(pubkey: &Pubkey, amount: u64) -> [u8; 32] {
    let addr = pubkey.to_string().to_lowercase();
    let leaf_str = format!("{}{}", addr, amount);
    let mut hasher = Keccak256::new();
    hasher.update(leaf_str.as_bytes());
    hasher.finalize().into()
}

/// Proof format "0x00" + hash or "0x01" + hash:
/// - position 0: we're the right child -> hash(sibling, current)
/// - position 1: we're the left child  -> hash(current, sibling)
pub fn verify(leaf_hash: [u8; 32], proof: &[[u8; 33]], root: &[u8; 32]) -> bool {
    let mut hash = leaf_hash;
    for step in proof {
        let position = step[0];
        let sibling: &[u8; 32] = step[1..33].try_into().unwrap();
        let mut combined = [0u8; 64];
        if position == 1 {
            combined[0..32].copy_from_slice(&hash);
            combined[32..64].copy_from_slice(sibling);
        } else {
            combined[0..32].copy_from_slice(sibling);
            combined[32..64].copy_from_slice(&hash);
        }
        let mut hasher = Keccak256::new();
        hasher.update(combined);
        hash = hasher.finalize().into();
    }
    &hash == root
}

#[derive(Clone)]
pub struct MerkleTree {
    pub root: [u8; 32],
    data: HashMap<String, Vec<Entry>>,
}

#[derive(Clone, Deserialize)]
struct Entry {
    amount: String,
    proofs: Vec<String>,
}

pub fn default_merkle() -> MerkleTree {
    load_merkle_data(GENERATED_MERKLE)
}

pub fn load_merkle_data(path: impl AsRef<Path>) -> MerkleTree {
    let raw: Value = serde_json::from_str(&fs::read_to_string(path.as_ref()).unwrap()).unwrap();
    let root = hex32(raw["merkleRoot"].as_str().expect("merkleRoot"));

    let mut data = HashMap::new();
    if let Value::Object(map) = raw {
        for (k, v) in map {
            if k == "merkleRoot" {
                continue;
            }
            let entries: Vec<Entry> = serde_json::from_value(v).unwrap();
            data.insert(k.to_lowercase(), entries);
        }
    }
    MerkleTree { root, data }
}

/// Look up `(allocation, proofs)` for a claimer pubkey.
pub fn get_proofs(merkle: &MerkleTree, claimer: &Pubkey) -> Option<(u64, Vec<[u8; 33]>)> {
    let key = claimer.to_string().to_lowercase();
    let entry = merkle.data.get(&key)?.first()?;
    let allocation = entry.amount.parse().ok()?;
    let proofs = entry.proofs.iter().map(|p| parse_proof_hex(p)).collect();
    Some((allocation, proofs))
}

/// Random invalid proofs for negative tests.
pub fn random_proofs() -> Vec<[u8; 33]> {
    vec![[1u8; 33], [2u8; 33]]
}

fn parse_proof_hex(s: &str) -> [u8; 33] {
    hex_bytes(s).try_into().expect("proof must be 33 bytes")
}

fn hex32(hex: &str) -> [u8; 32] {
    hex_bytes(hex).try_into().expect("root must be 32 bytes")
}

fn hex_bytes(s: &str) -> Vec<u8> {
    let hex = s.strip_prefix("0x").unwrap_or(s);
    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap())
        .collect()
}
