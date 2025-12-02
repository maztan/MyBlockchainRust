use serde::{Deserialize, Serialize};
use tokio::{net::TcpListener, signal, sync::watch, time::{self, Duration}};
use futures::StreamExt;
use log::{info, warn};
use std::{error::Error, str::FromStr};

use std::net::{AddrParseError, SocketAddr};
use tokio_util::codec::{Framed, LengthDelimitedCodec};

use tokio::net::TcpStream;
use std::io::{Read, Write};

use crate::blockchain::Blockchain;
use crate::protocol_messages::ProtocolMessage;
use bincode;

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
            network_addr: SocketAddr::from_str("127.0.0.1:1031").unwrap()
        }
    }

    pub async fn start_server(&self) -> Result<(), Box<dyn Error>> {
        // Placeholder for server start logic
        info!("Blockchain node {} server starting...", self.id);

        let listener = TcpListener::bind(self.network_addr).await?;

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

                    // Create a bincode deserializer for the byte slice
                    let mut deserializer = bincode::serde::decode_from_slice::<ProtocolMessage, bincode::config::Configuration>(&msg_bytes, bincode::config::standard());
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

trait MessageSender {
    fn send_message(&mut self, message: &[u8]) -> Result<(), Box<dyn Error>>;
}

