use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize, de::Visitor};
use bincode;

#[derive(Debug, TryFromPrimitive)]
#[repr(u8)]
pub enum ProtocolMessageType {
    Handshake = 0,
    Block = 1,
    Transaction = 2,
}

//#[repr(u8)]
//#[derive(Deserialize)] // makes sure all the data inside enum variants are deserializable
#[derive(Debug)]
pub enum ProtocolMessage {
    Handshake(HandshakeMessage),
    Block(BlockMessage),
    Transaction(TransactionMessage),
}

impl ProtocolMessage {
    pub fn get_type(&self) -> ProtocolMessageType {
        match self {
            ProtocolMessage::Handshake(_) => ProtocolMessageType::Handshake,
            ProtocolMessage::Block(_) => ProtocolMessageType::Block,
            ProtocolMessage::Transaction(_) => ProtocolMessageType::Transaction,
        }
    }

    fn serialize_message<T,S>(&self, m: &T, serializer: S) -> Result<S::Ok, S::Error>
    where T: Serialize, S: serde::Serializer
    {
        let mut serialized = Vec::<u8>::new();
        serialized.push(self.get_type() as u8);

        let encoded = bincode::serde::encode_to_vec(m, bincode::config::standard()).unwrap();
        serialized.extend_from_slice(&encoded);
        serializer.serialize_bytes(&serialized)
    }
}

impl Serialize for ProtocolMessage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {

        match self {
            ProtocolMessage::Handshake(m) => {
                self.serialize_message(m, serializer) // this has to repeat for each variant, cannot pass self (this would cause infinite recursion, trying to serialize self again to serialize ProtocolMessage again and calling this method again)
            }
            ProtocolMessage::Block(m) => {
                self.serialize_message(m, serializer)
            }
            ProtocolMessage::Transaction(m) => {
                self.serialize_message(m, serializer)
            }
        }
    }
}

struct ProtocolMessageVisitor;

impl ProtocolMessageVisitor {
    fn standard_decode_from_slice<D>(&self, data: &[u8]) -> Result<(D, usize), bincode::error::DecodeError>
        where D: serde::de::DeserializeOwned
    { 
        bincode::serde::decode_from_slice::<D, bincode::config::Configuration>(&data, bincode::config::standard())
    }

    fn deserialize_message<M>(&self, data: &[u8]) -> Result<(M, usize), bincode::error::DecodeError>
        where M: serde::de::DeserializeOwned
    {
        let msg_result : Result<(M, usize), bincode::error::DecodeError>
                        = self.standard_decode_from_slice(data);
                    
        match msg_result {
            Ok((msg, bytes_read)) => Ok((msg, bytes_read)),

            Err(e) => return Err(e),
        }
    }
}

impl Visitor<'_> for ProtocolMessageVisitor {
    type Value = ProtocolMessage;

    fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        formatter.write_str("a byte array representing a ProtocolMessage")
    }

    fn visit_bytes<E>(self, data: &[u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error
    {
        let message_type_raw = data[0];

        match ProtocolMessageType::try_from(message_type_raw) {
            Err(_) => return Err(serde::de::Error::custom("Invalid message type")),
            Ok(message_type) => match message_type {
                ProtocolMessageType::Handshake => {
                    match self.deserialize_message(&data[1..]){
                        Ok(msg_result) => return Ok(ProtocolMessage::Handshake(msg_result.0)),
                        Err(e) => return Err(serde::de::Error::custom(format!("Failed to deserialize HandshakeMessage: {}", e))),
                    }
                }
                ProtocolMessageType::Block => {
                    match self.deserialize_message(&data[1..]){
                        Ok(msg_result) => return Ok(ProtocolMessage::Block(msg_result.0)),
                        Err(e) => return Err(serde::de::Error::custom(format!("Failed to deserialize HandshakeMessage: {}", e))),
                    }
                }
                ProtocolMessageType::Transaction => {
                    match self.deserialize_message(&data[1..]){
                        Ok(msg_result) => return Ok(ProtocolMessage::Transaction(msg_result.0)),
                        Err(e) => return Err(serde::de::Error::custom(format!("Failed to deserialize HandshakeMessage: {}", e))),
                    }
                }
            },
        }
    }
}

impl<'de> Deserialize<'de> for ProtocolMessage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
                deserializer.deserialize_bytes(ProtocolMessageVisitor)
    }
}

// impl TryFrom<&[u8]> for ProtocolMessage {
//     type Error = bincode::Error;

//     fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        
//     }
// }

#[derive(Deserialize, Serialize, Debug)]
pub struct HandshakeMessage {
    pub node_id: String,
    pub protocol_version: u8,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct BlockMessage {
    pub block_data: Vec<u8>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TransactionMessage {
    pub transaction_data: Vec<u8>,
}
