

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DisconnectPacket {
    pub session_id: u32,
    reason: String,
}
#[wasm_bindgen]
impl DisconnectPacket {
    #[wasm_bindgen(constructor)]
    pub fn new(session_id: u32, reason: String) -> Self {
        Self { session_id, reason }
    }
    pub fn get_reason(&self) -> String {
        self.reason.clone()
    }
}