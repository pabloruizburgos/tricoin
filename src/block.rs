use crate::transaction::Transaction;
use crate::utils::current_timestamp;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub data: Vec<Transaction>,
    pub merkle_root: String, // NOTE: understand well
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64, // For the PoW, when changed, the hash canges (here it is incremental by 1)
}

impl Block {
    pub fn new(index: u64, data: Vec<Transaction>, previous_hash: String) -> Self {
        let mut block = Block {
            index,
            timestamp: 0, // Time will be set when the block is mined
            data,
            merkle_root: String::new(),
            previous_hash,
            hash: String::new(),
            nonce: 0,
        };
        block.hash = block.calculate_hash();
        block.merkle_root = block.calculate_merkle_root();
        block
    }

    pub fn calculate_hash(&self) -> String {
        // NOTE: now we rehash the merkle root, not using the data directly. Cool!
        let data = format!(
            "{}{}{}{}{}",
            self.index, self.timestamp, self.merkle_root, self.previous_hash, self.nonce
        );
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }

    pub fn mine_block(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty); // Target: first difficulty bytes must be 0
        while &self.hash[..difficulty] != target {
            self.timestamp = current_timestamp();
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }
        println!("Block mined: {}", self.hash)
    }

    fn calculate_merkle_root(&self) -> String {
        let data = self.data[0].display(); // WARNING: rn just taking 1 transaction, not acting as Vec
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }

    // FIX: change so it admits really a vec
    fn display_transactions(&self) -> String {
        self.data[0].display()
    }

    // NOTE: the transactions aren't included in the header (tho they are via the merkle root),
    // this guarantees more secure info management
    pub fn display(&self) {
        print!(
            "\nBlock: 
    <Header>
        Index: {}
        Timestamp: {}
        Merkle Root: {}
        Previous hash: {}
        Hash: {}
        Nonce: {}
    <Transactions>
        {}",
            self.index,
            self.timestamp,
            self.merkle_root,
            self.previous_hash,
            self.hash,
            self.nonce,
            self.display_transactions(),
        );
    }
}
