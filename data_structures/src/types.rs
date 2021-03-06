use std::fmt;

use crate::{
    chain::{Block, CheckpointBeacon, Hashable, InventoryEntry},
    proto::{schema::witnet, ProtobufConvert},
    transaction::Transaction,
};

/// Witnet's protocol messages
#[derive(Debug, Eq, PartialEq, Clone, ProtobufConvert)]
#[protobuf_convert(pb = "witnet::Message")]
pub struct Message {
    pub kind: Command,
    pub magic: u16,
}

/// Commands for the Witnet's protocol messages
#[derive(Debug, Eq, PartialEq, Clone, ProtobufConvert)]
#[protobuf_convert(pb = "witnet::Message_Command")]
// FIXME(#649): Remove clippy skip error
#[allow(clippy::large_enum_variant)]
pub enum Command {
    // Peer discovery messages
    GetPeers(GetPeers),
    Peers(Peers),

    // Handshake messages
    Verack(Verack),
    Version(Version),

    // Inventory messages
    Block(Block),
    Transaction(Transaction),
    InventoryAnnouncement(InventoryAnnouncement),
    InventoryRequest(InventoryRequest),
    LastBeacon(LastBeacon),
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Command::GetPeers(_) => f.write_str(&"GET_PEERS".to_string()),
            Command::Peers(_) => f.write_str(&"PEERS".to_string()),
            Command::Verack(_) => f.write_str(&"VERACK".to_string()),
            Command::Version(_) => f.write_str(&"VERSION".to_string()),
            Command::Block(block) => f.write_str(&format!("BLOCK: {}", block.hash())),
            Command::InventoryAnnouncement(_) => f.write_str(&"INVENTORY_ANNOUNCEMENT".to_string()),
            Command::InventoryRequest(_) => f.write_str(&"INVENTORY_REQUEST".to_string()),
            Command::LastBeacon(_) => f.write_str(&"LAST_BEACON".to_string()),
            Command::Transaction(tx) => match tx {
                Transaction::Commit(_) => f.write_str(&"COMMIT_TRANSACTION".to_string()),
                Transaction::ValueTransfer(_) => {
                    f.write_str(&"VALUE_TRANSFER_TRANSACTION".to_string())
                }
                Transaction::DataRequest(_) => f.write_str(&"DATA_REQUEST_TRANSACTION".to_string()),
                Transaction::Reveal(_) => f.write_str(&"REVEAL_TRANSACTION".to_string()),
                Transaction::Tally(_) => f.write_str(&"TALLY_TRANSACTION".to_string()),
                Transaction::Mint(_) => f.write_str(&"MINT_TRANSACTION".to_string()),
            },
        }
    }
}

///////////////////////////////////////////////////////////
// PEER DISCOVERY MESSAGES
///////////////////////////////////////////////////////////
#[derive(Debug, Eq, PartialEq, Clone, ProtobufConvert)]
#[protobuf_convert(pb = "witnet::GetPeers")]
pub struct GetPeers;

#[derive(Debug, Eq, PartialEq, Clone, ProtobufConvert)]
#[protobuf_convert(pb = "witnet::Peers")]
pub struct Peers {
    pub peers: Vec<Address>,
}

///////////////////////////////////////////////////////////
// HANDSHAKE MESSAGES
///////////////////////////////////////////////////////////
#[derive(Debug, Eq, PartialEq, Clone, ProtobufConvert)]
#[protobuf_convert(pb = "witnet::Verack")]
pub struct Verack;

#[derive(Debug, Eq, PartialEq, Clone, ProtobufConvert)]
#[protobuf_convert(pb = "witnet::Version")]
pub struct Version {
    pub version: u32,
    pub timestamp: i64,
    pub capabilities: u64,
    pub sender_address: Address,
    pub receiver_address: Address,
    pub user_agent: String,
    pub last_epoch: u32,
    pub nonce: u64,
}

///////////////////////////////////////////////////////////
// INVENTORY MESSAGES
///////////////////////////////////////////////////////////
#[derive(Debug, Eq, PartialEq, Clone, ProtobufConvert)]
#[protobuf_convert(pb = "witnet::InventoryAnnouncement")]
pub struct InventoryAnnouncement {
    pub inventory: Vec<InventoryEntry>,
}

#[derive(Debug, Eq, PartialEq, Clone, ProtobufConvert)]
#[protobuf_convert(pb = "witnet::InventoryRequest")]
pub struct InventoryRequest {
    pub inventory: Vec<InventoryEntry>,
}

#[derive(Debug, Eq, PartialEq, Clone, ProtobufConvert)]
#[protobuf_convert(pb = "witnet::LastBeacon")]
pub struct LastBeacon {
    pub highest_block_checkpoint: CheckpointBeacon,
}

///////////////////////////////////////////////////////////
// AUX TYPES
///////////////////////////////////////////////////////////
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum IpAddress {
    Ipv4 {
        ip: u32,
    },
    Ipv6 {
        ip0: u32,
        ip1: u32,
        ip2: u32,
        ip3: u32,
    },
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Address {
    pub ip: IpAddress,
    pub port: u16,
}
