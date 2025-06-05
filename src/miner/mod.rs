use crate::state::{Block, BlockHash};
use crate::utils::{hash_block_with_nonce, get_current_timestamp};
use rayon::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

/// Represents mining difficulty as a target value.
pub struct Difficulty {
    pub target_prefix_zero_bits: usize,
}

/// Tries to mine a block by finding a nonce such that the hash meets the difficulty.
pub fn mine_block(mut block: Block, difficulty: Difficulty) -> Block {
    let found = Arc::new(AtomicBool::new(false));
    let timestamp = get_current_timestamp();

    block.timestamp = timestamp;

    // Generate a large range of nonces to search in parallel
    let (nonce, hash) = (0u64..u64::MAX)
        .into_par_iter()
        .with_max_len(10_000_000) // fine-grained load balancing
        .find_any(|&nonce| {
            if found.load(Ordering::Relaxed) {
                return false;
            }

            let hash = hash_block_with_nonce(&block, nonce);
            if meets_difficulty(&hash, &difficulty) {
                found.store(true, Ordering::Relaxed);
                true
            } else {
                false
            }
        })
        .map(|nonce| {
            let hash = hash_block_with_nonce(&block, nonce);
            (nonce, hash)
        })
        .expect("No valid nonce found");

    block.nonce = nonce;
    block.hash = hash;
    block
}

/// Check if a given hash meets the difficulty by examining the number of leading zero bits.
fn meets_difficulty(hash: &BlockHash, difficulty: &Difficulty) -> bool {
    let mut bits = 0;

    for byte in hash.0.iter() {
        if *byte == 0 {
            bits += 8;
        } else {
            bits += byte.leading_zeros() as usize;
            break;
        }
    }

    bits >= difficulty.target_prefix_zero_bits
}
