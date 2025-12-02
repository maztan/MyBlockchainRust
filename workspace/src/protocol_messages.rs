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
}

impl Serialize for ProtocolMessage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let serialized = Vec::<u8>::new();
        serialized.push(self.get_type() as u8);

        match self {
            ProtocolMessage::Handshake(m) => {
                serialized.extend(bincode::serialize(m).unwrap());
                return serializer.serialize_bytes(&serialized);
            }
            ProtocolMessage::Block(m) => {
                serialized.extend(bincode::serialize(m).unwrap());
                return serializer.serialize_bytes(&serialized);
            }
            ProtocolMessage::Transaction(m) => {
                serialized.extend(bincode::serialize(m).unwrap());
                return serializer.serialize_bytes(&serialized);
            }
        }
    }
}

struct ProtocolMessageVisitor;

impl Visitor<'_> for ProtocolMessageVisitor {
    type Value = ProtocolMessage;

    fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        formatter.write_str("a byte array representing a ProtocolMessage")
    }

    fn visit_bytes<E>(self, data: &[u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let message_type_raw = data[0];

        match ProtocolMessageType::try_from(message_type_raw) {
            Err(_) => return Err(serde::de::Error::custom("Invalid message type")),
            Ok(message_type) => match message_type {
                ProtocolMessageType::Handshake => {
                    let msg: HandshakeMessage = bincode::deserialize(&data[1..])?;
                    Ok(ProtocolMessage::Handshake(msg))
                }
                ProtocolMessageType::Block => {
                    let msg: BlockMessage = bincode::deserialize(&data[1..])?;
                    Ok(ProtocolMessage::Block(msg))
                }
                ProtocolMessageType::Transaction => {
                    let msg: TransactionMessage = bincode::deserialize(&data[1..])?;
                    Ok(ProtocolMessage::Transaction(msg))
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

#[derive(Deserialize, Serialize)]
pub struct HandshakeMessage {
    pub protocol_version: u8,
}

#[derive(Deserialize, Serialize)]
pub struct BlockMessage {
    pub block_data: Vec<u8>,
}

#[derive(Deserialize, Serialize)]
pub struct TransactionMessage {
    pub transaction_data: Vec<u8>,
}
