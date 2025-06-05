use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub height: u64,
    pub prev_hash: BlockHash,
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
    pub nonce: u64,
    #[serde(skip)]
    pub hash: BlockHash,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct BlockHash(pub [u8; 32]);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    // Fill in fields as necessary
}
