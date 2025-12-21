mod node;

use node::Node;
use node::MessageSender;
use node::NodeClient;

use log::{info, warn, error, debug};
use env_logger;

mod blockchain;
mod protocol_messages;
use protocol_messages::ProtocolMessage;
use tokio::net::unix::pipe::Receiver;
use tokio::sync::oneshot;
use tokio::task::JoinHandle;

use rand::{distributions::Alphanumeric, Rng};
use tokio::task::JoinSet;

#[tokio::main]
async fn main() {
    env_logger::init();
    
    let node_id = random_node_id();
    let (server_handle, server_ready_rx) = spawn_node(node_id.clone());

    // Wait for the server to be ready
    let _ = server_ready_rx.await;
    info!("Server ready.");

    let client = NodeClient;

    debug!("Creating handshake message...");
    // Send a handshake
    let handshake_msg = ProtocolMessage::Handshake (
        protocol_messages::HandshakeMessage {
            node_id: node_id.clone(),
            protocol_version: 1,
    });

    if let Err(e) = client.send_message(handshake_msg).await{
        error!("Error sending handshake: {}", e);
    }
    
    server_handle.await.unwrap();
}

fn random_node_id() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect()
}

fn spawn_nodes(count: usize) -> JoinSet<()> {
    let mut node_handles_set = JoinSet::new();

    for i in 0..count {
        node_handles_set.spawn(async move {
            let node_id = format!("node_{}", i);
            info!("Starting node \"{}\"...", node_id);

            let node = Node::new(node_id.clone());
            let (server_ready_tx, server_ready_rx) = oneshot::channel();

            if let Err(e) = node.start_server(Some(server_ready_tx)).await {
                eprintln!("Error starting node: {}", e);
            }

            // Wait for the server to be ready
            let _ = server_ready_rx.await;
            info!("Node \"{}\" server ready.", node_id);
        });
    }

    node_handles_set
}

fn spawn_node(node_id: String) -> (JoinHandle<()>, oneshot::Receiver<()>) {
    info!("Starting node \"{}\"...", node_id);

    let node = Node::new(node_id.clone());
    let (server_ready_tx, server_ready_rx) = oneshot::channel();

    let server_handle = tokio::spawn(async move {
        if let Err(e) = node.start_server(Some(server_ready_tx)).await {
            eprintln!("Error starting node: {}", e);
        }
    });

    (server_handle, server_ready_rx)
}
