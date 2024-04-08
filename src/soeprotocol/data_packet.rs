
use byteorder::{BigEndian, WriteBytesExt};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use super::soeprotocol_functions::write_packet_data;


#[derive(Serialize, Deserialize, Debug, Clone)]
#[wasm_bindgen]
pub struct DataPacket {
    data: Vec<u8>,
    pub opcode: u16,
    pub sequence: u16,
}
#[wasm_bindgen]
impl DataPacket {
    #[wasm_bindgen(constructor)]
    pub fn new(data: Vec<u8>, sequence: u16, opcode: u16) -> Self {
        Self {
            data,
            sequence,
            opcode,
        }
    }
    pub fn get_sequence(&self) -> u16 {
        self.sequence
    }
    #[wasm_bindgen]
    pub fn build(&mut self) -> Vec<u8> {
        let mut wtr: Vec<u8> = vec![];
        wtr.write_u16::<BigEndian>(self.opcode).unwrap_or_default();
        write_packet_data(&mut wtr, self);
        wtr
    }
}
impl DataPacket {
    pub fn get_data(&mut self) -> &mut Vec<u8> {
        &mut self.data
    }
}