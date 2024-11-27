use crate::block::Block;
use crate::transaction::Transaction;
use crate::utils::current_timestamp;

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize, // the target byte slice that the beginning of the hash must match
}

impl Blockchain {
    pub fn new() -> Self {
        let mut chain = Blockchain {
            chain: Vec::new(),
            difficulty: 5, // Difficulty: match the first 4 bytes (2 is the minimum assignable)
        };
        chain.add_genesis_block();
        chain
    }

    // This "genesis block" would be the first in our chain by default always
    fn add_genesis_block(&mut self) {
        // Block constructor: {index, data, prev_hash}
        let genesis_transaction = Transaction::new(
            "".to_string(),
            "".to_string(),
            50.00,
            0.00,
            "The Times 26/Nov/2024 Chancellor on brink of second bailout for banks".to_string(),
        );
        let mut genesis_block = Block::new(0, vec![genesis_transaction], "0".to_string());
        genesis_block.timestamp = current_timestamp();
        self.chain.push(genesis_block);
    }

    pub fn add_block(&mut self, data: Vec<Transaction>) {
        let previous_block = self.chain.last().expect("Genesis didn't happen");
        let mut new_block = Block::new(previous_block.index + 1, data, previous_block.hash.clone());
        new_block.mine_block(self.difficulty);
        self.chain.push(new_block);
    }

    pub fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            // We start iterating from the second block ([1]) as the genesis block doesn't need to be checked
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];

            if current_block.hash != current_block.calculate_hash() {
                return false;
            }

            if current_block.previous_hash != previous_block.hash {
                return false;
            }
        }
        true
    }

    // WARNING: this function is for debug purposes only
    pub fn total_time_mining(&self) -> u64 {
        self.chain[self.chain.len() - 1].timestamp - self.chain[0].timestamp
    }

    pub fn display(&self) {
        for block in &self.chain {
            block.display();
        }
        println!("\nTotal mining time: {:?}s", self.total_time_mining()); // delete when due
    }
}
