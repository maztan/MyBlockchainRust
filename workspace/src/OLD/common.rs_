use serde::{Serialize, Deserialize};
use chrono::Utc;
use sha2::{Sha256, Digest};
use std::fmt;
use log::{info, warn, error, debug};

#[derive(Serialize, Deserialize)]
pub struct Block {
    index: u64,
    data: String,
    previous_hash: String,
    hash: String,
    timestamp: i64
}

impl Block {
    fn new(index: u64, data: String, previous_hash: String, difficulty: usize) -> Self {
        let timestamp = Utc::now().timestamp();
        let mut nounce = 0;

        loop{
            let raw = format!("{index}{data}{previous_hash}{timestamp}{nounce}");
            let hash = format!("{:x}",Sha256::digest(raw.as_bytes()));
            
            if hash.starts_with(&"0".repeat(difficulty)) {
                return Block{
                    index,
                    data,
                    previous_hash,
                    hash,
                    timestamp
                };
            }

            nounce += 1;
        }
    }
}

impl fmt::Debug for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Block {}, hash: {}", self.index, self.hash)
    }
}


#[derive(Debug)]
struct Blockchain{
    blocks: Vec<Block>,
    difficulty: usize
}

impl Blockchain {
    fn new(difficulty: usize) -> Self {
        let digest_hex_len = Sha256::output_size() * 2;
        let genesis = Block::new(0, "genesis".into(), "0".repeat(digest_hex_len), difficulty);
        Blockchain {blocks: vec![genesis], difficulty}
    }

    async fn add_block(&mut self, data: String){

        let index = self.blocks.len() as u64;
        let previous_hash = self.blocks.last().unwrap().hash.clone();
        let difficulty = self.difficulty;

        let new_block = tokio::task::spawn_blocking(move || {
            Block::new(index,data,previous_hash,difficulty)
        }).await.unwrap();

        self.blocks.push(new_block);
        info!("Mined new block: {:#?}", self.blocks.last().unwrap());
    }
}