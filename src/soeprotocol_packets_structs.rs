use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SessionRequestPacket {
    pub session_id: u32,
    pub protocol_version: u32,
    pub udp_length: u32,
    protocol: String,
}
#[wasm_bindgen]
impl SessionRequestPacket {
    #[wasm_bindgen(constructor)]
    pub fn new(session_id: u32, protocol_version: u32, udp_length: u32, protocol: String) -> Self {
        Self {
            session_id,
            protocol_version,
            udp_length,
            protocol,
        }
    }
    pub fn get_session_id(&self) -> u32 {
        self.session_id
    }
    pub fn get_protocol_version(&self) -> u32 {
        self.protocol_version
    }
    pub fn get_udp_length(&self) -> u32 {
        self.udp_length
    }
    pub fn get_protocol(&self) -> String {
        self.protocol.clone()
    }
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SessionReplyPacket {
    pub session_id: u32,
    pub crc_seed: u32,
    pub crc_length: u8,
    pub encrypt_method: u16,
    pub udp_length: u32,
}
#[wasm_bindgen]
impl SessionReplyPacket {
    #[wasm_bindgen(constructor)]
    pub fn new(
        session_id: u32,
        crc_seed: u32,
        crc_length: u8,
        encrypt_method: u16,
        udp_length: u32,
    ) -> Self {
        Self {
            session_id,
            crc_seed,
            crc_length,
            encrypt_method,
            udp_length,
        }
    }
    pub fn get_session_id(&self) -> u32 {
        self.session_id
    }
    pub fn get_crc_seed(&self) -> u32 {
        self.crc_seed
    }
    pub fn get_crc_length(&self) -> u8 {
        self.crc_length
    }
    pub fn get_encrypt_method(&self) -> u16 {
        self.encrypt_method
    }
    pub fn get_udp_length(&self) -> u32 {
        self.udp_length
    }
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NetStatusReplyPacket {
    pub client_tick_count: u16,
    pub server_tick_count: u32,
    pub client_packet_sent: u64,
    pub client_packet_received: u64,
    pub server_packet_sent: u64,
    pub server_packet_received: u64,
    pub unknown_field: u16,
}
#[wasm_bindgen]
impl NetStatusReplyPacket {
    #[wasm_bindgen(constructor)]
    pub fn new(
        client_tick_count: u16,
        server_tick_count: u32,
        client_packet_sent: u64,
        client_packet_received: u64,
        server_packet_sent: u64,
        server_packet_received: u64,
        unknown_field: u16,
    ) -> Self {
        Self {
            client_tick_count,
            server_tick_count,
            client_packet_sent,
            client_packet_received,
            server_packet_sent,
            server_packet_received,
            unknown_field,
        }
    }
    pub fn get_client_tick_count(&self) -> u16 {
        self.client_tick_count
    }
    pub fn get_server_tick_count(&self) -> u32 {
        self.server_tick_count
    }
    pub fn get_client_packet_sent(&self) -> u64 {
        self.client_packet_sent
    }
    pub fn get_client_packet_received(&self) -> u64 {
        self.client_packet_received
    }
    pub fn get_server_packet_sent(&self) -> u64 {
        self.server_packet_sent
    }
    pub fn get_server_packet_received(&self) -> u64 {
        self.server_packet_received
    }
    pub fn get_unknown_field(&self) -> u16 {
        self.unknown_field
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MultiPackablePacket {
    // should contain all possible field for a multiPackable packet
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<u8>>,
    pub sequence: u16,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[wasm_bindgen]
pub struct DataPacket {
    data: Vec<u8>,
    pub sequence: u16,
}
#[wasm_bindgen]
impl DataPacket {
    #[wasm_bindgen(constructor)]
    pub fn new(data: Vec<u8>, sequence: u16) -> Self {
        Self { data, sequence }
    }
    pub fn get_sequence(&self) -> u16 {
        self.sequence
    }
}
impl DataPacket {
    pub fn get_data(&mut self) -> &mut Vec<u8> {
        &mut self.data
    }
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AckPacket {
    pub sequence: u16,
}
#[wasm_bindgen]
impl AckPacket {
    #[wasm_bindgen(constructor)]
    pub fn new(sequence: u16) -> Self {
        Self { sequence }
    }
    pub fn get_sequence(&self) -> u16 {
        self.sequence
    }
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone)]
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
}
#[wasm_bindgen]
impl NetStatusRequestPacket {
    #[wasm_bindgen(constructor)]
    pub fn new(
        client_tick_count: u16,
        last_client_update: u32,
        average_update: u32,
        shortest_update: u32,
        longest_update: u32,
        last_server_update: u32,
        packets_sent: u64,
        packets_received: u64,
        unknown_field: u16,
    ) -> Self {
        Self {
            client_tick_count,
            last_client_update,
            average_update,
            shortest_update,
            longest_update,
            last_server_update,
            packets_sent,
            packets_received,
            unknown_field,
        }
    }
    pub fn get_client_tick_count(&self) -> u16 {
        self.client_tick_count
    }
    pub fn get_last_client_update(&self) -> u32 {
        self.last_client_update
    }
    pub fn get_average_update(&self) -> u32 {
        self.average_update
    }
    pub fn get_shortest_update(&self) -> u32 {
        self.shortest_update
    }
    pub fn get_longest_update(&self) -> u32 {
        self.longest_update
    }
    pub fn get_last_server_update(&self) -> u32 {
        self.last_server_update
    }
    pub fn get_packets_sent(&self) -> u64 {
        self.packets_sent
    }
    pub fn get_packets_received(&self) -> u64 {
        self.packets_received
    }
    pub fn get_unknown_field(&self) -> u16 {
        self.unknown_field
    }
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubBasePacket {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Vec<u8>>,
}
#[wasm_bindgen]
impl SubBasePacket {
    #[wasm_bindgen(constructor)]
    pub fn new(name: String, sequence: Option<u16>, data: Option<Vec<u8>>) -> Self {
        Self {
            name,
            sequence,
            data,
        }
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn get_sequence(&self) -> Option<u16> {
        self.sequence
    }
}
impl SubBasePacket {
    pub fn get_data(&self) -> &Option<Vec<u8>> {
        &self.data
    }
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubBasePackets {
    sub_packets: Vec<Vec<u8>>,
}
#[wasm_bindgen]
impl SubBasePackets {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            sub_packets: Vec::new(),
        }
    }
}
impl SubBasePackets {
    pub fn add_sub_packet(&mut self, sub_packet: Vec<u8>) {
        self.sub_packets.push(sub_packet);
    }
    pub fn get_sub_packets(&self) -> &Vec<Vec<u8>> {
        &self.sub_packets
    }
}
