use byteorder::{BigEndian, WriteBytesExt};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use super::lib_utils::u8_from_str_nul_utf8_unchecked;
use super::soeprotocol_functions::write_data_length;

pub trait PacketWrite {
    fn write_to(&self, wtr: &mut Vec<u8>);
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SessionRequestPacket {
    pub session_id: u32,
    pub protocol_version: u32,
    pub udp_length: u32,
    protocol: String,
}
#[wasm_bindgen]
impl SessionRequestPacket {
    #[wasm_bindgen(constructor)]
    pub fn new(session_id: u32, protocol_version: u32, udp_length: u32, protocol: String) -> Self {
        Self {
            session_id,
            protocol_version,
            udp_length,
            protocol,
        }
    }
    pub fn get_protocol(&self) -> String {
        self.protocol.clone()
    }
}
impl PacketWrite for SessionRequestPacket {
    fn write_to(&self, wtr: &mut Vec<u8>) {
        wtr.write_u32::<BigEndian>(self.protocol_version)
            .unwrap_or_default();
        wtr.write_u32::<BigEndian>(self.session_id)
            .unwrap_or_default();
        wtr.write_u32::<BigEndian>(self.udp_length)
            .unwrap_or_default();
        wtr.extend_from_slice(&u8_from_str_nul_utf8_unchecked(&self.protocol));
    }
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SessionReplyPacket {
    pub session_id: u32,
    pub crc_seed: u32,
    pub crc_length: u8,
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
}
impl PacketWrite for SessionReplyPacket {
    fn write_to(&self, wtr: &mut Vec<u8>) {
        wtr.write_u32::<BigEndian>(self.session_id)
            .unwrap_or_default();
        wtr.write_u32::<BigEndian>(self.crc_seed)
            .unwrap_or_default();
        wtr.write_u8(self.crc_length).unwrap_or_default();
        wtr.write_u16::<BigEndian>(self.encrypt_method)
            .unwrap_or_default();
        wtr.write_u32::<BigEndian>(self.udp_length)
            .unwrap_or_default();
        wtr.write_u32::<BigEndian>(3).unwrap_or_default();
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
}
impl PacketWrite for NetStatusReplyPacket {
    fn write_to(&self, wtr: &mut Vec<u8>) {
        wtr.write_u16::<BigEndian>(self.client_tick_count)
            .unwrap_or_default();
        wtr.write_u32::<BigEndian>(self.server_tick_count)
            .unwrap_or_default();
        wtr.write_u64::<BigEndian>(self.client_packet_sent)
            .unwrap_or_default();
        wtr.write_u64::<BigEndian>(self.client_packet_received)
            .unwrap_or_default();
        wtr.write_u64::<BigEndian>(self.server_packet_sent)
            .unwrap_or_default();
        wtr.write_u64::<BigEndian>(self.server_packet_received)
            .unwrap_or_default();
        wtr.write_u16::<BigEndian>(self.unknown_field)
            .unwrap_or_default();
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
    pub sequence: u16,
}
#[wasm_bindgen]
impl DataPacket {
    #[wasm_bindgen(constructor)]
    pub fn new(data: Vec<u8>, sequence: u16) -> Self {
        Self { data, sequence }
    }
}
impl DataPacket {
    pub fn data(&self) -> &[u8] {
        &self.data
    }
    pub fn get_data(&mut self) -> &mut Vec<u8> {
        &mut self.data
    }
}
impl PacketWrite for DataPacket {
    fn write_to(&self, wtr: &mut Vec<u8>) {
        wtr.write_u16::<BigEndian>(self.sequence)
            .unwrap_or_default();
        wtr.extend_from_slice(&self.data);
    }
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AckPacket {
    pub sequence: u16,
}
#[wasm_bindgen]
impl AckPacket {
    #[wasm_bindgen(constructor)]
    pub fn new(sequence: u16) -> Self {
        Self { sequence }
    }
}
impl PacketWrite for AckPacket {
    fn write_to(&self, wtr: &mut Vec<u8>) {
        wtr.write_u16::<BigEndian>(self.sequence)
            .unwrap_or_default();
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
}
impl PacketWrite for NetStatusRequestPacket {
    fn write_to(&self, wtr: &mut Vec<u8>) {
        wtr.write_u16::<BigEndian>(self.client_tick_count)
            .unwrap_or_default();
        wtr.write_u32::<BigEndian>(self.last_client_update)
            .unwrap_or_default();
        wtr.write_u32::<BigEndian>(self.average_update)
            .unwrap_or_default();
        wtr.write_u32::<BigEndian>(self.shortest_update)
            .unwrap_or_default();
        wtr.write_u32::<BigEndian>(self.longest_update)
            .unwrap_or_default();
        wtr.write_u32::<BigEndian>(self.last_server_update)
            .unwrap_or_default();
        wtr.write_u64::<BigEndian>(self.packets_sent)
            .unwrap_or_default();
        wtr.write_u64::<BigEndian>(self.packets_received)
            .unwrap_or_default();
        wtr.write_u16::<BigEndian>(self.unknown_field)
            .unwrap_or_default();
    }
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubBasePacket {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Vec<u8>>,
}
#[wasm_bindgen]
impl SubBasePacket {
    #[wasm_bindgen(constructor)]
    pub fn new(name: String, sequence: Option<u16>, data: Option<Vec<u8>>) -> Self {
        Self {
            name,
            sequence,
            data,
        }
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn get_sequence(&self) -> Option<u16> {
        self.sequence
    }
}
impl SubBasePacket {
    pub fn get_data(&self) -> &Option<Vec<u8>> {
        &self.data
    }
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubBasePackets {
    sub_packets: Vec<Vec<u8>>,
}
#[wasm_bindgen]
impl SubBasePackets {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            sub_packets: Vec::new(),
        }
    }

    pub fn add_sub_packet(&mut self, sub_packet: Vec<u8>) {
        self.sub_packets.push(sub_packet);
    }
}
impl SubBasePackets {
    pub fn get_sub_packets(&self) -> &Vec<Vec<u8>> {
        &self.sub_packets
    }
}
impl PacketWrite for SubBasePackets {
    fn write_to(&self, wtr: &mut Vec<u8>) {
        for packet in &self.sub_packets {
            write_data_length(wtr, packet.len());
            wtr.extend_from_slice(packet);
        }
    }
}
