use std::net::{AddrParseError, SocketAddr};
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use serde::{Serialize, Deserialize};
use serde_json;

use crate::common::{self, Block};

pub struct NetworkNode {
    id: u32,
    address: SocketAddr,
}

impl NetworkNode {
    const SIZE_FIELD_LEN_BYTES:usize = std::mem::size_of::<u32>();

    fn new(id: u32, address: String) -> Result<Self, AddrParseError>
    {
        let address = address.parse::<SocketAddr>()?; // "?" will return Err if parsing fails
        Ok(NetworkNode { id, address})
    }

    fn start_server(&self) {
        // Placeholder for server start logic
        println!("Starting server for node {} at {}", self.id, self.address);
        let listener = TcpListener::bind(self.address).unwrap();

        for stream in listener.incoming(){
            match stream {
                Ok(mut stream) =>{
                    println!("New connection: {}", stream.peer_addr().unwrap());
                    self.handle_peer_connection(stream);
                }
                Err(e) => {
                    eprintln!("Connection failed: {}", e);
                }
            }
        }
    }

    fn handle_peer_connection(&self, mut stream: TcpStream) {
        let mut accumulated_data = Vec::<u8>::new();
        let mut buffer = [0; 512];

        loop {
            match stream.read(&mut buffer) {
                Ok(0) =>{
                    println!("Connection closed by peer");
                }
                Ok(n) => {
                    println!("Received {} bytes: {:?}", n, &buffer[..n]);
                    accumulated_data.extend_from_slice(&buffer[..n]);
                    if(accumulated_data.len() < Self::SIZE_FIELD_LEN_BYTES){
                        continue;
                    }

                    let msg_size_bytes = &accumulated_data[..Self::SIZE_FIELD_LEN_BYTES];
                    let msg_bytes = &accumulated_data[Self::SIZE_FIELD_LEN_BYTES..];

                    match serde_json::from_slice::<Block>(&msg_bytes) {
                        Ok(msg) => {
                            accumulated_data.clear();
                        }
                        Err(e) => {
                            eprintln!("Failed to deserialize message: {}", e);
                        }            
                    };
                }
                Err(e) => {
                    eprintln!("Failed to read from connection: {}", e);
                }
            }
        }
    }
}

struct GossipTestMessage {
    sender_id: u32,
    content: String,
}