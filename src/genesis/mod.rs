use crate::state::{Block, BlockHash, Transaction};
use crate::utils::{hash_block, get_current_timestamp};

/// Returns the hardcoded genesis block.
/// This block starts the chain and must be identical for all nodes.
pub fn create_genesis_block() -> Block {
    let transactions: Vec<Transaction> = vec![]; // Genesis block has no transactions

    let block = Block {
        height: 0,
        prev_hash: BlockHash::default(), // No previous hash for genesis
        timestamp: get_current_timestamp(), // Or a fixed timestamp if needed
        transactions,
        nonce: 0,
    };

    let block_hash = hash_block(&block);

    Block {
        hash: block_hash,
        ..block
    }
}
