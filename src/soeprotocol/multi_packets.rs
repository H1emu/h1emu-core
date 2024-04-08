use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use super::{
    soeprotocol_functions::{extract_subpacket_data, get_data_end, read_data_length},
    soeprotocol_packets_structs::SoePacket,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MultiPackablePacket {
    // should contain all possible field for a multiPackable packet
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<u8>>,
    pub sequence: u16,
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GroupedPackets {
    sub_packets: Vec<SoePacket>,
}
#[wasm_bindgen]
impl GroupedPackets {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            sub_packets: Vec::new(),
        }
    }
}
impl GroupedPackets {
    pub fn add_packet(&mut self, packet: SoePacket) {
        self.sub_packets.push(packet);
    }
    pub fn get_packets(&self) -> &Vec<SoePacket> {
        &self.sub_packets
    }
}
