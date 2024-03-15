use crate::soeprotocol_functions::write_packet_data;

use super::soeprotocol::SoeOpcode;
use byteorder::{BigEndian, WriteBytesExt};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

// #[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SoePacket {
    SessionRequestPacket(SessionRequestPacket),
    SessionReplyPacket(SessionReplyPacket),
    NetStatusReplyPacket(NetStatusReplyPacket),
    MultiPackablePacket(MultiPackablePacket),
    DataPacket(DataPacket),
    AckPacket(AckPacket),
    NetStatusRequestPacket(NetStatusRequestPacket),
    GroupedPackets(GroupedPackets),
    PingPacket(PingPacket),
    FatalErrorPacket(FatalErrorPacket),
    DisconnectPacket(DisconnectPacket),
    UnknownPacket(UnknownPacket),
}
#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SoePacketParsed {
    pub opcode: SoeOpcode,
    packet: SoePacket,
}
#[wasm_bindgen]
impl SoePacketParsed {
    pub fn get_opcode(&self) -> SoeOpcode {
        self.opcode
    }
    pub fn get_session_request_packet(&self) -> Option<SessionRequestPacket> {
        match &self.packet {
            SoePacket::SessionRequestPacket(packet) => Some(packet.clone()),
            _ => None,
        }
    }
    pub fn get_session_reply_packet(&self) -> Option<SessionReplyPacket> {
        match &self.packet {
            SoePacket::SessionReplyPacket(packet) => Some(packet.clone()),
            _ => None,
        }
    }
    pub fn get_net_status_reply_packet(&self) -> Option<NetStatusReplyPacket> {
        match &self.packet {
            SoePacket::NetStatusReplyPacket(packet) => Some(packet.clone()),
            _ => None,
        }
    }
    pub fn get_data_packet(&self) -> Option<DataPacket> {
        match &self.packet {
            SoePacket::DataPacket(packet) => Some(packet.clone()),
            _ => None,
        }
    }
    pub fn get_ack_packet(&self) -> Option<AckPacket> {
        match &self.packet {
            SoePacket::AckPacket(packet) => Some(packet.clone()),
            _ => None,
        }
    }
    pub fn get_net_status_request_packet(&self) -> Option<NetStatusRequestPacket> {
        match &self.packet {
            SoePacket::NetStatusRequestPacket(packet) => Some(packet.clone()),
            _ => None,
        }
    }
    pub fn get_grouped_packets(&self) -> Option<GroupedPackets> {
        match &self.packet {
            SoePacket::GroupedPackets(packet) => Some(packet.clone()),
            _ => None,
        }
    }
    pub fn get_unknown_packet(&self) -> Option<UnknownPacket> {
        match &self.packet {
            SoePacket::UnknownPacket(packet) => Some(packet.clone()),
            _ => None,
        }
    }
}
impl SoePacketParsed {
    pub fn new(opcode: SoeOpcode, packet: SoePacket) -> Self {
        Self { opcode, packet }
    }
    pub fn get_packet(&self) -> SoePacket {
        self.packet.clone()
    }
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UnknownPacket {}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PingPacket {}
#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FatalErrorPacket {}
#[wasm_bindgen]
impl FatalErrorPacket {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {}
    }
}

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
#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SessionRequestPacket {
    pub session_id: u32,
    pub crc_length: u32,
    pub udp_length: u32,
    protocol: String,
}
#[wasm_bindgen]
impl SessionRequestPacket {
    #[wasm_bindgen(constructor)]
    pub fn new(session_id: u32, crc_length: u32, udp_length: u32, protocol: String) -> Self {
        Self {
            session_id,
            crc_length,
            udp_length,
            protocol,
        }
    }
    pub fn get_session_id(&self) -> u32 {
        self.session_id
    }
    pub fn get_crc_length(&self) -> u32 {
        self.crc_length
    }
    pub fn get_udp_length(&self) -> u32 {
        self.udp_length
    }
    pub fn get_protocol(&self) -> String {
        self.protocol.clone()
    }
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SessionReplyPacket {
    pub session_id: u32,
    pub crc_seed: u32,
    pub crc_length: u8,
    // TODO: use the EncryptionMethod enum
    pub encrypt_method: u16,
    pub udp_length: u32,
}
#[wasm_bindgen]
impl SessionReplyPacket {
    #[wasm_bindgen(constructor)]
    pub fn new(
        session_id: u32,
        crc_seed: u32,
        crc_length: u8,
        encrypt_method: u16,
        udp_length: u32,
    ) -> Self {
        Self {
            session_id,
            crc_seed,
            crc_length,
            encrypt_method,
            udp_length,
        }
    }
    pub fn get_session_id(&self) -> u32 {
        self.session_id
    }
    pub fn get_crc_seed(&self) -> u32 {
        self.crc_seed
    }
    pub fn get_crc_length(&self) -> u8 {
        self.crc_length
    }
    pub fn get_encrypt_method(&self) -> u16 {
        self.encrypt_method
    }
    pub fn get_udp_length(&self) -> u32 {
        self.udp_length
    }
}

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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MultiPackablePacket {
    // should contain all possible field for a multiPackable packet
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<u8>>,
    pub sequence: u16,
}

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

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NetStatusRequestPacket {
    pub client_tick_count: u16,
    pub last_client_update: u32,
    pub average_update: u32,
    pub shortest_update: u32,
    pub longest_update: u32,
    pub last_server_update: u32,
    pub packets_sent: u64,
    pub packets_received: u64,
    pub unknown_field: u16,
}
#[wasm_bindgen]
impl NetStatusRequestPacket {
    #[wasm_bindgen(constructor)]
    pub fn new(
        client_tick_count: u16,
        last_client_update: u32,
        average_update: u32,
        shortest_update: u32,
        longest_update: u32,
        last_server_update: u32,
        packets_sent: u64,
        packets_received: u64,
        unknown_field: u16,
    ) -> Self {
        Self {
            client_tick_count,
            last_client_update,
            average_update,
            shortest_update,
            longest_update,
            last_server_update,
            packets_sent,
            packets_received,
            unknown_field,
        }
    }
    pub fn get_client_tick_count(&self) -> u16 {
        self.client_tick_count
    }
    pub fn get_last_client_update(&self) -> u32 {
        self.last_client_update
    }
    pub fn get_average_update(&self) -> u32 {
        self.average_update
    }
    pub fn get_shortest_update(&self) -> u32 {
        self.shortest_update
    }
    pub fn get_longest_update(&self) -> u32 {
        self.longest_update
    }
    pub fn get_last_server_update(&self) -> u32 {
        self.last_server_update
    }
    pub fn get_packets_sent(&self) -> u64 {
        self.packets_sent
    }
    pub fn get_packets_received(&self) -> u64 {
        self.packets_received
    }
    pub fn get_unknown_field(&self) -> u16 {
        self.unknown_field
    }
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GroupedPackets {
    sub_packets: Vec<SoePacket>,
}
#[wasm_bindgen]
impl GroupedPackets {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            sub_packets: Vec::new(),
        }
    }
}
impl GroupedPackets {
    pub fn add_packet(&mut self, packet: SoePacket) {
        self.sub_packets.push(packet);
    }
    pub fn get_packets(&self) -> &Vec<SoePacket> {
        &self.sub_packets
    }
}
