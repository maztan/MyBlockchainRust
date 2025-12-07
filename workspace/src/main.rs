mod node;
use node::Node;
use node::MessageSender;
use node::NodeClient;

mod blockchain;
mod protocol_messages;
use protocol_messages::ProtocolMessage;
use tokio::sync::oneshot;

#[tokio::main]
async fn main() {
    println!("Starting node...");

    let node_id = "test-node".to_string();
    let node = Node::new(node_id.clone());

    let (server_ready_tx, server_ready_rx) = oneshot::channel();

    let server_handle = tokio::spawn(async move {
        if let Err(e) = node.start_server(Some(server_ready_tx)).await {
            eprintln!("Error starting server: {}", e);
        }
    });

    // Wait for the server to be ready
    let _ = server_ready_rx.await;
    println!("Server ready.");

    let client = NodeClient;

    println!("Creating handshake message...");
    // Send a handshake
    let handshake_msg = ProtocolMessage::Handshake (
        protocol_messages::HandshakeMessage {
            node_id: node_id.clone(),
            protocol_version: 1,
    });

    let handshake_msg2 = ProtocolMessage::Handshake (
        protocol_messages::HandshakeMessage {
            node_id: node_id.clone(),
            protocol_version: 1,
    });
    let encoded = bincode::serde::encode_to_vec(handshake_msg2, bincode::config::standard()).unwrap();
    println!("Encoded handshake message: {:?}", encoded);

    if let Err(e) = client.send_message(handshake_msg).await{
        eprintln!("Error sending handshake: {}", e);
    }
    
    server_handle.await.unwrap();
}
