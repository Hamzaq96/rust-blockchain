use chrono::prelude::*;
// Internal module
use super::block::Block;

type Blocks = Vec<Block>;

#[derive(Debug, Clone)]
pub struct Blockchain {
    pub genesis_block: Block,
    pub chain: Blocks,
    pub difficulty: usize,
}

impl Blockchain {
    pub fn new(difficulty: usize) -> Self {
        // First block in the chain
        let mut genesis_block = Block {
            index: 0,
            timestamp: Utc::now().timestamp_millis() as u64,
            nonce: 2836,
            data: String::from("Genesis Block"),
            prev_hash: String::default(),
            hash: String::default(),
        };

        let mut chain = Vec::new();
        chain.push(genesis_block.clone());

        let blockchain = Blockchain {
            genesis_block,
            chain,
            difficulty,
        };

        blockchain
    }

    pub fn add_block(&mut self) {
        let mut new_block = Block::new(
            self.chain.len() as u64,
            self.chain[&self.chain.len() - 1].prev_hash.clone(),
        );

        new_block.mine(self.clone());
        self.chain.push(new_block.clone());
        println!("New block added to the chain: {:?}", new_block);
        println!("Blockchain: {:?}", self)
    }

}
