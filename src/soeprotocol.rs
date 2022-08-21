use super::soeprotocol_functions::*;
use super::{
    crc::{append_crc, crc32},
    lib_utils::{str_from_u8_nul_utf8_unchecked, u8_from_str_nul_utf8_unchecked},
    soeprotocol_packets_structs::*,
};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use serde_json::*;
use std::io::Cursor;
use wasm_bindgen::prelude::*;

pub struct CachedPacket {
    parsed: String,
    packed: Vec<u8>,
}

pub struct CachedPackets {
    ping: CachedPacket,
}

#[wasm_bindgen]
pub struct Soeprotocol {
    use_crc: bool,
    crc_seed: u32,
    cached_packets: CachedPackets,
}

#[wasm_bindgen]
pub enum EncryptMethod {
    EncryptMethodNone = 0x0,
    EncryptMethodUserSupplied = 0x1,
    EncryptMethodUserSupplied2 = 0x2,
    EncryptMethodXorBuffer = 0x3,
    EncryptMethodXor = 0x4,
}

impl Soeprotocol {
    // rust only

    pub fn get_session_request_object(&mut self, packet_string: String) -> SessionRequestPacket {
        return serde_json::from_str(&packet_string).unwrap_or_else(|_| {
            return SessionRequestPacket {
                session_id: 0,
                crc_length: 0,
                udp_length: 0,
                protocol: "".to_string(),
                error: Some(true),
            };
        });
    }

    pub fn pack_session_request_object(&mut self, packet: SessionRequestPacket) -> Vec<u8> {
        if packet.error.is_some() {
            return gen_deserializing_error_json();
        }
        let mut wtr = vec![];
        wtr.write_u16::<BigEndian>(0x01).unwrap();
        wtr.write_u32::<BigEndian>(packet.crc_length).unwrap();
        wtr.write_u32::<BigEndian>(packet.session_id).unwrap();
        wtr.write_u32::<BigEndian>(packet.udp_length).unwrap();
        wtr.append(&mut u8_from_str_nul_utf8_unchecked(
            packet.protocol.as_str(),
        ));
        return wtr;
    }

    pub fn get_session_reply_object(&mut self, packet_string: String) -> SessionReplyPacket {
        return serde_json::from_str(&packet_string).unwrap_or_else(|_| {
            return SessionReplyPacket {
                session_id: 0,
                crc_seed: 0,
                crc_length: 0,
                encrypt_method: 0,
                udp_length: 0,
                error: Some(true),
            };
        });
    }

    pub fn pack_session_reply_object(&mut self, packet: SessionReplyPacket) -> Vec<u8> {
        if packet.error.is_some() {
            return gen_deserializing_error_json();
        }
        let mut wtr = vec![];
        wtr.write_u16::<BigEndian>(0x02).unwrap();
        wtr.write_u32::<BigEndian>(packet.session_id).unwrap();
        wtr.write_u32::<BigEndian>(packet.crc_seed).unwrap();
        wtr.write_u8(packet.crc_length).unwrap();
        wtr.write_u16::<BigEndian>(packet.encrypt_method).unwrap();
        wtr.write_u32::<BigEndian>(packet.udp_length).unwrap();
        wtr.write_u32::<BigEndian>(3).unwrap();
        return wtr;
    }

    pub fn get_net_status_request_object(
        &mut self,
        packet_string: String,
    ) -> NetStatusRequestPacket {
        return serde_json::from_str(&packet_string).unwrap_or_else(|_| {
            return NetStatusRequestPacket {
                client_tick_count: 0,
                last_client_update: 0,
                average_update: 0,
                shortest_update: 0,
                longest_update: 0,
                last_server_update: 0,
                packets_sent: 0,
                packets_received: 0,
                unknown_field: 0,
                error: Some(true),
            };
        });
    }

    pub fn pack_net_status_request_object(&mut self, packet: NetStatusRequestPacket) -> Vec<u8> {
        if packet.error.is_some() {
            return gen_deserializing_error_json();
        }
        let mut wtr = vec![];
        wtr.write_u16::<BigEndian>(0x07).unwrap();
        wtr.write_u16::<BigEndian>(packet.client_tick_count)
            .unwrap();
        wtr.write_u32::<BigEndian>(packet.last_client_update)
            .unwrap();
        wtr.write_u32::<BigEndian>(packet.average_update).unwrap();
        wtr.write_u32::<BigEndian>(packet.shortest_update).unwrap();
        wtr.write_u32::<BigEndian>(packet.longest_update).unwrap();
        wtr.write_u32::<BigEndian>(packet.last_server_update)
            .unwrap();
        wtr.write_u64::<BigEndian>(packet.packets_sent).unwrap();
        wtr.write_u64::<BigEndian>(packet.packets_received).unwrap();
        wtr.write_u16::<BigEndian>(packet.unknown_field).unwrap();
        return wtr;
    }

    pub fn get_net_status_reply_object(&mut self, packet_string: String) -> NetStatusReplyPacket {
        return serde_json::from_str(&packet_string).unwrap_or_else(|_| {
            return NetStatusReplyPacket {
                client_tick_count: 0,
                server_tick_count: 0,
                client_packet_sent: 0,
                client_packet_received: 0,
                server_packet_sent: 0,
                server_packet_received: 0,
                unknown_field: 0,
                error: Some(true),
            };
        });
    }

    pub fn pack_net_status_reply_object(&mut self, packet: NetStatusReplyPacket) -> Vec<u8> {
        if packet.error.is_some() {
            return gen_deserializing_error_json();
        }
        let mut wtr = vec![];
        wtr.write_u16::<BigEndian>(0x08).unwrap();
        wtr.write_u16::<BigEndian>(packet.client_tick_count)
            .unwrap();
        wtr.write_u32::<BigEndian>(packet.server_tick_count)
            .unwrap();
        wtr.write_u64::<BigEndian>(packet.client_packet_sent)
            .unwrap();
        wtr.write_u64::<BigEndian>(packet.client_packet_received)
            .unwrap();
        wtr.write_u64::<BigEndian>(packet.server_packet_sent)
            .unwrap();
        wtr.write_u64::<BigEndian>(packet.server_packet_received)
            .unwrap();
        wtr.write_u16::<BigEndian>(packet.unknown_field).unwrap();
        return wtr;
    }

    pub fn get_multi_object(&mut self, packet_string: String) -> SubBasePackets {
        return serde_json::from_str(&packet_string).unwrap_or_else(|_| {
            return SubBasePackets {
                sub_packets: vec![],
                error: Some(true),
            };
        });
    }

    pub fn pack_multi_object(&mut self, multi_packet: SubBasePackets) -> Vec<u8> {
        if multi_packet.error.is_some() {
            return gen_deserializing_error_json();
        }
        let mut wtr = vec![];
        wtr.write_u16::<BigEndian>(0x03).unwrap();
        let was_crc_enabled = self.is_using_crc();
        if was_crc_enabled {
            self.disable_crc();
        }
        for packet in multi_packet.sub_packets {
            let packet_object = serde_json::to_string(&packet).unwrap();
            let mut packet_data = self.pack(packet.name, packet_object);
            write_data_length(&mut wtr, packet_data.len());
            wtr.append(&mut packet_data);
        }
        if was_crc_enabled {
            self.enable_crc();
            append_crc(&mut wtr, self.get_crc_seed())
        }
        return wtr;
    }

    pub fn get_data_object(&mut self, packet_string: String) -> DataPacket {
        return serde_json::from_str(&packet_string).unwrap_or_else(|_| {
            return DataPacket {
                data: vec![],
                sequence: 0,
                error: Some(true),
            };
        });
    }

    fn _pack_data_object(&mut self, opcode: u16, mut packet: DataPacket) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u16::<BigEndian>(opcode).unwrap();
        write_packet_data(&mut wtr, &mut packet, self.crc_seed, self.use_crc);
        return wtr;
    }

    pub fn pack_data_object(&mut self, packet: DataPacket) -> Vec<u8> {
        if packet.error.is_some() {
            return gen_deserializing_error_json();
        }
        return self._pack_data_object(0x09, packet);
    }

    pub fn pack_fragment_data_object(&mut self, packet: DataPacket) -> Vec<u8> {
        if packet.error.is_some() {
            return gen_deserializing_error_json();
        }
        return self._pack_data_object(0x0d, packet);
    }

    pub fn get_ack_object(&mut self, packet_string: String) -> AckPacket {
        return serde_json::from_str(&packet_string).unwrap_or_else(|_| {
            return AckPacket {
                sequence: 0,
                error: Some(true),
            };
        });
    }

    fn _pack_ack_object(&mut self, opcode: u16, sequence: u16) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u16::<BigEndian>(opcode).unwrap();
        wtr.write_u16::<BigEndian>(sequence).unwrap();
        if self.use_crc {
            append_crc(&mut wtr, self.crc_seed);
        }
        return wtr;
    }

    pub fn pack_out_of_order_object(&mut self, packet: AckPacket) -> Vec<u8> {
        if packet.error.is_some() {
            return gen_deserializing_error_json();
        }

        return self._pack_ack_object(0x11, packet.sequence);
    }

    pub fn pack_ack_object(&mut self, packet: AckPacket) -> Vec<u8> {
        if packet.error.is_some() {
            return gen_deserializing_error_json();
        }
        return self._pack_ack_object(0x15, packet.sequence);
    }
}
#[wasm_bindgen]
impl Soeprotocol {
    // wasm lib
    #[wasm_bindgen(constructor)]
    pub fn initialize(use_crc: bool, crc_seed: u32) -> Soeprotocol {
        let ping_packet = CachedPacket {
            parsed: r#"{"name":"Ping"}"#.to_string(),
            packed: vec![0, 6],
        };
        let cached_packets = CachedPackets { ping: ping_packet };
        return Soeprotocol {
            use_crc,
            crc_seed,
            cached_packets,
        };
    }
    pub fn pack(&mut self, packet_name: String, packet: String) -> Vec<u8> {
        match packet_name.as_str() {
            "SessionRequest" => return self.pack_session_request(packet),
            "SessionReply" => return self.pack_session_reply(packet),
            "MultiPacket" => return self.pack_multi(packet),
            "Disconnect" => return vec![0, 5],
            "Ping" => self.cached_packets.ping.packed.to_owned(),
            "NetStatusRequest" => return self.pack_net_status_request(packet),
            "NetStatusReply" => return self.pack_net_status_reply(packet),
            "Data" => return self.pack_data(packet),
            "DataFragment" => return self.pack_fragment_data(packet),
            "OutOfOrder" => return self.pack_out_of_order(packet),
            "Ack" => return self.pack_ack(packet),
            _ => return vec![],
        }
    }

    pub fn pack_session_request(&mut self, packet: String) -> Vec<u8> {
        let packet_object: SessionRequestPacket = self.get_session_request_object(packet);
        return self.pack_session_request_object(packet_object);
    }

    pub fn pack_session_request_fromjs(&mut self, js_object: &JsValue) -> Vec<u8> {
        let packet: SessionRequestPacket = js_object.into_serde().unwrap();
        return self.pack_session_request_object(packet);
    }

    pub fn pack_session_request_packet(
        &mut self,
        session_id: u32,
        crc_length: u32,
        udp_length: u32,
        protocol: String,
    ) -> Vec<u8> {
        return self.pack_session_request_object(SessionRequestPacket {
            session_id,
            crc_length,
            udp_length,
            protocol,
            error: None,
        });
    }

    pub fn pack_session_reply(&mut self, packet: String) -> Vec<u8> {
        let packet_object: SessionReplyPacket = self.get_session_reply_object(packet);
        return self.pack_session_reply_object(packet_object);
    }

    pub fn pack_session_reply_fromjs(&mut self, js_object: &JsValue) -> Vec<u8> {
        let packet: SessionReplyPacket = js_object.into_serde().unwrap();
        return self.pack_session_reply_object(packet);
    }

    pub fn pack_session_reply_packet(
        &mut self,
        session_id: u32,
        crc_seed: u32,
        crc_length: u8,
        encrypt_method: u16,
        udp_length: u32,
    ) -> Vec<u8> {
        return self.pack_session_reply_object(SessionReplyPacket {
            session_id,
            crc_seed,
            crc_length,
            encrypt_method,
            udp_length,
            error: None,
        });
    }

    pub fn pack_net_status_request(&mut self, packet: String) -> Vec<u8> {
        let packet_object: NetStatusRequestPacket = self.get_net_status_request_object(packet);
        return self.pack_net_status_request_object(packet_object);
    }

    pub fn pack_net_status_request_fromjs(&mut self, js_object: &JsValue) -> Vec<u8> {
        let packet: NetStatusRequestPacket = js_object.into_serde().unwrap();
        return self.pack_net_status_request_object(packet);
    }

    pub fn pack_net_status_reply(&mut self, packet: String) -> Vec<u8> {
        let packet_object: NetStatusReplyPacket = self.get_net_status_reply_object(packet);
        return self.pack_net_status_reply_object(packet_object);
    }

    pub fn pack_net_status_reply_fromjs(&mut self, js_object: &JsValue) -> Vec<u8> {
        let packet: NetStatusReplyPacket = js_object.into_serde().unwrap();
        return self.pack_net_status_reply_object(packet);
    }

    pub fn pack_multi(&mut self, packet: String) -> Vec<u8> {
        let multi_packets: SubBasePackets = self.get_multi_object(packet);
        return self.pack_multi_object(multi_packets);
    }

    pub fn pack_multi_fromjs(&mut self, js_object: &JsValue) -> Vec<u8> {
        let packet: SubBasePackets = js_object.into_serde().unwrap();
        return self.pack_multi_object(packet);
    }

    pub fn pack_data(&mut self, packet: String) -> Vec<u8> {
        let packet_object: DataPacket = self.get_data_object(packet);
        return self.pack_data_object(packet_object);
    }

    pub fn pack_data_fromjs(&mut self, js_object: &JsValue) -> Vec<u8> {
        let packet: DataPacket = js_object.into_serde().unwrap();
        return self.pack_data_object(packet);
    }

    pub fn pack_data_packet(&mut self, data: Vec<u8>, sequence: u16) -> Vec<u8> {
        return self.pack_data_object(DataPacket {
            data,
            sequence,
            error: None,
        });
    }

    pub fn pack_fragment_data(&mut self, packet: String) -> Vec<u8> {
        let packet_object: DataPacket = self.get_data_object(packet);
        return self.pack_fragment_data_object(packet_object);
    }

    pub fn pack_fragment_data_fromjs(&mut self, js_object: &JsValue) -> Vec<u8> {
        let packet: DataPacket = js_object.into_serde().unwrap();
        return self.pack_fragment_data_object(packet);
    }

    pub fn pack_fragment_data_packet(&mut self, data: Vec<u8>, sequence: u16) -> Vec<u8> {
        return self.pack_fragment_data_object(DataPacket {
            data,
            sequence,
            error: None,
        });
    }

    pub fn pack_out_of_order(&mut self, packet: String) -> Vec<u8> {
        let packet_object: AckPacket = self.get_ack_object(packet);
        return self.pack_out_of_order_object(packet_object);
    }

    pub fn pack_out_of_order_fromjs(&mut self, js_object: &JsValue) -> Vec<u8> {
        let packet: AckPacket = js_object.into_serde().unwrap();
        return self.pack_out_of_order_object(packet);
    }

    pub fn pack_out_of_order_packet(&mut self, sequence: u16) -> Vec<u8> {
        return self.pack_out_of_order_object(AckPacket {
            sequence,
            error: None,
        });
    }

    pub fn pack_ack(&mut self, packet: String) -> Vec<u8> {
        let packet_object: AckPacket = self.get_ack_object(packet);
        return self.pack_ack_object(packet_object);
    }

    pub fn pack_ack_fromjs(&mut self, js_object: &JsValue) -> Vec<u8> {
        let packet: AckPacket = js_object.into_serde().unwrap();
        return self.pack_ack_object(packet);
    }

    pub fn pack_ack_packet(&mut self, sequence: u16) -> Vec<u8> {
        return self.pack_ack_object(AckPacket {
            sequence,
            error: None,
        });
    }

    pub fn parse(&mut self, data: Vec<u8>) -> String {
        let mut rdr = Cursor::new(&data);
        if data.len() < 2 {
            return json!({"name":"Unknown","raw":data}).to_string();
        }
        let opcode = rdr.read_u16::<BigEndian>().unwrap();

        return match opcode {
            0x01 => self.parse_session_request(rdr),
            0x02 => self.parse_session_reply(rdr),
            0x03 => self.parse_multi(rdr),
            0x05 => self.parse_disconnect(rdr),
            0x06 => self.cached_packets.ping.parsed.to_owned(),
            0x07 => self.parse_net_status_request(rdr),
            0x08 => self.parse_net_status_reply(rdr),
            0x09 => self.parse_data(rdr, opcode),
            0x0d => self.parse_data(rdr, opcode),
            0x11 => self.parse_ack(rdr, opcode),
            0x15 => self.parse_ack(rdr, opcode),
            _ => json!({"name":"Unknown","raw":data}).to_string(),
        };
    }
    fn parse_session_request(&mut self, mut rdr: Cursor<&std::vec::Vec<u8>>) -> String {
        if !check_min_size(&rdr, PacketsMinSize::SessionRequest as usize, false) {
            return gen_size_error_json(rdr);
        }

        let crc_length = rdr.read_u32::<BigEndian>().unwrap();
        let session_id = rdr.read_u32::<BigEndian>().unwrap();
        let udp_length = rdr.read_u32::<BigEndian>().unwrap();
        let protocol_data_position = rdr.position() as usize;
        let raw_data = rdr.into_inner();
        unsafe {
            let protocol = str_from_u8_nul_utf8_unchecked(&raw_data[protocol_data_position..]);
            return format!(
                r#"{{"name":"SessionRequest","crc_length":{},"session_id":{},"udp_length":{},"protocol":"{}"}}"#,
                crc_length, session_id, udp_length, protocol
            );
        }
    }

    fn parse_session_reply(&mut self, mut rdr: Cursor<&std::vec::Vec<u8>>) -> String {
        if rdr.get_ref().len() != PacketsMinSize::SessionReply as usize {
            return gen_size_error_json(rdr);
        }
        let session_id = rdr.read_u32::<BigEndian>().unwrap();
        let crc_seed = rdr.read_u32::<BigEndian>().unwrap();
        let crc_length = rdr.read_u8().unwrap();
        let encrypt_method = rdr.read_u16::<BigEndian>().unwrap();
        let udp_length = rdr.read_u32::<BigEndian>().unwrap();
        return format!(
            r#"{{"name":"SessionReply","session_id":{},"crc_seed":{},"crc_length":{},"encrypt_method":{},"udp_length":{}}}"#,
            session_id, crc_seed, crc_length, encrypt_method, udp_length
        );
    }

    fn parse_disconnect(&mut self, mut rdr: Cursor<&std::vec::Vec<u8>>) -> String {
        if rdr.get_ref().len() != PacketsMinSize::Disconnect as usize {
            return gen_size_error_json(rdr);
        }
        let session_id = rdr.read_u32::<BigEndian>().unwrap();
        let reason = disconnect_reason_to_string(rdr.read_u16::<BigEndian>().unwrap());
        return format!(
            r#"{{"name":"Disconnect" ,"session_id":{},"reason":"{}"}}"#,
            session_id, reason
        );
    }

    fn parse_net_status_request(&mut self, mut rdr: Cursor<&std::vec::Vec<u8>>) -> String {
        if rdr.get_ref().len() != PacketsMinSize::NetStatusPacket as usize {
            return gen_size_error_json(rdr);
        }
        let client_tick_count = rdr.read_u16::<BigEndian>().unwrap();
        let last_client_update = rdr.read_u32::<BigEndian>().unwrap();
        let average_update = rdr.read_u32::<BigEndian>().unwrap();
        let shortest_update = rdr.read_u32::<BigEndian>().unwrap();
        let longest_update = rdr.read_u32::<BigEndian>().unwrap();
        let last_server_update = rdr.read_u32::<BigEndian>().unwrap();
        let packets_sent = rdr.read_u64::<BigEndian>().unwrap();
        let packets_received = rdr.read_u64::<BigEndian>().unwrap();
        let unknown_field = rdr.read_u16::<BigEndian>().unwrap();
        return format!(
            r#"{{"name":"NetStatusRequest","client_tick_count":{},"last_client_update":{},"average_update":{},"shortest_update":{},"longest_update":{},"last_server_update":{},"packets_sent":{},"packets_received":{},"unknown_field":{}}}"#,
            client_tick_count,
            last_client_update,
            average_update,
            shortest_update,
            longest_update,
            last_server_update,
            packets_sent,
            packets_received,
            unknown_field
        );
    }

    fn parse_net_status_reply(&mut self, mut rdr: Cursor<&std::vec::Vec<u8>>) -> String {
        if rdr.get_ref().len() != PacketsMinSize::NetStatusPacket as usize {
            return gen_size_error_json(rdr);
        }
        let client_tick_count = rdr.read_u16::<BigEndian>().unwrap();
        let server_tick_count = rdr.read_u32::<BigEndian>().unwrap();
        let client_packet_sent = rdr.read_u64::<BigEndian>().unwrap();
        let client_packet_received = rdr.read_u64::<BigEndian>().unwrap();
        let server_packet_sent = rdr.read_u64::<BigEndian>().unwrap();
        let server_packet_received = rdr.read_u64::<BigEndian>().unwrap();
        let unknown_field = rdr.read_u16::<BigEndian>().unwrap();
        return format!(
            r#"{{"name":"NetStatusReply","client_tick_count":{},"server_tick_count":{},"client_packet_sent":{},"client_packet_received":{},"server_packet_sent":{},"server_packet_received":{},"unknown_field":{}}}"#,
            client_tick_count,
            server_tick_count,
            client_packet_sent,
            client_packet_received,
            server_packet_sent,
            server_packet_received,
            unknown_field
        );
    }

    fn parse_multi(&mut self, mut rdr: Cursor<&std::vec::Vec<u8>>) -> String {
        // check size
        if !check_min_size(
            &rdr,
            PacketsMinSize::MultiPacket as usize,
            self.is_using_crc(),
        ) {
            return gen_size_error_json(rdr);
        }
        let mut multi_result: String = r#"{"name": "MultiPacket","sub_packets":[ "#.to_owned();
        let data_end: u64 = get_data_end(&rdr, self.is_using_crc());
        let was_crc_enabled = self.is_using_crc();
        if was_crc_enabled {
            self.disable_crc();
        }
        loop {
            let sub_packet_data_length = read_data_length(&mut rdr);
            if sub_packet_data_length == 0
                || sub_packet_data_length as u64 + rdr.position() > data_end
            {
                return gen_corruption_error_json(rdr, sub_packet_data_length, data_end);
            }
            let sub_packet_data =
                extract_subpacket_data(&rdr, rdr.position(), sub_packet_data_length);
            rdr.set_position(sub_packet_data_length as u64 + rdr.position());
            let sub_packet = self.parse(sub_packet_data);
            multi_result.push_str(&sub_packet);
            if rdr.position() == data_end {
                break;
            } else {
                multi_result.push_str(",");
            }
        }
        multi_result.push_str("]}");
        if was_crc_enabled {
            self.enable_crc();
        }

        // TODO : check crc
        return multi_result;
    }

    fn parse_data(&mut self, mut rdr: Cursor<&std::vec::Vec<u8>>, opcode: u16) -> String {
        if !check_min_size(&rdr, PacketsMinSize::DataPacket as usize, self.use_crc) {
            return gen_size_error_json(rdr);
        }
        let name = if opcode == 0x09 {
            "Data"
        } else {
            "DataFragment"
        };
        let sequence = rdr.read_u16::<BigEndian>().unwrap();

        let data_end: u64 = get_data_end(&rdr, self.use_crc);
        let mut crc: u16 = 0;
        if self.use_crc {
            rdr.set_position(data_end);
            crc = rdr.read_u16::<BigEndian>().unwrap();
        }
        let vec = rdr.get_ref().to_vec();
        let data = &vec[4..data_end as usize];
        // check that crc value is correct
        if self.use_crc {
            let packet_without_crc = &vec[0..data_end as usize];
            let crc_value =
                (crc32(&&mut packet_without_crc.to_vec(), self.crc_seed as usize) & 0xffff) as u16;
            if crc_value as u16 != crc {
                return gen_crc_error_json(&vec, crc_value, crc);
            }
        }
        return format!(
            r#"{{"name":"{}","sequence":{},"data":{:?}}}"#,
            name, sequence, data
        );
    }

    fn parse_ack(&mut self, mut rdr: Cursor<&std::vec::Vec<u8>>, opcode: u16) -> String {
        if !check_min_size(&rdr, PacketsMinSize::Ack as usize, self.use_crc) {
            return gen_size_error_json(rdr);
        }
        let name = if opcode == 0x15 { "Ack" } else { "OutOfOrder" };
        let sequence = rdr.read_u16::<BigEndian>().unwrap();
        if self.use_crc {
            let crc = rdr.read_u16::<BigEndian>().unwrap();
            let data_end: u64 = get_data_end(&rdr, self.use_crc);
            let vec = rdr.into_inner();
            let packet_without_crc = &vec[0..data_end as usize];
            let crc_value =
                (crc32(&&mut packet_without_crc.to_vec(), self.crc_seed as usize) & 0xffff) as u16;
            if crc_value as u16 != crc {
                return gen_crc_error_json(vec, crc_value, crc);
            }
        }
        return format!(r#"{{"name":"{}","sequence":{}}}"#, name, sequence);
    }

    pub fn get_crc_seed(&self) -> u32 {
        return self.crc_seed;
    }
    pub fn is_using_crc(&mut self) -> bool {
        return self.use_crc;
    }
    pub fn disable_crc(&mut self) {
        self.use_crc = false;
    }
    pub fn enable_crc(&mut self) {
        self.use_crc = true;
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn session_request_parse_test() {
        let mut soeprotocol_class = Soeprotocol::initialize(true, 0);
        let data_to_parse: [u8; 25] = [
            0, 1, 0, 0, 0, 3, 60, 23, 140, 99, 0, 0, 2, 0, 76, 111, 103, 105, 110, 85, 100, 112,
            95, 57, 0,
        ];
        let data_parsed: Value =
            serde_json::from_str(&soeprotocol_class.parse(data_to_parse.to_vec())).unwrap();
        let succesful_data: Value = serde_json::from_str(r#"{"name":"SessionRequest","crc_length":3,"session_id":1008176227,"udp_length":512,"protocol":"LoginUdp_9"}"#).unwrap();
        assert_eq!(data_parsed, succesful_data)
    }

    #[test]
    fn session_request_parse_size_error_test() {
        let mut soeprotocol_class = Soeprotocol::initialize(true, 0);
        let data_to_parse: [u8; 12] = [0, 1, 111, 103, 105, 110, 85, 100, 112, 95, 57, 2];
        let data_parsed: Value =
            serde_json::from_str(&soeprotocol_class.parse(data_to_parse.to_vec())).unwrap();
        let succesful_data: Value = serde_json::from_str(r#"{"error": "size", "name": "Error", "raw": [0, 1, 111, 103, 105, 110, 85, 100, 112,95, 57, 2], "size": 12}"#).unwrap();
        assert_eq!(data_parsed, succesful_data)
    }

    #[test]
    fn session_request_pack_test() {
        let mut soeprotocol_class = Soeprotocol::initialize(true, 0);
        let data_to_pack =
            r#"{"crc_length":3,"session_id":1008176227,"udp_length":512,"protocol":"LoginUdp_9"}"#
                .to_string();
        let data_pack: Vec<u8> = soeprotocol_class.pack("SessionRequest".to_owned(), data_to_pack);
        assert_eq!(
            data_pack,
            [
                0, 1, 0, 0, 0, 3, 60, 23, 140, 99, 0, 0, 2, 0, 76, 111, 103, 105, 110, 85, 100,
                112, 95, 57, 0
            ]
        )
    }

    #[test]
    fn session_request_pack_test_deserializing_error() {
        let mut soeprotocol_class = Soeprotocol::initialize(false, 0);
        let data_to_pack =
            r#"{"crc_length":3,"udp_length":512,"protocol":"LoginUdp_9"}"#.to_string();
        let data_pack: Vec<u8> = soeprotocol_class.pack("SessionRequest".to_owned(), data_to_pack);
        assert_eq!(data_pack, vec![] as Vec<u8>)
    }

    #[test]
    fn session_reply_parse_size_error_test() {
        let mut soeprotocol_class = Soeprotocol::initialize(true, 0);
        let data_to_parse: [u8; 10] = [0, 2, 111, 103, 105, 110, 85, 100, 112, 95];
        let data_parsed: Value =
            serde_json::from_str(&soeprotocol_class.parse(data_to_parse.to_vec())).unwrap();
        let succesful_data: Value = serde_json::from_str(r#"{"error": "size", "name": "Error", "raw": [0,2,111,103,105,110,85,100,112,95], "size": 10}"#).unwrap();
        assert_eq!(data_parsed, succesful_data)
    }

    #[test]
    fn session_reply_parse_test() {
        let mut soeprotocol_class = Soeprotocol::initialize(true, 0);
        let data_to_parse: [u8; 21] = [
            0, 2, 60, 23, 140, 99, 0, 0, 0, 0, 2, 1, 0, 0, 0, 2, 0, 0, 0, 0, 3,
        ];
        let data_parsed: Value =
            serde_json::from_str(&soeprotocol_class.parse(data_to_parse.to_vec())).unwrap();
        let succesful_data: Value = serde_json::from_str(r#"{"name":"SessionReply","session_id":1008176227,"crc_seed":0,"crc_length":2,"encrypt_method":256,"udp_length":512}"#).unwrap();
        assert_eq!(data_parsed, succesful_data)
    }

    #[test]
    fn session_reply_pack_test() {
        let mut soeprotocol_class = Soeprotocol::initialize(true, 0);
        let data_to_pack =  r#"{"session_id":1008176227,"crc_seed":0,"crc_length":2,"encrypt_method":256,"udp_length":512}"#.to_string();
        let data_pack: Vec<u8> = soeprotocol_class.pack("SessionReply".to_owned(), data_to_pack);
        assert_eq!(
            data_pack,
            [0, 2, 60, 23, 140, 99, 0, 0, 0, 0, 2, 1, 0, 0, 0, 2, 0, 0, 0, 0, 3]
        )
    }

    #[test]
    fn session_reply_pack_test_deserializing_error() {
        let mut soeprotocol_class = Soeprotocol::initialize(false, 0);
        let data_to_pack =
            r#"{"crc_seed":0,"crc_length":2,"encrypt_method":256,"udp_length":512}"#.to_string();
        let data_pack: Vec<u8> = soeprotocol_class.pack("SessionReply".to_owned(), data_to_pack);
        assert_eq!(data_pack, vec![] as Vec<u8>)
    }

    #[test]
    fn net_status_request_parse_test() {
        let mut soeprotocol_class = Soeprotocol::initialize(true, 0);
        let data_to_parse: [u8; 42] = [
            0, 7, 251, 92, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 1, 235, 216,
        ];
        let data_parsed: Value =
            serde_json::from_str(&soeprotocol_class.parse(data_to_parse.to_vec())).unwrap();
        let succesful_data: Value = serde_json::from_str(r#"{"average_update": 0, "client_tick_count": 64348, "last_client_update": 0, "last_server_update": 0, "longest_update": 0, "name": "NetStatusRequest", "packets_received": 1, "packets_sent": 2, "shortest_update": 0, "unknown_field": 60376}"#).unwrap();
        assert_eq!(data_parsed, succesful_data)
    }

    #[test]
    fn net_status_request_parse_deserializing_error() {
        let mut soeprotocol_class = Soeprotocol::initialize(true, 0);
        let data_to_parse: [u8; 43] = [
            0, 7, 251, 92, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 1, 235, 216, 0,
        ];
        let data_parsed: Value =
            serde_json::from_str(&soeprotocol_class.parse(data_to_parse.to_vec())).unwrap();
        let succesful_data: Value = serde_json::from_str(r#"{"error": "size", "name": "Error", "raw": [0,7,251,92,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2,0,0,0,0,0,0,0,1,235,216,0], "size": 43}"#).unwrap();
        assert_eq!(data_parsed, succesful_data)
    }

    #[test]
    fn net_status_request_pack_test() {
        let mut soeprotocol_class = Soeprotocol::initialize(false, 0);
        let data_to_pack =
        r#"{"average_update": 0, "client_tick_count": 64348, "last_client_update": 0, "last_server_update": 0, "longest_update": 0, "name": "NetStatusRequest", "packets_received": 1, "packets_sent": 2, "shortest_update": 0, "unknown_field": 60376}"#
                .to_string();
        let data_pack: Vec<u8> =
            soeprotocol_class.pack("NetStatusRequest".to_owned(), data_to_pack);
        assert_eq!(
            data_pack,
            [
                0, 7, 251, 92, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 1, 235, 216
            ]
        )
    }

    #[test]
    fn net_status_request_pack_test_size_error() {
        let mut soeprotocol_class = Soeprotocol::initialize(false, 0);
        let data_to_pack =
        r#"{"client_packet_received": 0, "client_packet_sent": 0, "client_tick_count": 64348, "name": "NetStatusRequest", "server_packet_received": 1, "server_packet_sent": 2, "server_tick_count": 0}"#.to_string();
        let data_pack: Vec<u8> =
            soeprotocol_class.pack("NetStatusRequest".to_owned(), data_to_pack);
        assert_eq!(data_pack, vec![] as Vec<u8>)
    }

    #[test]
    fn net_status_reply_parse_test() {
        let mut soeprotocol_class = Soeprotocol::initialize(true, 0);
        let data_to_parse: [u8; 42] = [
            0, 8, 251, 92, 33, 39, 197, 60, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0,
            0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 2, 131, 212,
        ];
        let data_parsed: Value =
            serde_json::from_str(&soeprotocol_class.parse(data_to_parse.to_vec())).unwrap();
        let succesful_data: Value = serde_json::from_str(r#"{"client_packet_received": 1, "client_packet_sent": 2, "client_tick_count": 64348, "name": "NetStatusReply", "server_packet_received": 2, "server_packet_sent": 1, "server_tick_count": 556254524, "unknown_field": 33748}"#).unwrap();
        assert_eq!(data_parsed, succesful_data)
    }

    #[test]
    fn net_status_reply_parse_size_error() {
        let mut soeprotocol_class = Soeprotocol::initialize(true, 0);
        let data_to_parse: [u8; 43] = [
            0, 8, 251, 92, 33, 39, 197, 60, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0,
            0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 2, 131, 212, 0,
        ];
        let data_parsed: Value =
            serde_json::from_str(&soeprotocol_class.parse(data_to_parse.to_vec())).unwrap();
        let succesful_data: Value = serde_json::from_str(r#"{"error": "size", "name": "Error", "raw": [0,8,251,92,33,39,197,60,0,0,0,0,0,0,0,2,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,2,131,212,0], "size": 43}"#).unwrap();
        assert_eq!(data_parsed, succesful_data)
    }

    #[test]
    fn net_status_reply_pack_test() {
        let mut soeprotocol_class = Soeprotocol::initialize(false, 0);
        let data_to_pack =
        r#"{"client_packet_received": 1, "client_packet_sent": 2, "client_tick_count": 64348, "name": "NetStatusReply", "server_packet_received": 2, "server_packet_sent": 1, "server_tick_count": 556254524, "unknown_field": 33748}"#
                .to_string();
        let data_pack: Vec<u8> = soeprotocol_class.pack("NetStatusReply".to_owned(), data_to_pack);
        assert_eq!(
            data_pack,
            [
                0, 8, 251, 92, 33, 39, 197, 60, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 1, 0,
                0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 2, 131, 212
            ]
        )
    }

    #[test]
    fn net_status_reply_pack_test_deserializing_error() {
        let mut soeprotocol_class = Soeprotocol::initialize(false, 0);
        let data_to_pack =
        r#"{"client_packet_received": 0, "client_packet_sent": 0, "client_tick_cozunt": 64348, "name": "NetStatusRequest", "server_packet_received": 1, "server_packet_sent": 2, "server_tick_count": 0}"#.to_string();
        let data_pack: Vec<u8> =
            soeprotocol_class.pack("NetStatusRequest".to_owned(), data_to_pack);
        assert_eq!(data_pack, vec![] as Vec<u8>)
    }

    #[test]
    fn ping_parse_test() {
        let mut soeprotocol_class = Soeprotocol::initialize(true, 0);
        let data_to_parse: [u8; 2] = [0, 6];
        let data_parsed: String = soeprotocol_class.parse(data_to_parse.to_vec());
        assert_eq!(data_parsed, r#"{"name":"Ping"}"#)
    }

    #[test]
    fn ping_pack_test() {
        let mut soeprotocol_class = Soeprotocol::initialize(true, 0);
        let data_to_pack: String = r#"{"name":"Ping"}"#.to_owned();
        let data_pack: Vec<u8> = soeprotocol_class.pack("Ping".to_owned(), data_to_pack);
        assert_eq!(data_pack, [0, 6])
    }

    #[test]
    fn outoforder_parse_size_error_test() {
        let mut soeprotocol_class = Soeprotocol::initialize(true, 0);
        let data_to_parse: [u8; 3] = [0, 17, 111];
        let data_parsed: Value =
            serde_json::from_str(&soeprotocol_class.parse(data_to_parse.to_vec())).unwrap();
        let succesful_data: Value = serde_json::from_str(
            r#"{"error": "size", "name": "Error", "raw": [0, 17, 111], "size": 3}"#,
        )
        .unwrap();
        assert_eq!(data_parsed, succesful_data)
    }

    #[test]
    fn outoforder_parse_test() {
        let mut soeprotocol_class = Soeprotocol::initialize(true, 0);
        let data_to_parse: [u8; 6] = [0, 17, 0, 1, 38, 184];
        let data_parsed: Value =
            serde_json::from_str(&soeprotocol_class.parse(data_to_parse.to_vec())).unwrap();
        let succesful_data: Value =
            serde_json::from_str(r#"{"name":"OutOfOrder","sequence":1}"#).unwrap();
        assert_eq!(data_parsed, succesful_data)
    }

    #[test]
    fn outoforder_parse_test_crc_fail() {
        let mut soeprotocol_class = Soeprotocol::initialize(true, 0);
        let data_to_parse: [u8; 6] = [0, 17, 0, 1, 142, 100];
        let data_parsed: Value =
            serde_json::from_str(&soeprotocol_class.parse(data_to_parse.to_vec())).unwrap();
        let succesful_data: Value = serde_json::from_str(r#"{"error": "crc", "expected_crc": 9912, "given_crc": 36452, "name": "Error", "raw": [0, 17, 0, 1, 142, 100]}"#).unwrap();
        assert_eq!(data_parsed, succesful_data)
    }

    #[test]
    fn outoforder_pack_test() {
        let mut soeprotocol_class = Soeprotocol::initialize(false, 0);
        let data_to_pack: String = r#"{"name":"OutOfOrder","sequence":1}"#.to_owned();
        let data_pack: Vec<u8> = soeprotocol_class.pack("OutOfOrder".to_owned(), data_to_pack);
        assert_eq!(data_pack, [0, 17, 0, 1])
    }

    #[test]
    fn outoforder_pack_test_deserializing_error() {
        let mut soeprotocol_class = Soeprotocol::initialize(false, 0);
        let data_to_pack: String = r#"{"sequednce":1}"#.to_owned();
        let data_pack: Vec<u8> = soeprotocol_class.pack("OutOfOrder".to_owned(), data_to_pack);
        assert_eq!(data_pack, vec![] as Vec<u8>)
    }

    #[test]
    fn outoforder_pack_with_crc_test() {
        let mut soeprotocol_class = Soeprotocol::initialize(true, 0);
        let data_to_pack: String = r#"{"name":"OutOfOrder","sequence":1}"#.to_owned();
        let data_pack: Vec<u8> = soeprotocol_class.pack("OutOfOrder".to_owned(), data_to_pack);
        assert_eq!(data_pack, [0, 17, 0, 1, 38, 184])
    }

    #[test]
    fn ack_parse_size_error_test() {
        let mut soeprotocol_class = Soeprotocol::initialize(true, 0);
        let data_to_parse: [u8; 3] = [0, 21, 111];
        let data_parsed: Value =
            serde_json::from_str(&soeprotocol_class.parse(data_to_parse.to_vec())).unwrap();
        let succesful_data: Value = serde_json::from_str(
            r#"{"error": "size", "name": "Error", "raw": [0, 21, 111], "size": 3}"#,
        )
        .unwrap();
        assert_eq!(data_parsed, succesful_data)
    }

    #[test]
    fn ack_parse_test() {
        let mut soeprotocol_class = Soeprotocol::initialize(true, 0);
        let data_to_parse: [u8; 6] = [0, 21, 0, 1, 142, 100];
        let data_parsed: Value =
            serde_json::from_str(&soeprotocol_class.parse(data_to_parse.to_vec())).unwrap();
        let succesful_data: Value = serde_json::from_str(r#"{"name":"Ack","sequence":1}"#).unwrap();
        assert_eq!(data_parsed, succesful_data)
    }

    #[test]
    fn ack_pack_test() {
        let mut soeprotocol_class = Soeprotocol::initialize(false, 0);
        let data_to_pack: String = r#"{"name":"Ack","sequence":1}"#.to_owned();
        let data_pack: Vec<u8> = soeprotocol_class.pack("Ack".to_owned(), data_to_pack);
        assert_eq!(data_pack, [0, 21, 0, 1])
    }

    #[test]
    fn ack_pack_test_deserializing_error() {
        let mut soeprotocol_class = Soeprotocol::initialize(false, 0);
        let data_to_pack: String = r#"{"name":"Ack"}"#.to_owned();
        let data_pack: Vec<u8> = soeprotocol_class.pack("Ack".to_owned(), data_to_pack);
        assert_eq!(data_pack, vec![] as Vec<u8>)
    }

    #[test]
    fn ack_pack_with_crc_test() {
        let mut soeprotocol_class = Soeprotocol::initialize(true, 0);
        let data_to_pack: String = r#"{"name":"Ack","sequence":1}"#.to_owned();
        let data_pack: Vec<u8> = soeprotocol_class.pack("Ack".to_owned(), data_to_pack);
        assert_eq!(data_pack, [0, 21, 0, 1, 142, 100])
    }

    #[test]
    fn multi_parse_size_error_test() {
        let mut soeprotocol_class = Soeprotocol::initialize(true, 0);
        let data_to_parse: [u8; 4] = [0, 3, 4, 0];
        let data_parsed: Value =
            serde_json::from_str(&soeprotocol_class.parse(data_to_parse.to_vec())).unwrap();
        let succesful_data: Value = serde_json::from_str(
            r#"{"error": "size", "name": "Error", "raw": [0, 3, 4, 0], "size": 4}"#,
        )
        .unwrap();
        assert_eq!(data_parsed, succesful_data)
    }

    #[test]
    fn multi_parse_corrupted_test() {
        let mut soeprotocol_class = Soeprotocol::initialize(true, 0);
        let data_to_parse: [u8; 75] = [
            0, 3, 54, 0, 21, 0, 206, 67, 0, 9, 0, 1, 0, 25, 41, 141, 45, 189, 85, 241, 64, 165, 71,
            228, 114, 81, 54, 5, 184, 205, 104, 0, 125, 184, 210, 74, 0, 247, 152, 225, 169, 102,
            204, 158, 233, 202, 228, 34, 202, 238, 136, 31, 3, 121, 222, 106, 11, 247, 177, 138,
            145, 21, 221, 187, 36, 170, 37, 171, 6, 32, 11, 180, 97, 10, 246,
        ];
        let data_parsed: Value =
            serde_json::from_str(&soeprotocol_class.parse(data_to_parse.to_vec())).unwrap();
        let succesful_data: Value = serde_json::from_str(r#"{"error": "corruption", "name": "Error", "data_end": 73,"position": 58, "subpacket_length": 247,"raw":[0, 3, 54, 0, 21, 0, 206, 67, 0, 9, 0, 1, 0, 25, 41, 141, 45, 189, 85, 241, 64, 165, 71,228, 114, 81, 54, 5, 184, 205, 104, 0, 125, 184, 210, 74, 0, 247, 152, 225, 169, 102,204, 158, 233, 202, 228, 34, 202, 238, 136, 31, 3, 121, 222, 106, 11, 247, 177, 138,145, 21, 221, 187, 36, 170, 37, 171, 6, 32, 11, 180, 97, 10, 246]}"#).unwrap();
        assert_eq!(data_parsed, succesful_data)
    }

    #[test]
    fn multi_parse_test() {
        let mut soeprotocol_class = Soeprotocol::initialize(false, 0);
        let data_to_parse: [u8; 75] = [
            0, 3, 4, 0, 21, 0, 206, 67, 0, 9, 0, 1, 0, 25, 41, 141, 45, 189, 85, 241, 64, 165, 71,
            228, 114, 81, 54, 5, 184, 205, 104, 0, 125, 184, 210, 74, 0, 247, 152, 225, 169, 102,
            204, 158, 233, 202, 228, 34, 202, 238, 136, 31, 3, 121, 222, 106, 11, 247, 177, 138,
            145, 21, 221, 187, 36, 170, 37, 171, 6, 32, 11, 180, 97, 10, 246,
        ];
        let data_parsed: Value =
            serde_json::from_str(&soeprotocol_class.parse(data_to_parse.to_vec())).unwrap();
        let succesful_data: Value = serde_json::from_str(r#"{"name":"MultiPacket","sub_packets":[{"name":"Ack","sequence":206},{"name":"Data","sequence":1,"data":[0,25,41,141,45,189,85,241,64,165,71,228,114,81,54,5,184,205,104,0,125,184,210,74,0,247,152,225,169,102,204,158,233,202,228,34,202,238,136,31,3,121,222,106,11,247,177,138,145,21,221,187,36,170,37,171,6,32,11,180,97,10,246]}]}"#).unwrap();
        assert_eq!(data_parsed, succesful_data)
    }

    #[test]
    fn multi_parse_with_crc_test() {
        let mut soeprotocol_class = Soeprotocol::initialize(true, 0);
        let data_to_parse: [u8; 77] = [
            0, 3, 4, 0, 21, 0, 206, 67, 0, 9, 0, 1, 0, 25, 41, 141, 45, 189, 85, 241, 64, 165, 71,
            228, 114, 81, 54, 5, 184, 205, 104, 0, 125, 184, 210, 74, 0, 247, 152, 225, 169, 102,
            204, 158, 233, 202, 228, 34, 202, 238, 136, 31, 3, 121, 222, 106, 11, 247, 177, 138,
            145, 21, 221, 187, 36, 170, 37, 171, 6, 32, 11, 180, 97, 10, 246, 10, 27,
        ];
        let data_parsed: Value =
            serde_json::from_str(&soeprotocol_class.parse(data_to_parse.to_vec())).unwrap();
        let succesful_data: Value = serde_json::from_str(r#"{"name":"MultiPacket","sub_packets":[{"name":"Ack","sequence":206},{"name":"Data","sequence":1,"data":[0,25,41,141,45,189,85,241,64,165,71,228,114,81,54,5,184,205,104,0,125,184,210,74,0,247,152,225,169,102,204,158,233,202,228,34,202,238,136,31,3,121,222,106,11,247,177,138,145,21,221,187,36,170,37,171,6,32,11,180,97,10,246]}]}"#).unwrap();
        assert_eq!(data_parsed, succesful_data)
    }

    #[test]
    fn multi_pack_test() {
        let mut soeprotocol_class = Soeprotocol::initialize(false, 0);
        let data_to_pack:String = r#"{"sub_packets":[{"name":"Ack","sequence":206},{"name":"Data","sequence":1,"data":[0,25,41,141,45,189,85,241,64,165,71,228,114,81,54,5,184,205,104,0,125,184,210,74,0,247,152,225,169,102,204,158,233,202,228,34,202,238,136,31,3,121,222,106,11,247,177,138,145,21,221,187,36,170,37,171,6,32,11,180,97,10,246]}]}"#.to_owned();
        let data_pack: Vec<u8> = soeprotocol_class.pack("MultiPacket".to_owned(), data_to_pack);
        assert_eq!(
            data_pack,
            [
                0, 3, 4, 0, 21, 0, 206, 67, 0, 9, 0, 1, 0, 25, 41, 141, 45, 189, 85, 241, 64, 165,
                71, 228, 114, 81, 54, 5, 184, 205, 104, 0, 125, 184, 210, 74, 0, 247, 152, 225,
                169, 102, 204, 158, 233, 202, 228, 34, 202, 238, 136, 31, 3, 121, 222, 106, 11,
                247, 177, 138, 145, 21, 221, 187, 36, 170, 37, 171, 6, 32, 11, 180, 97, 10, 246,
            ]
        )
    }

    #[test]
    fn multi_pack_test_deserializing_error() {
        let mut soeprotocol_class = Soeprotocol::initialize(false, 0);
        let data_to_pack:String = r#"{"sub_packets":[{"sequence":206},{"name":"Data","sequence":1,"data":[0,25,41,141,45,189,85,241,64,165,71,228,114,81,54,5,184,205,104,0,125,184,210,74,0,247,152,225,169,102,204,158,233,202,228,34,202,238,136,31,3,121,222,106,11,247,177,138,145,21,221,187,36,170,37,171,6,32,11,180,97,10,246]}]}"#.to_owned();
        let data_pack: Vec<u8> = soeprotocol_class.pack("MultiPacket".to_owned(), data_to_pack);
        assert_eq!(data_pack, vec![] as Vec<u8>)
    }

    #[test]
    fn multi_pack_with_crc_test() {
        let mut soeprotocol_class = Soeprotocol::initialize(true, 0);
        let data_to_pack:String = r#"{"sub_packets":[{"name":"Ack","sequence":206},{"name":"Data","sequence":1,"data":[0,25,41,141,45,189,85,241,64,165,71,228,114,81,54,5,184,205,104,0,125,184,210,74,0,247,152,225,169,102,204,158,233,202,228,34,202,238,136,31,3,121,222,106,11,247,177,138,145,21,221,187,36,170,37,171,6,32,11,180,97,10,246]}]}"#.to_owned();
        let data_pack: Vec<u8> = soeprotocol_class.pack("MultiPacket".to_owned(), data_to_pack);
        assert_eq!(
            data_pack,
            [
                0, 3, 4, 0, 21, 0, 206, 67, 0, 9, 0, 1, 0, 25, 41, 141, 45, 189, 85, 241, 64, 165,
                71, 228, 114, 81, 54, 5, 184, 205, 104, 0, 125, 184, 210, 74, 0, 247, 152, 225,
                169, 102, 204, 158, 233, 202, 228, 34, 202, 238, 136, 31, 3, 121, 222, 106, 11,
                247, 177, 138, 145, 21, 221, 187, 36, 170, 37, 171, 6, 32, 11, 180, 97, 10, 246,
                10, 27
            ]
        )
    }

    #[test]
    fn data_parse_test() {
        let mut soeprotocol_class = Soeprotocol::initialize(false, 0);
        let data_to_parse: [u8; 45] = [
            0, 9, 0, 4, 252, 100, 40, 209, 68, 247, 21, 93, 18, 172, 91, 68, 145, 53, 24, 155, 2,
            113, 179, 28, 217, 33, 80, 76, 9, 235, 87, 98, 233, 235, 220, 124, 107, 61, 62, 132,
            117, 146, 204, 94, 60,
        ];
        let data_parsed: Value =
            serde_json::from_str(&soeprotocol_class.parse(data_to_parse.to_vec())).unwrap();
        let succesful_data: Value = serde_json::from_str(r#"{"name":"Data","sequence":4,"data":[252,100,40,209,68,247,21,93,18,172,91,68,145,53,24,155,2,113,179,28,217,33,80,76,9,235,87,98,233,235,220,124,107,61,62,132,117,146,204,94,60]}"#).unwrap();
        assert_eq!(data_parsed, succesful_data)
    }

    #[test]
    fn data_parse_with_crc_test() {
        let mut soeprotocol_class = Soeprotocol::initialize(true, 0);
        let data_to_parse: [u8; 45] = [
            0, 9, 0, 4, 252, 100, 40, 209, 68, 247, 21, 93, 18, 172, 91, 68, 145, 53, 24, 155, 2,
            113, 179, 28, 217, 33, 80, 76, 9, 235, 87, 98, 233, 235, 220, 124, 107, 61, 62, 132,
            117, 146, 204, 94, 60,
        ];
        let data_parsed: Value =
            serde_json::from_str(&soeprotocol_class.parse(data_to_parse.to_vec())).unwrap();
        let succesful_data: Value = serde_json::from_str(r#"{"name":"Data","sequence":4,"data":[252,100,40,209,68,247,21,93,18,172,91,68,145,53,24,155,2,113,179,28,217,33,80,76,9,235,87,98,233,235,220,124,107,61,62,132,117,146,204]}"#).unwrap();
        assert_eq!(data_parsed, succesful_data)
    }

    #[test]
    fn data_parse_with_crc_test_fail() {
        let mut soeprotocol_class = Soeprotocol::initialize(true, 0);
        let data_to_parse: [u8; 45] = [
            0, 9, 0, 4, 252, 100, 40, 209, 68, 247, 21, 93, 18, 172, 91, 68, 145, 53, 24, 155, 2,
            113, 179, 28, 217, 33, 80, 76, 9, 235, 87, 98, 233, 235, 220, 124, 107, 61, 62, 132,
            117, 146, 204, 94, 61,
        ];
        let data_parsed: Value =
            serde_json::from_str(&soeprotocol_class.parse(data_to_parse.to_vec())).unwrap();
        let succesful_data: Value = serde_json::from_str(r#"{"error": "crc", "expected_crc": 24124, "given_crc": 24125, "name": "Error", "raw": [0, 9, 0, 4, 252, 100, 40, 209, 68, 247, 21, 93, 18, 172, 91, 68, 145, 53, 24, 155, 2,113, 179, 28, 217, 33, 80, 76, 9, 235, 87, 98, 233, 235, 220, 124, 107, 61, 62, 132,117, 146, 204, 94, 61]}"#).unwrap();
        assert_eq!(data_parsed, succesful_data)
    }

    #[test]
    fn data_pack_test() {
        let mut soeprotocol_class = Soeprotocol::initialize(false, 0);
        let data_to_pack =
            r#"{"sequence":0,"data":[2,1,1,0,0,0,1,1,3,0,0,0,115,111,101,0,0,0,0]}"#.to_string();
        let data_pack: Vec<u8> = soeprotocol_class.pack("Data".to_owned(), data_to_pack);
        assert_eq!(
            data_pack,
            [0, 9, 0, 0, 2, 1, 1, 0, 0, 0, 1, 1, 3, 0, 0, 0, 115, 111, 101, 0, 0, 0, 0]
        )
    }

    #[test]
    fn data_pack_test_deserializing_error() {
        let mut soeprotocol_class = Soeprotocol::initialize(false, 0);
        let data_to_pack = r#"{"data":[2,1,1,0,0,0,1,1,3,0,0,0,115,111,101,0,0,0,0]}"#.to_string();
        let data_pack: Vec<u8> = soeprotocol_class.pack("Data".to_owned(), data_to_pack);
        assert_eq!(data_pack, vec![] as Vec<u8>)
    }

    #[test]
    fn data_pack_with_crc_test() {
        let mut soeprotocol_class = Soeprotocol::initialize(true, 0);
        let data_to_pack =
            r#"{"sequence":0,"data":[2,1,1,0,0,0,1,1,3,0,0,0,115,111,101,0,0,0,0]}"#.to_string();
        let data_pack: Vec<u8> = soeprotocol_class.pack("Data".to_owned(), data_to_pack);
        assert_eq!(
            data_pack,
            [0, 9, 0, 0, 2, 1, 1, 0, 0, 0, 1, 1, 3, 0, 0, 0, 115, 111, 101, 0, 0, 0, 0, 23, 207]
        )
    }

    #[test]
    fn data_fragment_parse_test() {
        let mut soeprotocol_class = Soeprotocol::initialize(false, 0);
        let data_to_parse: [u8; 257] = [
            0, 13, 0, 2, 208, 127, 31, 117, 87, 54, 201, 180, 188, 226, 247, 253, 136, 66, 78, 125,
            224, 112, 23, 87, 147, 110, 18, 68, 183, 87, 20, 3, 65, 116, 82, 111, 93, 219, 229, 20,
            61, 238, 143, 63, 8, 137, 8, 196, 128, 89, 59, 4, 198, 191, 207, 141, 23, 164, 242, 77,
            176, 206, 49, 45, 207, 210, 17, 33, 75, 177, 157, 242, 169, 37, 60, 87, 245, 58, 2,
            130, 102, 146, 227, 66, 193, 153, 155, 105, 230, 203, 120, 114, 160, 223, 229, 190,
            129, 106, 19, 25, 8, 52, 55, 8, 100, 68, 109, 228, 178, 186, 148, 108, 138, 242, 136,
            66, 219, 25, 73, 129, 110, 31, 121, 32, 246, 86, 156, 212, 85, 217, 213, 119, 165, 140,
            83, 95, 6, 183, 184, 251, 73, 102, 221, 156, 240, 204, 50, 217, 217, 13, 218, 2, 19,
            44, 143, 73, 168, 109, 67, 176, 129, 225, 187, 171, 12, 146, 21, 66, 252, 150, 143,
            142, 46, 39, 72, 12, 22, 222, 7, 29, 63, 201, 227, 251, 9, 28, 0, 100, 84, 153, 84,
            212, 163, 78, 135, 33, 66, 20, 195, 223, 62, 214, 32, 59, 6, 187, 222, 99, 29, 34, 87,
            81, 61, 63, 174, 255, 1, 85, 241, 6, 10, 152, 237, 52, 51, 126, 149, 218, 125, 232,
            199, 40, 113, 139, 187, 43, 232, 209, 167, 226, 91, 236, 212, 165, 117, 19, 118, 110,
            18, 0, 26, 152, 33, 115, 61, 208, 21,
        ];
        let data_parsed: Value =
            serde_json::from_str(&soeprotocol_class.parse(data_to_parse.to_vec())).unwrap();
        let succesful_data: Value = serde_json::from_str(r#"{"name":"DataFragment","sequence":2,"data":[208,127,31,117,87,54,201,180,188,226,247,253,136,66,78,125,224,112,23,87,147,110,18,68,183,87,20,3,65,116,82,111,93,219,229,20,61,238,143,63,8,137,8,196,128,89,59,4,198,191,207,141,23,164,242,77,176,206,49,45,207,210,17,33,75,177,157,242,169,37,60,87,245,58,2,130,102,146,227,66,193,153,155,105,230,203,120,114,160,223,229,190,129,106,19,25,8,52,55,8,100,68,109,228,178,186,148,108,138,242,136,66,219,25,73,129,110,31,121,32,246,86,156,212,85,217,213,119,165,140,83,95,6,183,184,251,73,102,221,156,240,204,50,217,217,13,218,2,19,44,143,73,168,109,67,176,129,225,187,171,12,146,21,66,252,150,143,142,46,39,72,12,22,222,7,29,63,201,227,251,9,28,0,100,84,153,84,212,163,78,135,33,66,20,195,223,62,214,32,59,6,187,222,99,29,34,87,81,61,63,174,255,1,85,241,6,10,152,237,52,51,126,149,218,125,232,199,40,113,139,187,43,232,209,167,226,91,236,212,165,117,19,118,110,18,0,26,152,33,115,61,208,21]}"#).unwrap();
        assert_eq!(data_parsed, succesful_data)
    }

    #[test]
    fn data_fragment_parse_with_crc_test() {
        let mut soeprotocol_class = Soeprotocol::initialize(false, 0);
        let data_to_parse: [u8; 257] = [
            0, 13, 0, 2, 208, 127, 31, 117, 87, 54, 201, 180, 188, 226, 247, 253, 136, 66, 78, 125,
            224, 112, 23, 87, 147, 110, 18, 68, 183, 87, 20, 3, 65, 116, 82, 111, 93, 219, 229, 20,
            61, 238, 143, 63, 8, 137, 8, 196, 128, 89, 59, 4, 198, 191, 207, 141, 23, 164, 242, 77,
            176, 206, 49, 45, 207, 210, 17, 33, 75, 177, 157, 242, 169, 37, 60, 87, 245, 58, 2,
            130, 102, 146, 227, 66, 193, 153, 155, 105, 230, 203, 120, 114, 160, 223, 229, 190,
            129, 106, 19, 25, 8, 52, 55, 8, 100, 68, 109, 228, 178, 186, 148, 108, 138, 242, 136,
            66, 219, 25, 73, 129, 110, 31, 121, 32, 246, 86, 156, 212, 85, 217, 213, 119, 165, 140,
            83, 95, 6, 183, 184, 251, 73, 102, 221, 156, 240, 204, 50, 217, 217, 13, 218, 2, 19,
            44, 143, 73, 168, 109, 67, 176, 129, 225, 187, 171, 12, 146, 21, 66, 252, 150, 143,
            142, 46, 39, 72, 12, 22, 222, 7, 29, 63, 201, 227, 251, 9, 28, 0, 100, 84, 153, 84,
            212, 163, 78, 135, 33, 66, 20, 195, 223, 62, 214, 32, 59, 6, 187, 222, 99, 29, 34, 87,
            81, 61, 63, 174, 255, 1, 85, 241, 6, 10, 152, 237, 52, 51, 126, 149, 218, 125, 232,
            199, 40, 113, 139, 187, 43, 232, 209, 167, 226, 91, 236, 212, 165, 117, 19, 118, 110,
            18, 0, 26, 152, 33, 115, 61, 208, 21,
        ];
        let data_parsed: Value =
            serde_json::from_str(&soeprotocol_class.parse(data_to_parse.to_vec())).unwrap();
        let succesful_data: Value = serde_json::from_str(r#"{"name":"DataFragment","sequence":2,"data":[208,127,31,117,87,54,201,180,188,226,247,253,136,66,78,125,224,112,23,87,147,110,18,68,183,87,20,3,65,116,82,111,93,219,229,20,61,238,143,63,8,137,8,196,128,89,59,4,198,191,207,141,23,164,242,77,176,206,49,45,207,210,17,33,75,177,157,242,169,37,60,87,245,58,2,130,102,146,227,66,193,153,155,105,230,203,120,114,160,223,229,190,129,106,19,25,8,52,55,8,100,68,109,228,178,186,148,108,138,242,136,66,219,25,73,129,110,31,121,32,246,86,156,212,85,217,213,119,165,140,83,95,6,183,184,251,73,102,221,156,240,204,50,217,217,13,218,2,19,44,143,73,168,109,67,176,129,225,187,171,12,146,21,66,252,150,143,142,46,39,72,12,22,222,7,29,63,201,227,251,9,28,0,100,84,153,84,212,163,78,135,33,66,20,195,223,62,214,32,59,6,187,222,99,29,34,87,81,61,63,174,255,1,85,241,6,10,152,237,52,51,126,149,218,125,232,199,40,113,139,187,43,232,209,167,226,91,236,212,165,117,19,118,110,18,0,26,152,33,115,61,208,21]}"#).unwrap();
        assert_eq!(data_parsed, succesful_data)
    }

    #[test]
    fn data_fragment_pack_test() {
        let mut soeprotocol_class = Soeprotocol::initialize(false, 0);
        let data_to_pack =
            r#"{"sequence":2,"data":[2,1,1,0,0,0,1,1,3,0,0,0,115,111,101,0,0,0,0]}"#.to_string();
        let data_pack: Vec<u8> = soeprotocol_class.pack("DataFragment".to_owned(), data_to_pack);
        assert_eq!(
            data_pack,
            [0, 13, 0, 2, 2, 1, 1, 0, 0, 0, 1, 1, 3, 0, 0, 0, 115, 111, 101, 0, 0, 0, 0]
        )
    }

    #[test]
    fn data_fragment_pack_test_deserializing_error() {
        let mut soeprotocol_class = Soeprotocol::initialize(false, 0);
        let data_to_pack = r#"{"data":[2,1,1,0,0,0,1,1,3,0,0,0,115,111,101,0,0,0,0]}"#.to_string();
        let data_pack: Vec<u8> = soeprotocol_class.pack("DataFragment".to_owned(), data_to_pack);
        assert_eq!(data_pack, vec![] as Vec<u8>)
    }

    #[test]
    fn data_fragment_pack_with_crc_test() {
        let mut soeprotocol_class = Soeprotocol::initialize(true, 0);
        let data_to_pack =
            r#"{"sequence":2,"data":[2,1,1,0,0,0,1,1,3,0,0,0,115,111,101,0,0,0,0]}"#.to_string();
        let data_pack: Vec<u8> = soeprotocol_class.pack("DataFragment".to_owned(), data_to_pack);
        assert_eq!(
            data_pack,
            [0, 13, 0, 2, 2, 1, 1, 0, 0, 0, 1, 1, 3, 0, 0, 0, 115, 111, 101, 0, 0, 0, 0, 242, 67]
        )
    }
}
