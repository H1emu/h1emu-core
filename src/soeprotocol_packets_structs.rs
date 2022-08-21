use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone)]
pub struct SessionRequestPacket {
    pub session_id: u32,
    pub crc_length: u32,
    pub udp_length: u32,
    pub protocol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<bool>, // used internnaly to identify deserialization errors
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SessionReplyPacket {
    pub session_id: u32,
    pub crc_seed: u32,
    pub crc_length: u8,
    pub encrypt_method: u16,
    pub udp_length: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<bool>, // used internnaly to identify deserialization errors
}

#[derive(Serialize, Deserialize)]
pub struct NetStatusReplyPacket {
    pub client_tick_count: u16,
    pub server_tick_count: u32,
    pub client_packet_sent: u64,
    pub client_packet_received: u64,
    pub server_packet_sent: u64,
    pub server_packet_received: u64,
    pub unknown_field: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<bool>, // used internnaly to identify deserialization errors
}

#[derive(Serialize, Deserialize)]
pub struct MultiPackablePacket {
    // should contain all possible field for a multiPackable packet
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<u8>>,
    pub sequence: u16,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DataPacket {
    pub data: Vec<u8>,
    pub sequence: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<bool>, // used internnaly to identify deserialization errors
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AckPacket {
    pub sequence: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<bool>, // used internnaly to identify deserialization errors
}

#[derive(Serialize, Deserialize)]
pub struct NetStatusRequestPacket {
    pub client_tick_count: u16,
    pub last_client_update: u32,
    pub average_update: u32,
    pub shortest_update: u32,
    pub longest_update: u32,
    pub last_server_update: u32,
    pub packets_sent: u64,
    pub packets_received: u64,
    pub unknown_field: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<bool>, // used internnaly to identify deserialization errors
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SubBasePacket {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<u8>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<bool>, // used internnaly to identify deserialization errors
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SubBasePackets {
    pub sub_packets: Vec<SubBasePacket>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<bool>, // used internnaly to identify deserialization errors
}
