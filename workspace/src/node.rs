use serde::{Deserialize, Serialize};
use tokio::{signal, sync::watch, time::{self, Duration}};
use log::{info, warn};
use std::error::Error;

use crate::blockchain::Blockchain;

/// Represents a node in the blockchain network.
#[derive(Debug)]
pub struct Node {
    /// Unique identifier for the node.
    id: String,
    /// A map that associates node IDs with their corresponding addresses.
    peers: std::collections::HashMap<String, String>,

    blockchain: Blockchain,
}

impl Node {
    /// Read-only accessor for the node id.
    pub fn id(&self) -> &str {
        &self.id
    }
}