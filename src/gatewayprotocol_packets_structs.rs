use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use super::lib_utils::read_prefixed_string_le;

use super::gatewayprotocol::GatewayOpcode;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::Cursor;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum GatewayPacket {
    LoginRequest(LoginRequestPacket),
    LoginReply(LoginReplyPacket),
    ChannelIsRoutable(ChannelIsRoutablePacket),
    ChannelIsNotRoutable(ChannelIsNotRoutablePacket),
    Tunnel(TunnelPacket),
    Unknown(UnknownGatewayPacket),
    Logout(LogoutPacket),
    ForceDisconnect(ForceDisconnectPacket),
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GatewayPacketParsed {
    opcode: GatewayOpcode,
    packet: GatewayPacket,
}
#[wasm_bindgen]
impl GatewayPacketParsed {
    pub fn get_opcode(&self) -> GatewayOpcode {
        self.opcode
    }
    pub fn get_login_request_packet(&self) -> Option<LoginRequestPacket> {
        match &self.packet {
            GatewayPacket::LoginRequest(packet) => Some(packet.clone()),
            _ => None,
        }
    }
    pub fn get_login_reply_packet(&self) -> Option<LoginReplyPacket> {
        match &self.packet {
            GatewayPacket::LoginReply(packet) => Some(packet.clone()),
            _ => None,
        }
    }
    pub fn get_channel_is_routable_packet(&self) -> Option<ChannelIsRoutablePacket> {
        match &self.packet {
            GatewayPacket::ChannelIsRoutable(packet) => Some(packet.clone()),
            _ => None,
        }
    }
    pub fn get_channel_is_not_routable_packet(&self) -> Option<ChannelIsNotRoutablePacket> {
        match &self.packet {
            GatewayPacket::ChannelIsNotRoutable(packet) => Some(packet.clone()),
            _ => None,
        }
    }
    pub fn get_tunnel_packet(&self) -> Option<TunnelPacket> {
        match &self.packet {
            GatewayPacket::Tunnel(packet) => Some(packet.clone()),
            _ => None,
        }
    }
    pub fn get_unknown_packet(&self) -> Option<UnknownGatewayPacket> {
        match &self.packet {
            GatewayPacket::Unknown(packet) => Some(packet.clone()),
            _ => None,
        }
    }
    pub fn get_logout_packet(&self) -> Option<LogoutPacket> {
        match &self.packet {
            GatewayPacket::Logout(packet) => Some(packet.clone()),
            _ => None,
        }
    }
    pub fn get_force_disconnect_packet(&self) -> Option<ForceDisconnectPacket> {
        match &self.packet {
            GatewayPacket::ForceDisconnect(packet) => Some(packet.clone()),
            _ => None,
        }
    }
}
impl GatewayPacketParsed {
    pub fn new(opcode: GatewayOpcode, packet: GatewayPacket) -> Self {
        Self { opcode, packet }
    }
    pub fn get_packet(&self) -> GatewayPacket {
        self.packet.clone()
    }
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginRequestPacket {
    pub character_id: u64,
    ticket: String,
    client_protocol: String,
    client_build: String,
}
#[wasm_bindgen]
impl LoginRequestPacket {
    #[wasm_bindgen(constructor)]
    pub fn new(
        character_id: u64,
        ticket: String,
        client_protocol: String,
        client_build: String,
    ) -> LoginRequestPacket {
        LoginRequestPacket {
            character_id,
            ticket,
            client_protocol,
            client_build,
        }
    }
    pub fn get_character_id(&self) -> u64 {
        self.character_id
    }
    pub fn get_ticket(&self) -> String {
        self.ticket.clone()
    }
    pub fn get_client_protocol(&self) -> String {
        self.client_protocol.clone()
    }
    pub fn get_client_build(&self) -> String {
        self.client_build.clone()
    }
    pub fn build(&self) -> Vec<u8> {
        let mut wtr: Vec<u8> = vec![];
        wtr.write_u8(GatewayOpcode::LoginRequest as u8)
            .unwrap_or_default();
        wtr.write_u64::<LittleEndian>(self.character_id)
            .unwrap_or_default();
        let ticket = self.get_ticket();
        wtr.write_u32::<LittleEndian>(ticket.len() as u32)
            .unwrap_or_default();
        wtr.append(&mut ticket.as_bytes().to_vec());
        let client_protocol = self.get_client_protocol();
        wtr.write_u32::<LittleEndian>(client_protocol.len() as u32)
            .unwrap_or_default();
        wtr.append(&mut client_protocol.as_bytes().to_vec());
        let client_build = self.get_client_build();
        wtr.write_u32::<LittleEndian>(client_build.len() as u32)
            .unwrap_or_default();
        wtr.append(&mut client_build.as_bytes().to_vec());
        wtr
    }
}
impl LoginRequestPacket {
    pub fn from(mut rdr: Cursor<&std::vec::Vec<u8>>) -> LoginRequestPacket {
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
        LoginRequestPacket {
            character_id,
            ticket,
            client_protocol,
            client_build,
        }
    }
}
#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginReplyPacket {
    pub logged_in: bool,
}
#[wasm_bindgen]
impl LoginReplyPacket {
    #[wasm_bindgen(constructor)]
    pub fn new(logged_in: bool) -> LoginReplyPacket {
        LoginReplyPacket { logged_in }
    }
    pub fn get_logged_in(&self) -> bool {
        self.logged_in
    }
    pub fn build(&self) -> Vec<u8> {
        let mut wtr: Vec<u8> = vec![];
        wtr.write_u8(GatewayOpcode::LoginReply as u8)
            .unwrap_or_default();
        wtr.write_u8(self.logged_in as u8).unwrap_or_default();
        wtr
    }
}
impl LoginReplyPacket {
    pub fn from(mut rdr: Cursor<&std::vec::Vec<u8>>) -> LoginReplyPacket {
        let logged_in: bool = rdr.read_u8().unwrap_or_default() != 0; // convert to bool
        LoginReplyPacket { logged_in }
    }
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChannelIsRoutablePacket {
    pub is_routable: bool,
}
#[wasm_bindgen]
impl ChannelIsRoutablePacket {
    #[wasm_bindgen(constructor)]
    pub fn new(is_routable: bool) -> ChannelIsRoutablePacket {
        ChannelIsRoutablePacket { is_routable }
    }
    pub fn get_is_routable(&self) -> bool {
        self.is_routable
    }
    pub fn build(&self) -> Vec<u8> {
        let mut wtr: Vec<u8> = vec![];
        wtr.write_u8(GatewayOpcode::ChannelIsRoutable as u8)
            .unwrap_or_default();
        wtr
    }
}

impl ChannelIsRoutablePacket {
    pub fn from(mut rdr: Cursor<&std::vec::Vec<u8>>) -> ChannelIsRoutablePacket {
        let is_routable = rdr.read_u8().unwrap_or_default() == 1;
        ChannelIsRoutablePacket { is_routable }
    }
}
#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone)]
// Internal
pub struct TunnelPacket {
    pub channel: u8,
    pub opcode: GatewayOpcode,
    tunnel_data: Vec<u8>,
}
#[wasm_bindgen]
impl TunnelPacket {
    #[wasm_bindgen(constructor)]
    pub fn new(opcode: GatewayOpcode, channel: u8, tunnel_data: Vec<u8>) -> TunnelPacket {
        TunnelPacket {
            opcode,
            channel,
            tunnel_data,
        }
    }
    pub fn get_channel(&self) -> u8 {
        self.channel
    }
    pub fn get_tunnel_data(&self) -> Vec<u8> {
        self.tunnel_data.clone()
    }
    pub fn build(&mut self) -> Vec<u8> {
        let mut wtr: Vec<u8> = vec![];
        wtr.write_u8(self.opcode as u8).unwrap_or_default();
        wtr.append(&mut self.tunnel_data);
        wtr
    }
}
impl TunnelPacket {
    pub fn from(mut data: std::vec::Vec<u8>) -> TunnelPacket {
        let channel = data.remove(0) >> 5;
        let tunnel_data = data;
        // FIXME: opcode
        TunnelPacket {
            opcode: GatewayOpcode::TunnelDataServer,
            channel,
            tunnel_data,
        }
    }
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChannelIsNotRoutablePacket {}
impl ChannelIsNotRoutablePacket {
    pub fn from(mut _rdr: Cursor<&std::vec::Vec<u8>>) -> ChannelIsNotRoutablePacket {
        ChannelIsNotRoutablePacket {}
    }
}
#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LogoutPacket {}
impl LogoutPacket {
    pub fn from(mut _rdr: Cursor<&std::vec::Vec<u8>>) -> LogoutPacket {
        LogoutPacket {}
    }
}
#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ForceDisconnectPacket {}
impl ForceDisconnectPacket {
    pub fn from(mut _rdr: Cursor<&std::vec::Vec<u8>>) -> ForceDisconnectPacket {
        ForceDisconnectPacket {}
    }
}
#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UnknownGatewayPacket {}
impl UnknownGatewayPacket {
    pub fn from(mut _rdr: Cursor<&std::vec::Vec<u8>>) -> UnknownGatewayPacket {
        UnknownGatewayPacket {}
    }
}
