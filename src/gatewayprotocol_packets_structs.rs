use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone)]
pub struct LoginRequestPacket {
    pub character_id: u64,
    pub ticket: String,
    pub client_protocol: String,
    pub client_build: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<bool>, // used internnaly to identify deserialization errors
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LoginReplyPacket {
    pub logged_in: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<bool>, // used internnaly to identify deserialization errors
}

#[derive(Serialize, Deserialize)]
pub struct ChannelIsRoutablePacket {
    pub is_routable: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<bool>, // used internnaly to identify deserialization errors
}

#[derive(Serialize)]
pub struct TunnelPacket {
    pub name: &'static str,
    pub flags: u8,
    pub tunnel_data: Vec<u8>,
}
