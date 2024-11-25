use crate::block::Block;

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize, // the target byte slice that the beginning of the hash must match
}

impl Blockchain {
    pub fn new() -> Self {
        let mut chain = Blockchain {
            chain: Vec::new(),
            difficulty: 8, // Difficulty: match the first 8 bytes
        };
        chain.add_genesis_block();
        chain
    }

    // This "genesis block" would be the first in our chain by default always
    fn add_genesis_block(&mut self) {
        // Block constructor: {index, data, prev_hash}
        let genesis_block = Block::new(0, "Genesis Block".to_string(), "0".to_string());
        self.chain.push(genesis_block);
    }

    pub fn add_block(&mut self, data: String) {
        let previous_block = self.chain.last().expect("Genesis didn't happen?");
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

    pub fn display(&self) {
        for block in &self.chain {
            println!("{:?}", block);
        }
    }
}
