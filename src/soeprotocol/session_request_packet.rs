use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SessionRequestPacket {
    pub session_id: u32,
    pub crc_length: u32,
    pub udp_length: u32,
    protocol: String,
}
#[wasm_bindgen]
impl SessionRequestPacket {
    #[wasm_bindgen(constructor)]
    pub fn new(session_id: u32, crc_length: u32, udp_length: u32, protocol: String) -> Self {
        Self {
            session_id,
            crc_length,
            udp_length,
            protocol,
        }
    }
    pub fn get_session_id(&self) -> u32 {
        self.session_id
    }
    pub fn get_crc_length(&self) -> u32 {
        self.crc_length
    }
    pub fn get_udp_length(&self) -> u32 {
        self.udp_length
    }
    pub fn get_protocol(&self) -> String {
        self.protocol.clone()
    }
}
