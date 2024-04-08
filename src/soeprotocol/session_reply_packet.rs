use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SessionReplyPacket {
    pub session_id: u32,
    pub crc_seed: u32,
    pub crc_length: u8,
    // TODO: use the EncryptionMethod enum
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
