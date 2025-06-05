use crate::state::Block;
use crate::state::BlockHash;
use sha2::{Sha256, Digest};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn hash_block(block: &Block) -> BlockHash {
    let mut hasher = Sha256::new();

    // Serialize parts of the block manually or with bincode/json if stable format is desired
    hasher.update(&block.height.to_be_bytes());
    hasher.update(&block.prev_hash.0);
    hasher.update(&block.timestamp.to_be_bytes());
    hasher.update(&block.nonce.to_be_bytes());

    // Omit transactions for simplicity; add them if hash must include them
    let result = hasher.finalize();
    let mut hash = [0u8; 32];
    hash.copy_from_slice(&result);
    BlockHash(hash)
}

pub fn get_current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}
