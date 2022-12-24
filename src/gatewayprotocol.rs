use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::Cursor;
use wasm_bindgen::prelude::*;

use crate::gatewayprotocol_packets_structs::*;
use crate::lib_utils::read_prefixed_string_le;
use crate::protocol_errors::gen_deserializing_error_json;

#[wasm_bindgen]
pub struct GatewayProtocol {
    wtr: Vec<u8>,
}

// #[wasm_bindgen]
impl GatewayProtocol {
    // #[wasm_bindgen(constructor)]
    pub fn initialize() -> GatewayProtocol {
        return GatewayProtocol { wtr: vec![] };
    }
    pub fn parse(&mut self, data: Vec<u8>) -> String {
        let mut rdr = Cursor::new(&data);
        if data.len() < 2 {
            return format!(r#"{{"name":"Unknown","raw":{:?}}}"#, data);
        }
        let opcode = rdr.read_u16::<BigEndian>().unwrap();

        return match opcode {
            0x01 => self.parse_login_request(rdr),
            0x02 => self.parse_login_reply(rdr),
            0x03 => r#"{"name":"Logout"}"#.to_string(),
            0x04 => r#"{"name":"ForceDisconnect"}"#.to_string(),
            0x05 => self.parse_tunnel_data(rdr),
            0x06 => self.parse_tunnel_data(rdr),
            0x07 => self.parse_channel_is_routable(rdr),
            0x08 => self.parse_channel_is_not_routable(rdr),
            _ => format!(r#"{{"name":"Unknown","raw":{:?}}}"#, data),
        };
    }

    pub fn pack_login_request_packet(
        &mut self,
        character_id: u64,
        ticket: String,
        client_protocol: String,
        client_build: String,
    ) -> Vec<u8> {
        todo!();
    }

    pub fn pack_login_request_object(&mut self, packet: LoginRequestPacket) -> Vec<u8> {
        if packet.error.is_some() {
            return gen_deserializing_error_json();
        }
        self.wtr.clear();
        self.wtr.write_u16::<BigEndian>(0x01).unwrap();
        self.wtr
            .write_u64::<BigEndian>(packet.character_id)
            .unwrap();
        self.wtr
            .write_u32::<BigEndian>(packet.ticket.len() as u32)
            .unwrap();
        // TODO: WIP
        return self.wtr.clone();
    }

    pub fn pack_login_reply_packet(&mut self, logged_in: bool) -> Vec<u8> {
        todo!();
    }
    pub fn pack_tunnel_data_packet(&mut self) -> Vec<u8> {
        todo!();
    }
    pub fn pack_channel_is_routable_packet(&mut self) -> Vec<u8> {
        todo!();
    }
    pub fn pack_channel_is_not_routable_packet(&mut self) -> Vec<u8> {
        todo!();
    }
}

impl GatewayProtocol {
    fn parse_login_request(&mut self, mut rdr: Cursor<&std::vec::Vec<u8>>) -> String {
        let character_id = rdr.read_u64::<BigEndian>().unwrap();
        let rdr_clone = rdr.clone();
        let raw_data = rdr_clone.into_inner();
        let ticket_data_pos = rdr.position();
        let ticket_data_len = rdr.read_u32::<BigEndian>().unwrap();
        let ticket = read_prefixed_string_le(raw_data, ticket_data_pos as usize, ticket_data_len);
        return format!(
            r#"{{"name":"LoginRequest","characterId":{},"ticket":{}"}}"#,
            character_id, ticket
        );
    }
    fn parse_login_reply(&mut self, mut rdr: Cursor<&std::vec::Vec<u8>>) -> String {
        todo!();
    }
    fn parse_tunnel_data(&mut self, mut rdr: Cursor<&std::vec::Vec<u8>>) -> String {
        todo!();
    }
    fn parse_channel_is_routable(&mut self, mut rdr: Cursor<&std::vec::Vec<u8>>) -> String {
        todo!();
    }
    fn parse_channel_is_not_routable(&mut self, mut rdr: Cursor<&std::vec::Vec<u8>>) -> String {
        todo!();
    }
}
