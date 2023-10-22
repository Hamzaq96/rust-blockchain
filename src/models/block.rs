use super::blockchain::Blockchain;
use chrono::prelude::*;
use sha2::{Sha256, Digest};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub nonce: u64,
    pub data: String,
    pub prev_hash: String,
    pub hash: String,
}

impl Block {
    pub fn new(index: u64, prev_hash: String) -> Self {
        // Current block to be created.
        let mut block = Block {
            index: 0,
            timestamp: Utc::now().timestamp_millis() as u64,
            nonce: u64::default(),
            data: String::from("New block"),
            prev_hash: String::default(),
            hash: String::default(),
        };
        block
    }

    pub fn mine(&mut self, blockchain: Blockchain) {
        loop {
            if !self.hash.starts_with(&"0".repeat(blockchain.difficulty)) {
                self.nonce += 1;
                self.hash = self.calculate_hash();
            } else {
                break
            }
        }
    }

    pub fn calculate_hash(&self) -> String {
        // In order to calculate hash, we need data, 
        let mut block_data = self.clone();
        block_data.hash = String::default();

        let serialized_block_data = serde_json::to_string(&block_data).unwrap();
        // Calculat the hash
        let mut hasher = Sha256::new();
        hasher.update(serialized_block_data);
        let result = hasher.finalize();
        format!("{:x}", result)
    }
}