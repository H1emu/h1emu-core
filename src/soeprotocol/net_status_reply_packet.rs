use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

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
