use byteorder::{BigEndian, WriteBytesExt};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AckPacket {
    pub opcode: u16,
    pub sequence: u16,
}
#[wasm_bindgen]
impl AckPacket {
    #[wasm_bindgen(constructor)]
    pub fn new(opcode: u16, sequence: u16) -> Self {
        Self { opcode, sequence }
    }
    pub fn get_sequence(&self) -> u16 {
        self.sequence
    }

    #[wasm_bindgen]
    pub fn build(&self) -> Vec<u8> {
        let mut wtr: Vec<u8> = vec![];
        wtr.write_u16::<BigEndian>(self.opcode).unwrap_or_default();
        wtr.write_u16::<BigEndian>(self.sequence)
            .unwrap_or_default();
        wtr
    }
}