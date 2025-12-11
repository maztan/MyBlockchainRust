use chrono::Utc;
use sha2::{Sha256, Digest};
use tokio::sync::mpsc;
use log::{info, warn, error, debug};
use std::fmt;

mod gossip_node;
mod common;
use gossip_node::NetworkNode;
use common::Block;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    println!("Welcome to the blockchain!");
    let mut blockchain = Blockchain::new(1)  ;
    //blockchain.add_block("data 1".to_string());
    //print!("{:#?}", blockchain);

    // this is our "mempool" queue
    let (tx, mut rx) = mpsc::channel::<String>(32);

    let miner_handle = tokio::spawn(async move{
        // there is single receiver for this channel, so we can assume we "own" the blockchanin
        // and don't need to synchronize access to len()

        while let Some(str_data) = rx.recv().await {
            //spawn dedicated thread for mining, so we don't block the async runtime

            blockchain.add_block(str_data).await;
        }
    });

    let sender = tx.clone();
    tokio::spawn(async move {
        let _ = sender.send("some string 1".to_string()).await;
        let _ = sender.send("some string 2".to_string()).await;
        let _ = sender.send("some string 3".to_string()).await;
    });

    miner_handle.await.unwrap();
}


