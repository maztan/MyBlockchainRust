use serde::{Deserialize, Serialize};
use tokio::{net::TcpListener, signal, sync::watch, time::{self, Duration}};
use futures::{StreamExt, SinkExt};
use log::{info, warn};
use std::{error::Error, str::FromStr};

use std::net::{AddrParseError, SocketAddr};
use tokio_util::codec::{Framed, LengthDelimitedCodec};

use tokio::net::TcpStream;
use std::io::{Read, Write};

use crate::blockchain::Blockchain;
use crate::protocol_messages::ProtocolMessage;
use bincode;
use tokio::sync::oneshot;

/// Represents a node in the blockchain network.
#[derive(Debug)]
pub struct Node {
    /// Unique identifier for the node.
    id: String,
    network_addr: SocketAddr,
    /// A map that associates node IDs with their corresponding addresses.
    peers: std::collections::HashMap<String, String>,

    blockchain: Blockchain,
}

impl Node {
    /// Read-only accessor for the node id.
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn new(id: String) -> Self {
        Node {
            id,
            peers: std::collections::HashMap::new(),
            blockchain: Blockchain::new(),
            network_addr: SocketAddr::from_str("127.0.0.1:10311").unwrap()
        }
    }

    pub async fn start_server(&self, ready_tx: Option<oneshot::Sender<()>>) -> Result<(), Box<dyn Error>> {
        // Placeholder for server start logic
        info!("Blockchain node {} server starting...", self.id);

        let listener = TcpListener::bind(self.network_addr).await?;

        // Notify that the server is ready
        if let Some(tx) = ready_tx {
            let _ = tx.send(());
        }

        loop{
            match listener.accept().await {
                Ok((socket, addr)) => {
                    info!("New connection from {}", addr);

                    tokio::spawn(async move {
                        Self::handle_peer_connection(socket).await;
                    });
                },
                Err(e) => {
                    warn!("Failed to accept connection: {}", e);
                }
            }
        }

        Ok(())
    }

    async fn handle_peer_connection(mut stream: TcpStream) {
        //let mut accumulated_data = Vec::<u8>::new();
        //let mut buffer = [0, 512];

        let mut framed = Framed::new(stream, LengthDelimitedCodec::new());

        loop  {
            match framed.next().await {
                Some(Ok(msg_bytes)) => {
                    println!("Received {} bytes of data", msg_bytes.len());
                    if msg_bytes.is_empty() {
                        warn!("Received empty message");
                        continue;
                    }

                    println!("Encoded handshake message (rcv by server): {:?}", msg_bytes.as_ref());

                    // Create a bincode deserializer for the byte slice
                    match bincode::serde::decode_from_slice::<ProtocolMessage, bincode::config::Configuration>(&msg_bytes, bincode::config::standard()){
                        Ok((protocol_msg, _bytes_read)) => {
                            println!("Decoded ProtocolMessage: {:?}", protocol_msg);
                            // Handle the protocol message as needed
                        },
                        Err(e) => {
                            warn!("Failed to decode ProtocolMessage: {}", e);
                        }
                    }
                },
                Some(Err(e)) => {
                    warn!("Failed to read from stream: {}", e);
                    break;
                },
                None => {
                    println!("Connection closed by peer");
                    break;
                }
            }
        } 
    }
}

pub struct NodeClient;

pub trait MessageSender<MessageType> {
    async fn send_message(&self, message: MessageType) -> Result<(), Box<dyn Error>>;
}

impl MessageSender<ProtocolMessage> for NodeClient {
    async fn send_message(&self, message: ProtocolMessage) -> Result<(), Box<dyn Error>> {
        // Placeholder for sending a message to a peer
        //let mut serialized_message = Vec::<u8>::new();

        let serialized_message = bincode::serde::encode_to_vec(message, bincode::config::standard())?;
        
        println!("Message serialized");

        println!("Before connecting to node");
        
        let addr = SocketAddr::from_str("127.0.0.1:10311").unwrap();

        println!("Connecting to {}", addr);
        let stream = tokio::net::TcpStream::connect(addr).await?;
        let mut framed = Framed::new(stream, LengthDelimitedCodec::new());
        framed.send(serialized_message.into()).await?;

        println!("Sent message to {}", addr);
        Ok(())
    }
}

