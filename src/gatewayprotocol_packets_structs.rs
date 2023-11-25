use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone)]
pub struct LoginRequestPacket {
    pub character_id: u64,
    pub ticket: String,
    pub client_protocol: String,
    pub client_build: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LoginReplyPacket {
    pub logged_in: bool,
}

#[derive(Serialize, Deserialize)]
pub struct ChannelIsRoutablePacket {
    pub is_routable: bool,
}

#[derive(Serialize)]
// Internal
pub struct TunnelPacket {
    pub name: &'static str,
    pub channel: u8,
    pub tunnel_data: Vec<u8>,
}
