use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};




#[derive(Debug)]
pub struct Blockchain{
    blocks: Vec<Block>,
    digest_hex_len: usize,
}

impl Blockchain{

    pub fn new() -> Self{
        let digest_hex_len = Sha256::output_size() * 2;
        let genesis = Block::new(0,"genesis".to_string(), "0".repeat(digest_hex_len));

        Blockchain{
            blocks: vec![genesis],
            digest_hex_len
        }
    }

    pub fn add_block(&mut self, block: Block) -> Result<(), &'static str>{
        if self.verify_new_block(&block){
            self.blocks.push(block);
            Ok(())
        }else{
            Err("Invalid block")
        }
    }

    fn verify_new_block(&self, block: &Block) -> bool {
        let last_block = self.blocks.last().unwrap();

        if block.previous_hash != last_block.hash {
            return false;
        }

        true
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    index: u64,
    data: String,
    previous_hash: String,
    hash: String,
    timestamp: i64
}

impl Block {
    fn new(index: u64, data: String, previous_hash: String) -> Self {
        let timestamp = chrono::Utc::now().timestamp();
        let hash = Block::calculate_hash(index, &data, &previous_hash, timestamp);

        Block {
            index,
            data,
            previous_hash,
            hash,
            timestamp
        }
    }

    fn calculate_hash(index: u64, data: &str, previous_hash: &str, timestamp: i64) -> String {
        let raw = format!("{index}{data}{previous_hash}{timestamp}");
        let hash = format!("{:x}",Sha256::digest(raw.as_bytes()));
        hash
    }
}