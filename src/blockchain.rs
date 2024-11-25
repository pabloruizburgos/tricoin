use crate::block::Block;

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize, // the target byte slice that the beginning of the hash must match
}

impl Blockchain {
    pub fn new() -> Self {
        let mut chain = Blockchain {
            chain: Vec::new(),
            difficulty: 2, // Difficulty: match the first 2 bytes
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
        true
    }

    pub fn display(&self) {
        for block in &self.chain {
            println!("{:?}", block);
        }
    }
}
