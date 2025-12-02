mod node;
use crate::node::Node;

mod blockchain;
mod protocol_messages;

#[tokio::main]
async fn main() {
    println!("Starting node...");
    let node = Node::new("test-node".to_string());
    
    if let Err(e) = node.start_server().await {
        eprintln!("Error starting server: {}", e);
    }
}