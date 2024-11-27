use crate::transaction::Transaction;
use crate::utils::current_timestamp;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub data: Vec<Transaction>,
    pub merkle_root: String,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64, // For the PoW, when changed, the hash canges (here it is incremental by 1)
    pub difficulty: usize, // the target byte slice that the beginning of the hash must match
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
            // NOTE: Difficulty should be specifically set for each block
            // but for simplicity, here we assume a difficulty of 5 for every block
            difficulty: 5, // Difficulty: match the first 4 bytes (2 is the minimum assignable)
        };
        block.hash = block.calculate_hash();
        block.merkle_root = block.calculate_merkle_root();
        block
    }

    pub fn calculate_hash(&self) -> String {
        // NOTE: now we rehash the merkle root, without using the data directly. Cool!
        let data = format!(
            "{}{}{}{}{}",
            self.index, self.timestamp, self.merkle_root, self.previous_hash, self.nonce
        );
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }

    pub fn mine_block(&mut self) {
        let target = "0".repeat(self.difficulty); // Target: first difficulty bytes must be 0
        while &self.hash[..self.difficulty] != target {
            self.timestamp = current_timestamp();
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }
        println!("Block mined: {}", self.hash)
    }

    // FIX: this isn't a merkle root... bi-tree!
    /*
     * Calculates the merkle root of the transactions 'data' by taking each transaction as a single
     * string and hashes it, and returns said hash
     */
    fn calculate_merkle_root(&self) -> String {
        let data = self.get_all_transactions_as_string();
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }

    /*
     * Takes each single-string transaction and joins it
     * Returns a long string that contains every transactions info
     */
    fn get_all_transactions_as_string(&self) -> String {
        let mut vector_transactions: Vec<String> = Vec::new();
        for i in 0..self.data.len() {
            vector_transactions.push(self.data[i].get_transaction_as_string());
        }
        vector_transactions.join("")
    }

    fn display_transactions(&self) {
        for transaction in &self.data {
            transaction.display();
        }
    }

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
    <Transactions>",
            self.index, self.timestamp, self.merkle_root, self.previous_hash, self.hash, self.nonce,
        );
        // NOTE: transactions aren't included in the header (they are via the merkle root)
        self.display_transactions();
    }
}
