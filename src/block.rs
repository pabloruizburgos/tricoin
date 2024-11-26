use crate::utils::current_timestamp;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub data: String,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64, // For the PoW, when changed, the hash canges (here it is incremental by 1)
}

impl Block {
    pub fn new(index: u64, data: String, previous_hash: String) -> Self {
        let mut block = Block {
            index,
            timestamp: 0, // Time set when the block is mined
            data,
            previous_hash,
            hash: String::new(),
            nonce: 0,
        };
        block.hash = block.calculate_hash();
        block
    }

    pub fn calculate_hash(&self) -> String {
        let data = format!(
            "{}{}{}{}{}",
            self.index, self.timestamp, self.data, self.previous_hash, self.nonce
        );
        let mut hasher = Sha256::new();
        hasher.update(data);
        //println!("Calculated hash: {:x}", hasher.finalize());
        format!("{:x}", hasher.finalize())
    }

    pub fn mine_block(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty); // Target: first difficulty bytes must be 0
        while &self.hash[..difficulty] != target {
            self.nonce += 1;
            self.timestamp = current_timestamp();
            self.hash = self.calculate_hash();
        }
        println!("Block mined: {}", self.hash);
    }
}
