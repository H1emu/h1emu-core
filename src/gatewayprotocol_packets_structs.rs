use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use super::gatewayprotocol::GatewayOpcode;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum GatewayPacket {
    LoginRequest(LoginRequestPacket),
    LoginReply(LoginReplyPacket),
    ChannelIsRoutable(ChannelIsRoutablePacket),
    Tunnel(TunnelPacket),
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GatewayPacketParsed {
    opcode: GatewayOpcode,
    packet: GatewayPacket,
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
}
#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginReplyPacket {
    pub logged_in: bool,
}
#[wasm_bindgen]
impl LoginReplyPacket {
    pub fn new(logged_in: bool) -> LoginReplyPacket {
        LoginReplyPacket { logged_in }
    }
    pub fn get_logged_in(&self) -> bool {
        self.logged_in
    }
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChannelIsRoutablePacket {
    pub is_routable: bool,
}
#[wasm_bindgen]
impl ChannelIsRoutablePacket {
    pub fn new(is_routable: bool) -> ChannelIsRoutablePacket {
        ChannelIsRoutablePacket { is_routable }
    }
    pub fn get_is_routable(&self) -> bool {
        self.is_routable
    }
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone)]
// Internal
pub struct TunnelPacket {
    pub channel: u8,
    tunnel_data: Vec<u8>,
}
#[wasm_bindgen]
impl TunnelPacket {
    pub fn new(channel: u8, tunnel_data: Vec<u8>) -> TunnelPacket {
        TunnelPacket {
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
}
