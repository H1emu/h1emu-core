use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use std::io::Cursor;
use wasm_bindgen::prelude::*;

use super::gatewayprotocol_packets_structs::*;
use super::lib_utils::read_prefixed_string_le;

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum GatewayOpcode {
    LoginRequest = 0x01,
    LoginReply = 0x02,
    Logout = 0x03,
    ForceDisconnect = 0x04,
    TunnelDataClient = 0x05,
    TunnelDataServer = 0x06,
    ChannelIsRoutable = 0x07,
    ChannelIsNotRoutable = 0x08,
}
#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum GatewayChannels {
    Zone = 0,
    World = 1,
    UpdatePosition = 2,
    ShortCircuitZone = 3,
    Gateway = 4,
}

#[wasm_bindgen]
pub struct GatewayProtocol {
    wtr: Vec<u8>,
}

#[wasm_bindgen]
impl GatewayProtocol {
    #[wasm_bindgen(constructor)]
    pub fn initialize() -> GatewayProtocol {
        GatewayProtocol { wtr: vec![] }
    }
    pub fn parse(&mut self, data: Vec<u8>) -> String {
        let mut rdr = Cursor::new(&data);
        if data.len() < 2 {
            return format!(r#"{{"name":"Unknown","raw":{:?}}}"#, data);
        }
        let full_opcode = rdr.read_u8().unwrap_or_default();
        let opcode = full_opcode & 0x1f;
        let channel = full_opcode >> 5;

        match opcode {
            0x01 => self.parse_login_request(rdr),
            0x02 => self.parse_login_reply(rdr),
            0x03 => r#"{"name":"Logout"}"#.to_string(),
            0x04 => r#"{"name":"ForceDisconnect"}"#.to_string(),
            0x05 => self.parse_tunnel_data(data),
            0x06 => self.parse_tunnel_data(data),
            0x07 => self.parse_channel_is_routable(rdr),
            0x08 => self.parse_channel_is_not_routable(rdr),
            _ => format!(
                r#"{{"name":"Unknown","channel":{},"raw":{:?}}}"#,
                channel, data
            ),
        }
    }

    pub fn pack_login_request_packet(
        &mut self,
        character_id: u64,
        ticket: String,
        client_protocol: String,
        client_build: String,
    ) -> Vec<u8> {
        self.pack_login_request_object(LoginRequestPacket::new(
            character_id,
            ticket,
            client_protocol,
            client_build,
        ))
    }
    pub fn pack_login_reply_packet(&mut self, logged_in: bool) -> Vec<u8> {
        self.pack_login_reply_object(LoginReplyPacket { logged_in })
    }
    pub fn pack_tunnel_data_packet_for_client(&mut self, data: Vec<u8>, channel: u8) -> Vec<u8> {
        self._pack_tunnel_data_packet(0x05, data, channel)
    }
    pub fn pack_tunnel_data_packet_for_server(&mut self, data: Vec<u8>, channel: u8) -> Vec<u8> {
        self._pack_tunnel_data_packet(0x06, data, channel)
    }
    fn _pack_tunnel_data_packet(
        &mut self,
        base_opcode: u8,
        mut data: Vec<u8>,
        channel: u8,
    ) -> Vec<u8> {
        let opcode = base_opcode | channel << 5;
        self.wtr.clear();
        self.wtr.write_u8(opcode).unwrap_or_default();
        self.wtr.append(&mut data);
        self.wtr.clone()
    }
    pub fn pack_channel_is_routable_packet(&mut self) -> Vec<u8> {
        let opcode = 0x07;
        self.wtr.clear();
        self.wtr.write_u8(opcode).unwrap_or_default();
        self.wtr.clone()
    }
    pub fn pack_channel_is_not_routable_packet(&mut self) -> Vec<u8> {
        let opcode = 0x08;
        self.wtr.clear();
        self.wtr.write_u8(opcode).unwrap_or_default();
        self.wtr.clone()
    }
}

impl GatewayProtocol {
    fn parse_login_request(&mut self, mut rdr: Cursor<&std::vec::Vec<u8>>) -> String {
        let character_id = rdr.read_u64::<LittleEndian>().unwrap_or_default();
        let raw_data = rdr.clone().into_inner();
        let ticket_data_pos = rdr.position();
        let ticket_data_len = rdr.read_u32::<LittleEndian>().unwrap_or_default();
        let ticket = read_prefixed_string_le(raw_data, ticket_data_pos as usize, ticket_data_len);
        rdr.set_position(ticket_data_pos + ticket_data_len as u64 + 4);
        let client_protocol_data_pos = rdr.position();
        let client_protocol_data_len = rdr.read_u32::<LittleEndian>().unwrap_or_default();
        let client_protocol = read_prefixed_string_le(
            raw_data,
            client_protocol_data_pos as usize,
            client_protocol_data_len,
        );
        rdr.set_position(client_protocol_data_pos + client_protocol_data_len as u64 + 4);
        let client_build_data_pos = rdr.position();
        let client_build_data_len = rdr.read_u32::<LittleEndian>().unwrap_or_default();
        let client_build = read_prefixed_string_le(
            raw_data,
            client_build_data_pos as usize,
            client_build_data_len,
        );
        format!(
            r#"{{"name":"LoginRequest","character_id":"0x{:x}","ticket":"{}","client_protocol":"{}","client_build":"{}"}}"#,
            character_id, ticket, client_protocol, client_build
        )
    }
    fn parse_login_reply(&mut self, mut rdr: Cursor<&std::vec::Vec<u8>>) -> String {
        let logged_in: bool = rdr.read_u8().unwrap_or_default() != 0; // convert to bool
        format!(r#"{{"name":"LoginReply","logged_in":{}}}"#, logged_in)
    }
    fn parse_tunnel_data(&mut self, mut data: std::vec::Vec<u8>) -> String {
        let channel = data.remove(0) >> 5;
        let tunnel_data = data;
        let packet = TunnelPacket::new(channel, tunnel_data);
        serde_json::to_string(&packet).unwrap_or_default()
    }
    fn parse_channel_is_routable(&mut self, mut _rdr: Cursor<&std::vec::Vec<u8>>) -> String {
        format!(r#"{{"name":"ChannelIsRoutable","raw":"{:?}"}}"#, _rdr)
    }
    fn parse_channel_is_not_routable(&mut self, mut _rdr: Cursor<&std::vec::Vec<u8>>) -> String {
        format!(r#"{{"name":"ChannelIsNotRoutable","raw":"{:?}"}}"#, _rdr)
    }

    pub fn pack_login_request_object(&mut self, packet: LoginRequestPacket) -> Vec<u8> {
        self.wtr.clear();
        self.wtr.write_u8(0x01).unwrap_or_default();
        self.wtr
            .write_u64::<LittleEndian>(packet.character_id)
            .unwrap_or_default();
        let ticket = packet.get_ticket();
        self.wtr
            .write_u32::<LittleEndian>(ticket.len() as u32)
            .unwrap_or_default();
        self.wtr.append(&mut ticket.as_bytes().to_vec());
        let client_protocol = packet.get_client_protocol();
        self.wtr
            .write_u32::<LittleEndian>(client_protocol.len() as u32)
            .unwrap_or_default();
        self.wtr.append(&mut client_protocol.as_bytes().to_vec());
        let client_build = packet.get_client_build();
        self.wtr
            .write_u32::<LittleEndian>(client_build.len() as u32)
            .unwrap_or_default();
        self.wtr.append(&mut client_build.as_bytes().to_vec());
        self.wtr.clone()
    }

    pub fn pack_login_reply_object(&mut self, packet: LoginReplyPacket) -> Vec<u8> {
        self.wtr.clear();
        self.wtr.write_u8(0x02).unwrap_or_default();
        self.wtr
            .write_u8(packet.logged_in as u8)
            .unwrap_or_default();
        self.wtr.clone()
    }
}
