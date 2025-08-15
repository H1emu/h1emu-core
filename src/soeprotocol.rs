use super::protocol_errors::{
    gen_corruption_error_json, gen_crc_error_json, gen_deserializing_error_json,
    gen_size_error_json,
};

use super::soeprotocol_functions::*;
use super::{
    crc::crc32,
    lib_utils::{str_from_u8_nul_utf8_checked, u8_from_str_nul_utf8_unchecked},
    soeprotocol_packets_structs::*,
};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use gloo_utils::format::JsValueSerdeExt;
use std::io::Cursor;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Soeprotocol {
    use_crc: bool,
    crc_seed: u32,
    wtr: Vec<u8>,
}

#[wasm_bindgen]
pub enum EncryptMethod {
    EncryptMethodNone = 0x0,
    EncryptMethodUserSupplied = 0x1,
    EncryptMethodUserSupplied2 = 0x2,
    EncryptMethodXorBuffer = 0x3,
    EncryptMethodXor = 0x4,
}

#[wasm_bindgen]
#[derive(Debug)]
pub enum SoeOpcode {
    SessionRequest = 0x01,
    SessionReply = 0x02,
    MultiPacket = 0x03,
    Disconnect = 0x05,
    Ping = 0x06,
    NetStatusRequest = 0x07,
    NetStatusReply = 0x08,
    Data = 0x09,
    DataFragment = 0x0d,
    OutOfOrder = 0x11,
    Ack = 0x15,
    Group = 0x19,
    Ordered = 0x1B,
    FatalError = 0x1D,
    Unknown = 0x00,
}

impl Soeprotocol {
    // rust only
    pub fn get_opcode(&mut self, rdr: &mut Cursor<&Vec<u8>>) -> SoeOpcode {
        let opcode = rdr.read_u16::<BigEndian>().unwrap_or_default();
        match opcode {
            0x01 => SoeOpcode::SessionRequest,
            0x02 => SoeOpcode::SessionReply,
            0x03 => SoeOpcode::MultiPacket,
            0x05 => SoeOpcode::Disconnect,
            0x06 => SoeOpcode::Ping,
            0x07 => SoeOpcode::NetStatusRequest,
            0x08 => SoeOpcode::NetStatusReply,
            0x09 => SoeOpcode::Data,
            0x0d => SoeOpcode::DataFragment,
            0x11 => SoeOpcode::OutOfOrder,
            0x15 => SoeOpcode::Ack,
            0x1B => SoeOpcode::Ordered,
            0x1D => SoeOpcode::FatalError,
            _ => SoeOpcode::Unknown,
        }
    }
    pub fn get_session_request_object(
        &mut self,
        packet_string: String,
    ) -> Result<SessionRequestPacket, serde_json::Error> {
        serde_json::from_str(&packet_string)
    }

    pub fn pack_ordered_object(&mut self, packet: DataPacket) -> Vec<u8> {
        self._pack_data_object(SoeOpcode::Ordered as u16, packet)
    }

    pub fn pack_session_request_object(&mut self, packet: SessionRequestPacket) -> Vec<u8> {
        self.wtr.clear();
        self.wtr
            .write_u16::<BigEndian>(SoeOpcode::SessionRequest as u16)
            .unwrap_or_default();
        self.wtr
            .write_u32::<BigEndian>(packet.protocol_version)
            .unwrap_or_default();
        self.wtr
            .write_u32::<BigEndian>(packet.session_id)
            .unwrap_or_default();
        self.wtr
            .write_u32::<BigEndian>(packet.udp_length)
            .unwrap_or_default();
        self.wtr.append(&mut u8_from_str_nul_utf8_unchecked(
            packet.get_protocol().as_str(),
        ));
        self.wtr.clone()
    }

    pub fn get_session_reply_object(
        &mut self,
        packet_string: String,
    ) -> Result<SessionReplyPacket, serde_json::Error> {
        serde_json::from_str(&packet_string)
    }

    pub fn pack_session_reply_object(&mut self, packet: SessionReplyPacket) -> Vec<u8> {
        self.wtr.clear();
        self.wtr
            .write_u16::<BigEndian>(SoeOpcode::SessionReply as u16)
            .unwrap_or_default();
        self.wtr
            .write_u32::<BigEndian>(packet.session_id)
            .unwrap_or_default();
        self.wtr
            .write_u32::<BigEndian>(packet.crc_seed)
            .unwrap_or_default();
        self.wtr.write_u8(packet.crc_length).unwrap_or_default();
        self.wtr
            .write_u16::<BigEndian>(packet.encrypt_method)
            .unwrap_or_default();
        self.wtr
            .write_u32::<BigEndian>(packet.udp_length)
            .unwrap_or_default();
        self.wtr.write_u32::<BigEndian>(3).unwrap_or_default();
        self.wtr.clone()
    }

    pub fn get_net_status_request_object(
        &mut self,
        packet_string: String,
    ) -> Result<NetStatusRequestPacket, serde_json::Error> {
        serde_json::from_str(&packet_string)
    }

    pub fn pack_net_status_request_object(&mut self, packet: NetStatusRequestPacket) -> Vec<u8> {
        self.wtr.clear();
        self.wtr
            .write_u16::<BigEndian>(SoeOpcode::NetStatusRequest as u16)
            .unwrap_or_default();
        self.wtr
            .write_u16::<BigEndian>(packet.client_tick_count)
            .unwrap_or_default();
        self.wtr
            .write_u32::<BigEndian>(packet.last_client_update)
            .unwrap_or_default();
        self.wtr
            .write_u32::<BigEndian>(packet.average_update)
            .unwrap_or_default();
        self.wtr
            .write_u32::<BigEndian>(packet.shortest_update)
            .unwrap_or_default();
        self.wtr
            .write_u32::<BigEndian>(packet.longest_update)
            .unwrap_or_default();
        self.wtr
            .write_u32::<BigEndian>(packet.last_server_update)
            .unwrap_or_default();
        self.wtr
            .write_u64::<BigEndian>(packet.packets_sent)
            .unwrap_or_default();
        self.wtr
            .write_u64::<BigEndian>(packet.packets_received)
            .unwrap_or_default();
        self.wtr
            .write_u16::<BigEndian>(packet.unknown_field)
            .unwrap_or_default();
        self.wtr.clone()
    }

    pub fn get_net_status_reply_object(&mut self, packet_string: String) -> NetStatusReplyPacket {
        serde_json::from_str(&packet_string).unwrap_or({
            NetStatusReplyPacket {
                client_tick_count: 0,
                server_tick_count: 0,
                client_packet_sent: 0,
                client_packet_received: 0,
                server_packet_sent: 0,
                server_packet_received: 0,
                unknown_field: 0,
            }
        })
    }

    pub fn pack_net_status_reply_object(&mut self, packet: NetStatusReplyPacket) -> Vec<u8> {
        self.wtr.clear();
        self.wtr
            .write_u16::<BigEndian>(SoeOpcode::NetStatusReply as u16)
            .unwrap_or_default();
        self.wtr
            .write_u16::<BigEndian>(packet.client_tick_count)
            .unwrap_or_default();
        self.wtr
            .write_u32::<BigEndian>(packet.server_tick_count)
            .unwrap_or_default();
        self.wtr
            .write_u64::<BigEndian>(packet.client_packet_sent)
            .unwrap_or_default();
        self.wtr
            .write_u64::<BigEndian>(packet.client_packet_received)
            .unwrap_or_default();
        self.wtr
            .write_u64::<BigEndian>(packet.server_packet_sent)
            .unwrap_or_default();
        self.wtr
            .write_u64::<BigEndian>(packet.server_packet_received)
            .unwrap_or_default();
        self.wtr
            .write_u16::<BigEndian>(packet.unknown_field)
            .unwrap_or_default();
        self.wtr.clone()
    }

    pub fn get_multi_object(
        &mut self,
        packet_string: String,
    ) -> Result<SubBasePackets, serde_json::Error> {
        serde_json::from_str(&packet_string)
    }

    pub fn group_packets(&mut self, opcode: u16, packets: &Vec<Vec<u8>>) -> Vec<u8> {
        self.wtr.clear();
        self.wtr.write_u16::<BigEndian>(opcode).unwrap_or_default();
        for packet in packets {
            write_data_length(&mut self.wtr, packet.len());
            // FIXME: shitty clone
            let mut packet = packet.clone();
            self.wtr.append(&mut packet);
        }
        self.wtr.clone()
    }

    pub fn pack_group_object(&mut self, group_packet: SubBasePackets) -> Vec<u8> {
        self.group_packets(SoeOpcode::Group as u16, group_packet.get_sub_packets())
    }

    pub fn pack_multi_object(&mut self, multi_packet: SubBasePackets) -> Vec<u8> {
        self.group_packets(
            SoeOpcode::MultiPacket as u16,
            multi_packet.get_sub_packets(),
        )
    }

    pub fn get_data_object(
        &mut self,
        packet_string: String,
    ) -> Result<DataPacket, serde_json::Error> {
        serde_json::from_str(&packet_string)
    }

    fn _pack_data_object(&mut self, opcode: u16, mut packet: DataPacket) -> Vec<u8> {
        self.wtr.clear();
        self.wtr.write_u16::<BigEndian>(opcode).unwrap_or_default();
        write_packet_data(&mut self.wtr, &mut packet);
        self.wtr.clone()
    }

    pub fn pack_data_object(&mut self, packet: DataPacket) -> Vec<u8> {
        self._pack_data_object(0x09, packet)
    }

    pub fn pack_fragment_data_object(&mut self, packet: DataPacket) -> Vec<u8> {
        self._pack_data_object(0x0d, packet)
    }

    pub fn get_ack_object(
        &mut self,
        packet_string: String,
    ) -> Result<AckPacket, serde_json::Error> {
        serde_json::from_str(&packet_string)
    }

    fn _pack_ack_object(&mut self, opcode: u16, sequence: u16) -> Vec<u8> {
        self.wtr.clear();
        self.wtr.write_u16::<BigEndian>(opcode).unwrap_or_default();
        self.wtr
            .write_u16::<BigEndian>(sequence)
            .unwrap_or_default();
        self.wtr.clone()
    }

    pub fn pack_out_of_order_object(&mut self, packet: AckPacket) -> Vec<u8> {
        self._pack_ack_object(0x11, packet.sequence)
    }

    pub fn pack_ack_object(&mut self, packet: AckPacket) -> Vec<u8> {
        self._pack_ack_object(0x15, packet.sequence)
    }
}
#[wasm_bindgen]
impl Soeprotocol {
    // wasm lib
    #[wasm_bindgen(constructor)]
    pub fn initialize(use_crc: bool, crc_seed: u32) -> Soeprotocol {
        Soeprotocol {
            use_crc,
            crc_seed,
            wtr: vec![],
        }
    }
    pub fn pack(&mut self, packet_opcode: SoeOpcode, packet: String) -> Vec<u8> {
        match packet_opcode {
            SoeOpcode::SessionRequest => self.pack_session_request(packet),
            SoeOpcode::SessionReply => self.pack_session_reply(packet),
            SoeOpcode::MultiPacket => self.pack_multi(packet),
            SoeOpcode::Group => self.pack_group(packet),
            SoeOpcode::Disconnect => vec![0, 5],
            SoeOpcode::Ping => vec![0, 6],
            SoeOpcode::NetStatusRequest => self.pack_net_status_request(packet),
            SoeOpcode::NetStatusReply => self.pack_net_status_reply(packet),
            SoeOpcode::Data => self.pack_data(packet),
            SoeOpcode::DataFragment => self.pack_fragment_data(packet),
            SoeOpcode::OutOfOrder => self.pack_out_of_order(packet),
            SoeOpcode::Ack => self.pack_ack(packet),
            SoeOpcode::Ordered => self.pack_ordered(packet),
            SoeOpcode::FatalError => vec![],
            SoeOpcode::Unknown => vec![],
        }
    }

    pub fn pack_ordered(&mut self, packet: String) -> Vec<u8> {
        let packet_object: Result<DataPacket, serde_json::Error> = self.get_data_object(packet);
        if let Ok(packet_object) = packet_object {
            self.pack_ordered_object(packet_object)
        } else {
            gen_deserializing_error_json(packet_object.err().unwrap())
        }
    }

    pub fn pack_ordered_packet(&mut self, data: Vec<u8>, sequence: u16) -> Vec<u8> {
        self.pack_ordered_object(DataPacket::new(data, sequence))
    }

    pub fn pack_session_request(&mut self, packet: String) -> Vec<u8> {
        let packet_object: Result<SessionRequestPacket, serde_json::Error> =
            self.get_session_request_object(packet);
        if let Ok(packet_object) = packet_object {
            self.pack_session_request_object(packet_object)
        } else {
            gen_deserializing_error_json(packet_object.err().unwrap())
        }
    }

    pub fn pack_session_request_fromjs(&mut self, js_object: &JsValue) -> Vec<u8> {
        let packet: SessionRequestPacket = js_object.into_serde().unwrap();
        self.pack_session_request_object(packet)
    }

    pub fn pack_session_request_packet(
        &mut self,
        session_id: u32,
        crc_length: u32,
        udp_length: u32,
        protocol: String,
    ) -> Vec<u8> {
        self.pack_session_request_object(SessionRequestPacket::new(
            session_id, crc_length, udp_length, protocol,
        ))
    }

    pub fn pack_session_reply(&mut self, packet: String) -> Vec<u8> {
        let packet_object: Result<SessionReplyPacket, serde_json::Error> =
            self.get_session_reply_object(packet);
        if let Ok(packet_object) = packet_object {
            self.pack_session_reply_object(packet_object)
        } else {
            gen_deserializing_error_json(packet_object.err().unwrap())
        }
    }

    pub fn pack_session_reply_fromjs(&mut self, js_object: &JsValue) -> Vec<u8> {
        let packet: SessionReplyPacket = js_object.into_serde().unwrap();
        self.pack_session_reply_object(packet)
    }

    pub fn pack_session_reply_packet(
        &mut self,
        session_id: u32,
        crc_seed: u32,
        crc_length: u8,
        encrypt_method: u16,
        udp_length: u32,
    ) -> Vec<u8> {
        self.pack_session_reply_object(SessionReplyPacket {
            session_id,
            crc_seed,
            crc_length,
            encrypt_method,
            udp_length,
        })
    }

    pub fn pack_net_status_request(&mut self, packet: String) -> Vec<u8> {
        let packet_object: Result<NetStatusRequestPacket, serde_json::Error> =
            self.get_net_status_request_object(packet);
        if let Ok(packet_object) = packet_object {
            self.pack_net_status_request_object(packet_object)
        } else {
            gen_deserializing_error_json(packet_object.err().unwrap())
        }
    }

    pub fn pack_net_status_request_fromjs(&mut self, js_object: &JsValue) -> Vec<u8> {
        let packet: NetStatusRequestPacket = js_object.into_serde().unwrap();
        self.pack_net_status_request_object(packet)
    }

    pub fn pack_net_status_reply(&mut self, packet: String) -> Vec<u8> {
        let packet_object: NetStatusReplyPacket = self.get_net_status_reply_object(packet);
        self.pack_net_status_reply_object(packet_object)
    }

    pub fn pack_net_status_reply_fromjs(&mut self, js_object: &JsValue) -> Vec<u8> {
        let packet: NetStatusReplyPacket = js_object.into_serde().unwrap();
        self.pack_net_status_reply_object(packet)
    }

    pub fn pack_multi(&mut self, packet: String) -> Vec<u8> {
        let multi_packets: Result<SubBasePackets, serde_json::Error> =
            self.get_multi_object(packet);
        if let Ok(multi_packets) = multi_packets {
            self.pack_multi_object(multi_packets)
        } else {
            gen_deserializing_error_json(multi_packets.err().unwrap())
        }
    }

    pub fn pack_multi_fromjs(&mut self, js_object: &JsValue) -> Vec<u8> {
        let packet: SubBasePackets = js_object.into_serde().unwrap();
        self.pack_multi_object(packet)
    }

    pub fn pack_group(&mut self, packet: String) -> Vec<u8> {
        let group_packets: Result<SubBasePackets, serde_json::Error> =
            self.get_multi_object(packet);
        if let Ok(group_packets) = group_packets {
            self.pack_group_object(group_packets)
        } else {
            gen_deserializing_error_json(group_packets.err().unwrap())
        }
    }

    pub fn pack_group_fromjs(&mut self, js_object: &JsValue) -> Vec<u8> {
        let packet: SubBasePackets = js_object.into_serde().unwrap();
        self.pack_group_object(packet)
    }

    pub fn pack_data(&mut self, packet: String) -> Vec<u8> {
        let packet_object: Result<DataPacket, serde_json::Error> = self.get_data_object(packet);
        if let Ok(packet_object) = packet_object {
            self.pack_data_object(packet_object)
        } else {
            gen_deserializing_error_json(packet_object.err().unwrap())
        }
    }

    pub fn pack_data_fromjs(&mut self, js_object: &JsValue) -> Vec<u8> {
        let packet: DataPacket = js_object.into_serde().unwrap();
        self.pack_data_object(packet)
    }

    pub fn pack_data_packet(&mut self, data: Vec<u8>, sequence: u16) -> Vec<u8> {
        self.pack_data_object(DataPacket::new(data, sequence))
    }

    pub fn pack_fragment_data(&mut self, packet: String) -> Vec<u8> {
        let packet_object: Result<DataPacket, serde_json::Error> = self.get_data_object(packet);
        if let Ok(packet_object) = packet_object {
            self.pack_fragment_data_object(packet_object)
        } else {
            gen_deserializing_error_json(packet_object.err().unwrap())
        }
    }

    pub fn pack_fragment_data_fromjs(&mut self, js_object: &JsValue) -> Vec<u8> {
        let packet: DataPacket = js_object.into_serde().unwrap();
        self.pack_fragment_data_object(packet)
    }

    pub fn pack_fragment_data_packet(&mut self, data: Vec<u8>, sequence: u16) -> Vec<u8> {
        self.pack_fragment_data_object(DataPacket::new(data, sequence))
    }

    pub fn pack_out_of_order(&mut self, packet: String) -> Vec<u8> {
        let packet_object: Result<AckPacket, serde_json::Error> = self.get_ack_object(packet);
        if let Ok(packet_object) = packet_object {
            self.pack_out_of_order_object(packet_object)
        } else {
            gen_deserializing_error_json(packet_object.err().unwrap())
        }
    }

    pub fn pack_out_of_order_fromjs(&mut self, js_object: &JsValue) -> Vec<u8> {
        let packet: AckPacket = js_object.into_serde().unwrap();
        self.pack_out_of_order_object(packet)
    }

    pub fn pack_out_of_order_packet(&mut self, sequence: u16) -> Vec<u8> {
        self.pack_out_of_order_object(AckPacket { sequence })
    }

    pub fn pack_ack(&mut self, packet: String) -> Vec<u8> {
        let packet_object: Result<AckPacket, serde_json::Error> = self.get_ack_object(packet);
        if let Ok(packet_object) = packet_object {
            self.pack_ack_object(packet_object)
        } else {
            gen_deserializing_error_json(packet_object.err().unwrap())
        }
    }

    pub fn pack_ack_fromjs(&mut self, js_object: &JsValue) -> Vec<u8> {
        let packet: AckPacket = js_object.into_serde().unwrap();
        self.pack_ack_object(packet)
    }

    pub fn pack_ack_packet(&mut self, sequence: u16) -> Vec<u8> {
        self.pack_ack_object(AckPacket { sequence })
    }

    pub fn parse(&mut self, data: Vec<u8>) -> String {
        let mut rdr = Cursor::new(&data);
        let opcode: SoeOpcode = if data.len() >= 2 {
            self.get_opcode(&mut rdr)
        } else {
            SoeOpcode::Unknown
        };

        match opcode {
            SoeOpcode::SessionRequest => self.parse_session_request(rdr),
            SoeOpcode::SessionReply => self.parse_session_reply(rdr),
            SoeOpcode::MultiPacket => self.parse_multi(rdr),
            SoeOpcode::Group => self.parse_multi(rdr),
            SoeOpcode::Disconnect => self.parse_disconnect(rdr),
            SoeOpcode::Ping => r#"{"name":"Ping"}"#.to_string(),
            SoeOpcode::NetStatusRequest => self.parse_net_status_request(rdr),
            SoeOpcode::NetStatusReply => self.parse_net_status_reply(rdr),
            SoeOpcode::Data => self.parse_data(rdr, opcode as u16),
            SoeOpcode::DataFragment => self.parse_data(rdr, opcode as u16),
            SoeOpcode::OutOfOrder => self.parse_ack(rdr, opcode as u16),
            SoeOpcode::Ack => self.parse_ack(rdr, opcode as u16),
            SoeOpcode::Ordered => self.parse_ordered(rdr),
            SoeOpcode::FatalError => format!(r#"{{"name":"FatalError","raw":{:?}}}"#, data),
            SoeOpcode::Unknown => format!(r#"{{"name":"Unknown","raw":{:?}}}"#, data),
        }
    }

    fn parse_ordered(&mut self, mut rdr: Cursor<&std::vec::Vec<u8>>) -> String {
        if !check_min_size(&rdr, PacketsMinSize::DataPacket as usize, self.use_crc) {
            return gen_size_error_json(rdr);
        }
        let order = rdr.read_u16::<BigEndian>().unwrap_or_default();
        let data_end: u64 = get_data_end(&rdr, self.use_crc);
        let mut crc: u16 = 0;
        if self.use_crc {
            rdr.set_position(data_end);
            crc = rdr.read_u16::<BigEndian>().unwrap_or_default();
        }
        let vec = rdr.get_ref().to_vec();
        let data = &vec[4..data_end as usize];
        // check that crc value is correct
        if self.use_crc {
            let packet_without_crc = &vec[0..data_end as usize];
            let crc_value =
                (crc32(&&mut packet_without_crc.to_vec(), self.crc_seed as usize) & 0xffff) as u16;
            if crc_value != crc {
                return gen_crc_error_json(&vec, crc_value, crc);
            }
        }
        format!(
            r#"{{"name":"Ordered","order":{},"data":{:?}}}"#,
            order, data
        )
    }
    fn parse_session_request(&mut self, mut rdr: Cursor<&std::vec::Vec<u8>>) -> String {
        if !check_min_size(&rdr, PacketsMinSize::SessionRequest as usize, false) {
            return gen_size_error_json(rdr);
        }

        let crc_length = rdr.read_u32::<BigEndian>().unwrap_or_default();
        let session_id = rdr.read_u32::<BigEndian>().unwrap_or_default();
        let udp_length = rdr.read_u32::<BigEndian>().unwrap_or_default();
        let protocol_data_position = rdr.position() as usize;
        let raw_data = rdr.into_inner();
        let protocol = str_from_u8_nul_utf8_checked(&raw_data[protocol_data_position..]);
        format!(
            r#"{{"name":"SessionRequest","protocol_version":{},"session_id":{},"udp_length":{},"protocol":"{}"}}"#,
            crc_length, session_id, udp_length, protocol
        )
    }

    fn parse_session_reply(&mut self, mut rdr: Cursor<&std::vec::Vec<u8>>) -> String {
        if rdr.get_ref().len() != PacketsMinSize::SessionReply as usize {
            return gen_size_error_json(rdr);
        }
        let session_id = rdr.read_u32::<BigEndian>().unwrap_or_default();
        let crc_seed = rdr.read_u32::<BigEndian>().unwrap_or_default();
        let crc_length = rdr.read_u8().unwrap_or_default();
        let encrypt_method = rdr.read_u16::<BigEndian>().unwrap_or_default();
        let udp_length = rdr.read_u32::<BigEndian>().unwrap_or_default();
        format!(
            r#"{{"name":"SessionReply","session_id":{},"crc_seed":{},"crc_length":{},"encrypt_method":{},"udp_length":{}}}"#,
            session_id, crc_seed, crc_length, encrypt_method, udp_length
        )
    }

    fn parse_disconnect(&mut self, mut rdr: Cursor<&std::vec::Vec<u8>>) -> String {
        if rdr.get_ref().len() < PacketsMinSize::Disconnect as usize {
            return r#"{"name":"Disconnect" ,"session_id":null,"reason":"unknown"}"#.to_string();
        }
        let session_id = rdr.read_u32::<BigEndian>().unwrap_or_default();
        let reason = disconnect_reason_to_string(rdr.read_u16::<BigEndian>().unwrap_or_default());
        format!(
            r#"{{"name":"Disconnect" ,"session_id":{},"reason":"{}"}}"#,
            session_id, reason
        )
    }

    fn parse_net_status_request(&mut self, mut rdr: Cursor<&std::vec::Vec<u8>>) -> String {
        if rdr.get_ref().len() != PacketsMinSize::NetStatusPacket as usize {
            return gen_size_error_json(rdr);
        }
        let client_tick_count = rdr.read_u16::<BigEndian>().unwrap_or_default();
        let last_client_update = rdr.read_u32::<BigEndian>().unwrap_or_default();
        let average_update = rdr.read_u32::<BigEndian>().unwrap_or_default();
        let shortest_update = rdr.read_u32::<BigEndian>().unwrap_or_default();
        let longest_update = rdr.read_u32::<BigEndian>().unwrap_or_default();
        let last_server_update = rdr.read_u32::<BigEndian>().unwrap_or_default();
        let packets_sent = rdr.read_u64::<BigEndian>().unwrap_or_default();
        let packets_received = rdr.read_u64::<BigEndian>().unwrap_or_default();
        let unknown_field = rdr.read_u16::<BigEndian>().unwrap_or_default();
        format!(
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
        )
    }

    fn parse_net_status_reply(&mut self, mut rdr: Cursor<&std::vec::Vec<u8>>) -> String {
        if rdr.get_ref().len() != PacketsMinSize::NetStatusPacket as usize {
            return gen_size_error_json(rdr);
        }
        let client_tick_count = rdr.read_u16::<BigEndian>().unwrap_or_default();
        let server_tick_count = rdr.read_u32::<BigEndian>().unwrap_or_default();
        let client_packet_sent = rdr.read_u64::<BigEndian>().unwrap_or_default();
        let client_packet_received = rdr.read_u64::<BigEndian>().unwrap_or_default();
        let server_packet_sent = rdr.read_u64::<BigEndian>().unwrap_or_default();
        let server_packet_received = rdr.read_u64::<BigEndian>().unwrap_or_default();
        let unknown_field = rdr.read_u16::<BigEndian>().unwrap_or_default();
        format!(
            r#"{{"name":"NetStatusReply","client_tick_count":{},"server_tick_count":{},"client_packet_sent":{},"client_packet_received":{},"server_packet_sent":{},"server_packet_received":{},"unknown_field":{}}}"#,
            client_tick_count,
            server_tick_count,
            client_packet_sent,
            client_packet_received,
            server_packet_sent,
            server_packet_received,
            unknown_field
        )
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
            rdr.set_position(data_end);
            let crc: u16 = rdr.read_u16::<BigEndian>().unwrap_or_default();
            let vec = rdr.clone().into_inner();
            let packet_without_crc = &vec[0..data_end as usize];
            let crc_value =
                (crc32(&&mut packet_without_crc.to_vec(), self.crc_seed as usize) & 0xffff) as u16;
            if crc_value != crc {
                return gen_crc_error_json(vec, crc_value, crc);
            }
            rdr.set_position(2); // reset pos after the opcode
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
                multi_result.push(',');
            }
        }
        multi_result.push_str("]}");
        if was_crc_enabled {
            self.enable_crc();
        }
        multi_result
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
        let sequence = rdr.read_u16::<BigEndian>().unwrap_or_default();

        let data_end: u64 = get_data_end(&rdr, self.use_crc);
        let mut crc: u16 = 0;
        if self.use_crc {
            rdr.set_position(data_end);
            crc = rdr.read_u16::<BigEndian>().unwrap_or_default();
        }
        let vec = rdr.get_ref().to_vec();
        let data = &vec[4..data_end as usize];
        // check that crc value is correct
        if self.use_crc {
            let packet_without_crc = &vec[0..data_end as usize];
            let crc_value =
                (crc32(&&mut packet_without_crc.to_vec(), self.crc_seed as usize) & 0xffff) as u16;
            if crc_value != crc {
                return gen_crc_error_json(&vec, crc_value, crc);
            }
        }
        format!(
            r#"{{"name":"{}","sequence":{},"data":{:?}}}"#,
            name, sequence, data
        )
    }

    fn parse_ack(&mut self, mut rdr: Cursor<&std::vec::Vec<u8>>, opcode: u16) -> String {
        if !check_min_size(&rdr, PacketsMinSize::Ack as usize, self.use_crc) {
            return gen_size_error_json(rdr);
        }
        let name = if opcode == 0x15 { "Ack" } else { "OutOfOrder" };
        let sequence = rdr.read_u16::<BigEndian>().unwrap_or_default();
        if self.use_crc {
            let crc = rdr.read_u16::<BigEndian>().unwrap_or_default();
            let data_end: u64 = get_data_end(&rdr, self.use_crc);
            let vec = rdr.into_inner();
            let packet_without_crc = &vec[0..data_end as usize];
            let crc_value =
                (crc32(&&mut packet_without_crc.to_vec(), self.crc_seed as usize) & 0xffff) as u16;
            if crc_value != crc {
                return gen_crc_error_json(vec, crc_value, crc);
            }
        }
        format!(r#"{{"name":"{}","sequence":{}}}"#, name, sequence)
    }

    pub fn get_crc_seed(&self) -> u32 {
        self.crc_seed
    }
    pub fn set_crc_seed(&mut self, crc_seed: u32) {
        self.crc_seed = crc_seed;
    }
    pub fn is_using_crc(&mut self) -> bool {
        self.use_crc
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

    use super::super::crc::append_crc;

    use super::super::protocol_errors::*;
    use super::*;

    #[test]
    fn session_request_parse_test() {
        let mut soeprotocol_class = super::Soeprotocol::initialize(true, 0);
        let data_to_parse: [u8; 25] = [
            0, 1, 0, 0, 0, 3, 60, 23, 140, 99, 0, 0, 2, 0, 76, 111, 103, 105, 110, 85, 100, 112,
            95, 57, 0,
        ];
        let data_parsed: serde_json::Value =
            serde_json::from_str(&soeprotocol_class.parse(data_to_parse.to_vec()))
                .unwrap_or_default();
        let succesful_data:serde_json::Value = serde_json::from_str(r#"{"name":"SessionRequest","protocol_version":3,"session_id":1008176227,"udp_length":512,"protocol":"LoginUdp_9"}"#).unwrap_or_default();
        assert_eq!(data_parsed, succesful_data)
    }

    #[test]
    fn session_request_parse_size_error_test() {
        let mut soeprotocol_class = super::Soeprotocol::initialize(true, 0);
        let data_to_parse: [u8; 12] = [0, 1, 111, 103, 105, 110, 85, 100, 112, 95, 57, 2];
        let data_parsed: serde_json::Value =
            serde_json::from_str(&soeprotocol_class.parse(data_to_parse.to_vec()))
                .unwrap_or_default();
        let succesful_data:serde_json::Value = serde_json::from_str(r#"{"error": "size", "name": "Error", "raw": [0, 1, 111, 103, 105, 110, 85, 100, 112,95, 57, 2], "size": 12}"#).unwrap_or_default();
        assert_eq!(data_parsed, succesful_data)
    }

    #[test]
    fn session_request_pack_test() {
        let mut soeprotocol_class = super::Soeprotocol::initialize(true, 0);
        let data_to_pack =
            r#"{"protocol_version":3,"session_id":1008176227,"udp_length":512,"protocol":"LoginUdp_9"}"#
                .to_string();
        let data_pack: Vec<u8> = soeprotocol_class.pack(SoeOpcode::SessionRequest, data_to_pack);
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
        let mut soeprotocol_class = super::Soeprotocol::initialize(false, 0);
        let data_to_pack =
            r#"{"protocol_version":3,"udp_length":512,"protocol":"LoginUdp_9"}"#.to_string();
        let data_pack: Vec<u8> = soeprotocol_class.pack(SoeOpcode::SessionRequest, data_to_pack);
        let data_pack_opcode: u16 = u16::from_be_bytes([data_pack[0], data_pack[1]]);
        assert_eq!(data_pack_opcode, ErrorType::Deserializing as u16)
    }

    #[test]
    fn session_reply_parse_size_error_test() {
        let mut soeprotocol_class = super::Soeprotocol::initialize(true, 0);
        let data_to_parse: [u8; 10] = [0, 2, 111, 103, 105, 110, 85, 100, 112, 95];
        let data_parsed: serde_json::Value =
            serde_json::from_str(&soeprotocol_class.parse(data_to_parse.to_vec()))
                .unwrap_or_default();
        let succesful_data:serde_json::Value = serde_json::from_str(r#"{"error": "size", "name": "Error", "raw": [0,2,111,103,105,110,85,100,112,95], "size": 10}"#).unwrap_or_default();
        assert_eq!(data_parsed, succesful_data)
    }

    #[test]
    fn session_reply_parse_test() {
        let mut soeprotocol_class = super::Soeprotocol::initialize(true, 0);
        let data_to_parse: [u8; 21] = [
            0, 2, 60, 23, 140, 99, 0, 0, 0, 0, 2, 1, 0, 0, 0, 2, 0, 0, 0, 0, 3,
        ];
        let data_parsed: serde_json::Value =
            serde_json::from_str(&soeprotocol_class.parse(data_to_parse.to_vec()))
                .unwrap_or_default();
        let succesful_data:serde_json::Value = serde_json::from_str(r#"{"name":"SessionReply","session_id":1008176227,"crc_seed":0,"crc_length":2,"encrypt_method":256,"udp_length":512}"#).unwrap_or_default();
        assert_eq!(data_parsed, succesful_data)
    }

    #[test]
    fn session_reply_pack_test() {
        let mut soeprotocol_class = super::Soeprotocol::initialize(true, 0);
        let data_to_pack =  r#"{"session_id":1008176227,"crc_seed":0,"crc_length":2,"encrypt_method":256,"udp_length":512}"#.to_string();
        let data_pack: Vec<u8> = soeprotocol_class.pack(SoeOpcode::SessionReply, data_to_pack);
        assert_eq!(
            data_pack,
            [0, 2, 60, 23, 140, 99, 0, 0, 0, 0, 2, 1, 0, 0, 0, 2, 0, 0, 0, 0, 3]
        )
    }

    #[test]
    fn session_reply_pack_test_deserializing_error() {
        let mut soeprotocol_class = super::Soeprotocol::initialize(false, 0);
        let data_to_pack =
            r#"{"crc_seed":0,"crc_length":2,"encrypt_method":256,"udp_length":512}"#.to_string();
        let data_pack: Vec<u8> = soeprotocol_class.pack(SoeOpcode::SessionReply, data_to_pack);
        let data_pack_opcode: u16 = u16::from_be_bytes([data_pack[0], data_pack[1]]);
        assert_eq!(data_pack_opcode, ErrorType::Deserializing as u16)
    }

    #[test]
    fn net_status_request_parse_test() {
        let mut soeprotocol_class = super::Soeprotocol::initialize(true, 0);
        let data_to_parse: [u8; 42] = [
            0, 7, 251, 92, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 1, 235, 216,
        ];
        let data_parsed: serde_json::Value =
            serde_json::from_str(&soeprotocol_class.parse(data_to_parse.to_vec()))
                .unwrap_or_default();
        let succesful_data:serde_json::Value = serde_json::from_str(r#"{"average_update": 0, "client_tick_count": 64348, "last_client_update": 0, "last_server_update": 0, "longest_update": 0, "name": "NetStatusRequest", "packets_received": 1, "packets_sent": 2, "shortest_update": 0, "unknown_field": 60376}"#).unwrap_or_default();
        assert_eq!(data_parsed, succesful_data)
    }

    #[test]
    fn net_status_request_parse_deserializing_error() {
        let mut soeprotocol_class = super::Soeprotocol::initialize(true, 0);
        let data_to_parse: [u8; 43] = [
            0, 7, 251, 92, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 1, 235, 216, 0,
        ];
        let data_parsed: serde_json::Value =
            serde_json::from_str(&soeprotocol_class.parse(data_to_parse.to_vec()))
                .unwrap_or_default();
        let succesful_data:serde_json::Value = serde_json::from_str(r#"{"error": "size", "name": "Error", "raw": [0,7,251,92,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2,0,0,0,0,0,0,0,1,235,216,0], "size": 43}"#).unwrap_or_default();
        assert_eq!(data_parsed, succesful_data)
    }

    #[test]
    fn net_status_request_pack_test() {
        let mut soeprotocol_class = super::Soeprotocol::initialize(false, 0);
        let data_to_pack =
        r#"{"average_update": 0, "client_tick_count": 64348, "last_client_update": 0, "last_server_update": 0, "longest_update": 0, "name": "NetStatusRequest", "packets_received": 1, "packets_sent": 2, "shortest_update": 0, "unknown_field": 60376}"#
                .to_string();
        let data_pack: Vec<u8> = soeprotocol_class.pack(SoeOpcode::NetStatusRequest, data_to_pack);
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
        let mut soeprotocol_class = super::Soeprotocol::initialize(false, 0);
        let data_to_pack =
        r#"{"client_packet_received": 0, "client_packet_sent": 0, "client_tick_count": 64348, "name": "NetStatusRequest", "server_packet_received": 1, "server_packet_sent": 2, "server_tick_count": 0}"#.to_string();
        let data_pack: Vec<u8> = soeprotocol_class.pack(SoeOpcode::NetStatusRequest, data_to_pack);
        let data_pack_opcode: u16 = u16::from_be_bytes([data_pack[0], data_pack[1]]);
        assert_eq!(data_pack_opcode, ErrorType::Deserializing as u16)
    }

    #[test]
    fn net_status_reply_parse_test() {
        let mut soeprotocol_class = super::Soeprotocol::initialize(true, 0);
        let data_to_parse: [u8; 42] = [
            0, 8, 251, 92, 33, 39, 197, 60, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0,
            0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 2, 131, 212,
        ];
        let data_parsed: serde_json::Value =
            serde_json::from_str(&soeprotocol_class.parse(data_to_parse.to_vec()))
                .unwrap_or_default();
        let succesful_data:serde_json::Value = serde_json::from_str(r#"{"client_packet_received": 1, "client_packet_sent": 2, "client_tick_count": 64348, "name": "NetStatusReply", "server_packet_received": 2, "server_packet_sent": 1, "server_tick_count": 556254524, "unknown_field": 33748}"#).unwrap_or_default();
        assert_eq!(data_parsed, succesful_data)
    }

    #[test]
    fn net_status_reply_parse_size_error() {
        let mut soeprotocol_class = super::Soeprotocol::initialize(true, 0);
        let data_to_parse: [u8; 43] = [
            0, 8, 251, 92, 33, 39, 197, 60, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0,
            0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 2, 131, 212, 0,
        ];
        let data_parsed: serde_json::Value =
            serde_json::from_str(&soeprotocol_class.parse(data_to_parse.to_vec()))
                .unwrap_or_default();
        let succesful_data:serde_json::Value = serde_json::from_str(r#"{"error": "size", "name": "Error", "raw": [0,8,251,92,33,39,197,60,0,0,0,0,0,0,0,2,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,2,131,212,0], "size": 43}"#).unwrap_or_default();
        assert_eq!(data_parsed, succesful_data)
    }

    #[test]
    fn net_status_reply_pack_test() {
        let mut soeprotocol_class = super::Soeprotocol::initialize(false, 0);
        let data_to_pack =
        r#"{"client_packet_received": 1, "client_packet_sent": 2, "client_tick_count": 64348, "name": "NetStatusReply", "server_packet_received": 2, "server_packet_sent": 1, "server_tick_count": 556254524, "unknown_field": 33748}"#
                .to_string();
        let data_pack: Vec<u8> = soeprotocol_class.pack(SoeOpcode::NetStatusReply, data_to_pack);
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
        let mut soeprotocol_class = super::Soeprotocol::initialize(false, 0);
        let data_to_pack =
        r#"{"client_packet_received": 0, "client_packet_sent": 0, "client_tick_cozunt": 64348, "name": "NetStatusRequest", "server_packet_received": 1, "server_packet_sent": 2, "server_tick_count": 0}"#.to_string();
        let data_pack: Vec<u8> = soeprotocol_class.pack(SoeOpcode::NetStatusRequest, data_to_pack);
        let data_pack_opcode: u16 = u16::from_be_bytes([data_pack[0], data_pack[1]]);
        assert_eq!(data_pack_opcode, ErrorType::Deserializing as u16)
    }

    #[test]
    fn ping_parse_test() {
        let mut soeprotocol_class = super::Soeprotocol::initialize(true, 0);
        let data_to_parse: [u8; 2] = [0, 6];
        let data_parsed: String = soeprotocol_class.parse(data_to_parse.to_vec());
        assert_eq!(data_parsed, r#"{"name":"Ping"}"#)
    }

    #[test]
    fn ping_pack_test() {
        let mut soeprotocol_class = super::Soeprotocol::initialize(true, 0);
        let data_to_pack: String = r#"{"name":"Ping"}"#.to_owned();
        let data_pack: Vec<u8> = soeprotocol_class.pack(SoeOpcode::Ping, data_to_pack);
        assert_eq!(data_pack, [0, 6])
    }

    #[test]
    fn outoforder_parse_size_error_test() {
        let mut soeprotocol_class = super::Soeprotocol::initialize(true, 0);
        let data_to_parse: [u8; 3] = [0, 17, 111];
        let data_parsed: serde_json::Value =
            serde_json::from_str(&soeprotocol_class.parse(data_to_parse.to_vec()))
                .unwrap_or_default();
        let succesful_data: serde_json::Value = serde_json::from_str(
            r#"{"error": "size", "name": "Error", "raw": [0, 17, 111], "size": 3}"#,
        )
        .unwrap_or_default();
        assert_eq!(data_parsed, succesful_data)
    }

    #[test]
    fn outoforder_parse_test() {
        let mut soeprotocol_class = super::Soeprotocol::initialize(true, 0);
        let data_to_parse: [u8; 6] = [0, 17, 0, 1, 38, 184];
        let data_parsed: serde_json::Value =
            serde_json::from_str(&soeprotocol_class.parse(data_to_parse.to_vec()))
                .unwrap_or_default();
        let succesful_data: serde_json::Value =
            serde_json::from_str(r#"{"name":"OutOfOrder","sequence":1}"#).unwrap_or_default();
        assert_eq!(data_parsed, succesful_data)
    }

    #[test]
    fn outoforder_parse_test_crc_fail() {
        let mut soeprotocol_class = super::Soeprotocol::initialize(true, 0);
        let data_to_parse: [u8; 6] = [0, 17, 0, 1, 142, 100];
        let data_parsed: serde_json::Value =
            serde_json::from_str(&soeprotocol_class.parse(data_to_parse.to_vec()))
                .unwrap_or_default();
        let succesful_data:serde_json::Value = serde_json::from_str(r#"{"error": "crc", "expected_crc": 9912, "given_crc": 36452, "name": "Error", "raw": [0, 17, 0, 1, 142, 100]}"#).unwrap_or_default();
        assert_eq!(data_parsed, succesful_data)
    }

    #[test]
    fn outoforder_pack_test() {
        let mut soeprotocol_class = super::Soeprotocol::initialize(false, 0);
        let data_to_pack: String = r#"{"name":"OutOfOrder","sequence":1}"#.to_owned();
        let data_pack: Vec<u8> = soeprotocol_class.pack(SoeOpcode::OutOfOrder, data_to_pack);
        assert_eq!(data_pack, [0, 17, 0, 1])
    }

    #[test]
    fn outoforder_pack_test_deserializing_error() {
        let mut soeprotocol_class = super::Soeprotocol::initialize(false, 0);
        let data_to_pack: String = r#"{"sequednce":1}"#.to_owned();
        let data_pack: Vec<u8> = soeprotocol_class.pack(SoeOpcode::OutOfOrder, data_to_pack);
        let data_pack_opcode: u16 = u16::from_be_bytes([data_pack[0], data_pack[1]]);
        assert_eq!(data_pack_opcode, ErrorType::Deserializing as u16)
    }

    #[test]
    fn ack_parse_size_error_test() {
        let mut soeprotocol_class = super::Soeprotocol::initialize(true, 0);
        let data_to_parse: [u8; 3] = [0, 21, 111];
        let data_parsed: serde_json::Value =
            serde_json::from_str(&soeprotocol_class.parse(data_to_parse.to_vec()))
                .unwrap_or_default();
        let succesful_data: serde_json::Value = serde_json::from_str(
            r#"{"error": "size", "name": "Error", "raw": [0, 21, 111], "size": 3}"#,
        )
        .unwrap_or_default();
        assert_eq!(data_parsed, succesful_data)
    }

    #[test]
    fn ack_parse_test() {
        let mut soeprotocol_class = super::Soeprotocol::initialize(true, 0);
        let data_to_parse: [u8; 6] = [0, 21, 0, 1, 142, 100];
        let data_parsed: serde_json::Value =
            serde_json::from_str(&soeprotocol_class.parse(data_to_parse.to_vec()))
                .unwrap_or_default();
        let succesful_data: serde_json::Value =
            serde_json::from_str(r#"{"name":"Ack","sequence":1}"#).unwrap_or_default();
        assert_eq!(data_parsed, succesful_data)
    }

    #[test]
    fn ack_pack_test() {
        let mut soeprotocol_class = super::Soeprotocol::initialize(false, 0);
        let data_to_pack: String = r#"{"name":"Ack","sequence":1}"#.to_owned();
        let data_pack: Vec<u8> = soeprotocol_class.pack(SoeOpcode::Ack, data_to_pack);
        assert_eq!(data_pack, [0, 21, 0, 1])
    }

    #[test]
    fn ack_pack_test_deserializing_error() {
        let mut soeprotocol_class = super::Soeprotocol::initialize(false, 0);
        let data_to_pack: String = r#"{"name":"Ack"}"#.to_owned();
        let data_pack: Vec<u8> = soeprotocol_class.pack(SoeOpcode::Ack, data_to_pack);
        let data_pack_opcode: u16 = u16::from_be_bytes([data_pack[0], data_pack[1]]);
        assert_eq!(data_pack_opcode, ErrorType::Deserializing as u16)
    }

    #[test]
    fn multi_parse_size_error_test() {
        let mut soeprotocol_class = super::Soeprotocol::initialize(true, 0);
        let data_to_parse: [u8; 4] = [0, 3, 4, 0];
        let data_parsed: serde_json::Value =
            serde_json::from_str(&soeprotocol_class.parse(data_to_parse.to_vec()))
                .unwrap_or_default();
        let succesful_data: serde_json::Value = serde_json::from_str(
            r#"{"error": "size", "name": "Error", "raw": [0, 3, 4, 0], "size": 4}"#,
        )
        .unwrap_or_default();
        assert_eq!(data_parsed, succesful_data)
    }

    #[test]
    fn multi_parse_corrupted_test() {
        let mut soeprotocol_class = super::Soeprotocol::initialize(false, 0);
        let data_to_parse: [u8; 75] = [
            0, 3, 54, 0, 21, 0, 206, 67, 0, 9, 0, 1, 0, 25, 41, 141, 45, 189, 85, 241, 64, 165, 71,
            228, 114, 81, 54, 5, 184, 205, 104, 0, 125, 184, 210, 74, 0, 247, 152, 225, 169, 102,
            204, 158, 233, 202, 228, 34, 202, 238, 136, 31, 3, 121, 222, 106, 11, 247, 177, 138,
            145, 21, 221, 187, 36, 170, 37, 171, 6, 32, 11, 180, 97, 10, 246,
        ];
        let data_parsed: serde_json::Value =
            serde_json::from_str(&soeprotocol_class.parse(data_to_parse.to_vec()))
                .unwrap_or_default();
        let succesful_data:serde_json::Value = serde_json::from_str(r#"{"error": "corruption", "name": "Error", "data_end": 75,"position": 58, "subpacket_length": 247,"raw":[0, 3, 54, 0, 21, 0, 206, 67, 0, 9, 0, 1, 0, 25, 41, 141, 45, 189, 85, 241, 64, 165, 71,228, 114, 81, 54, 5, 184, 205, 104, 0, 125, 184, 210, 74, 0, 247, 152, 225, 169, 102,204, 158, 233, 202, 228, 34, 202, 238, 136, 31, 3, 121, 222, 106, 11, 247, 177, 138,145, 21, 221, 187, 36, 170, 37, 171, 6, 32, 11, 180, 97, 10, 246]}"#).unwrap_or_default();
        assert_eq!(data_parsed, succesful_data)
    }
    #[test]
    fn multi_parse_crc_fail_test() {
        let mut soeprotocol_class = super::Soeprotocol::initialize(true, 0);
        let data_to_parse: [u8; 75] = [
            0, 3, 54, 0, 21, 0, 206, 67, 0, 9, 0, 1, 0, 25, 41, 141, 45, 189, 85, 241, 64, 165, 71,
            228, 114, 81, 54, 5, 184, 205, 104, 0, 125, 184, 210, 74, 0, 247, 152, 225, 169, 102,
            204, 158, 233, 202, 228, 34, 202, 238, 136, 31, 3, 121, 222, 106, 11, 247, 177, 138,
            145, 21, 221, 187, 36, 170, 37, 171, 6, 32, 11, 180, 97, 10, 246,
        ];
        let data_parsed: serde_json::Value =
            serde_json::from_str(&soeprotocol_class.parse(data_to_parse.to_vec()))
                .unwrap_or_default();
        let succesful_data:serde_json::Value = serde_json::from_str(r#"{"error": "crc", "name": "Error", "expected_crc": 62304,"given_crc": 2806,"raw":[0, 3, 54, 0, 21, 0, 206, 67, 0, 9, 0, 1, 0, 25, 41, 141, 45, 189, 85, 241, 64, 165, 71,228, 114, 81, 54, 5, 184, 205, 104, 0, 125, 184, 210, 74, 0, 247, 152, 225, 169, 102,204, 158, 233, 202, 228, 34, 202, 238, 136, 31, 3, 121, 222, 106, 11, 247, 177, 138,145, 21, 221, 187, 36, 170, 37, 171, 6, 32, 11, 180, 97, 10, 246]}"#).unwrap_or_default();
        assert_eq!(data_parsed, succesful_data)
    }

    #[test]
    fn multi_parse_test() {
        let mut soeprotocol_class = super::Soeprotocol::initialize(false, 0);
        let data_to_parse: [u8; 75] = [
            0, 3, 4, 0, 21, 0, 206, 67, 0, 9, 0, 1, 0, 25, 41, 141, 45, 189, 85, 241, 64, 165, 71,
            228, 114, 81, 54, 5, 184, 205, 104, 0, 125, 184, 210, 74, 0, 247, 152, 225, 169, 102,
            204, 158, 233, 202, 228, 34, 202, 238, 136, 31, 3, 121, 222, 106, 11, 247, 177, 138,
            145, 21, 221, 187, 36, 170, 37, 171, 6, 32, 11, 180, 97, 10, 246,
        ];
        let data_parsed: serde_json::Value =
            serde_json::from_str(&soeprotocol_class.parse(data_to_parse.to_vec()))
                .unwrap_or_default();
        let succesful_data:serde_json::Value = serde_json::from_str(r#"{"name":"MultiPacket","sub_packets":[{"name":"Ack","sequence":206},{"name":"Data","sequence":1,"data":[0,25,41,141,45,189,85,241,64,165,71,228,114,81,54,5,184,205,104,0,125,184,210,74,0,247,152,225,169,102,204,158,233,202,228,34,202,238,136,31,3,121,222,106,11,247,177,138,145,21,221,187,36,170,37,171,6,32,11,180,97,10,246]}]}"#).unwrap_or_default();
        assert_eq!(data_parsed, succesful_data)
    }

    #[test]
    fn multi_parse_with_crc_test() {
        let mut soeprotocol_class = super::Soeprotocol::initialize(true, 0);
        let data_to_parse: [u8; 77] = [
            0, 3, 4, 0, 21, 0, 206, 67, 0, 9, 0, 1, 0, 25, 41, 141, 45, 189, 85, 241, 64, 165, 71,
            228, 114, 81, 54, 5, 184, 205, 104, 0, 125, 184, 210, 74, 0, 247, 152, 225, 169, 102,
            204, 158, 233, 202, 228, 34, 202, 238, 136, 31, 3, 121, 222, 106, 11, 247, 177, 138,
            145, 21, 221, 187, 36, 170, 37, 171, 6, 32, 11, 180, 97, 10, 246, 10, 27,
        ];
        let data_parsed: serde_json::Value =
            serde_json::from_str(&soeprotocol_class.parse(data_to_parse.to_vec()))
                .unwrap_or_default();
        let succesful_data:serde_json::Value = serde_json::from_str(r#"{"name":"MultiPacket","sub_packets":[{"name":"Ack","sequence":206},{"name":"Data","sequence":1,"data":[0,25,41,141,45,189,85,241,64,165,71,228,114,81,54,5,184,205,104,0,125,184,210,74,0,247,152,225,169,102,204,158,233,202,228,34,202,238,136,31,3,121,222,106,11,247,177,138,145,21,221,187,36,170,37,171,6,32,11,180,97,10,246]}]}"#).unwrap_or_default();
        assert_eq!(data_parsed, succesful_data)
    }

    #[test]
    fn multi_pack_test() {
        let mut soeprotocol_class = super::Soeprotocol::initialize(false, 0);
        let data_to_pack:String = r#"{"sub_packets":[[0, 21, 0, 1],[0, 9, 0, 1, 0, 25, 41, 141, 45, 189, 85, 241, 64, 165,
        71, 228, 114, 81, 54, 5, 184, 205, 104, 0, 125, 184, 210, 74, 0, 247, 152, 225,
        169, 102, 204, 158, 233, 202, 228, 34, 202, 238, 136, 31, 3, 121, 222, 106, 11,
        247, 177, 138, 145, 21, 221, 187, 36, 170, 37, 171, 6, 32, 11, 180, 97, 10, 246]]}"#.to_owned();
        let data_pack: Vec<u8> = soeprotocol_class.pack(SoeOpcode::MultiPacket, data_to_pack);
        assert_eq!(
            data_pack,
            [
                0, 3, 4, 0, 21, 0, 1, 67, 0, 9, 0, 1, 0, 25, 41, 141, 45, 189, 85, 241, 64, 165,
                71, 228, 114, 81, 54, 5, 184, 205, 104, 0, 125, 184, 210, 74, 0, 247, 152, 225,
                169, 102, 204, 158, 233, 202, 228, 34, 202, 238, 136, 31, 3, 121, 222, 106, 11,
                247, 177, 138, 145, 21, 221, 187, 36, 170, 37, 171, 6, 32, 11, 180, 97, 10, 246
            ]
        )
    }

    #[test]
    fn multi_pack_test_deserializing_error() {
        let mut soeprotocol_class = super::Soeprotocol::initialize(false, 0);
        let data_to_pack:String = r#"{"sub_packzts":[[4, 0, 21, 0, 206],[0, 9, 0, 1, 0, 25, 41, 141, 45, 189, 85, 241, 64, 165,
        71, 228, 114, 81, 54, 5, 184, 205, 104, 0, 125, 184, 210, 74, 0, 247, 152, 225,
        169, 102, 204, 158, 233, 202, 228, 34, 202, 238, 136, 31, 3, 121, 222, 106, 11,
        247, 177, 138, 145, 21, 221, 187, 36, 170, 37, 171, 6, 32, 11, 180, 97, 10, 246]]}"#.to_owned();
        let data_pack: Vec<u8> = soeprotocol_class.pack(SoeOpcode::MultiPacket, data_to_pack);
        let data_pack_opcode: u16 = u16::from_be_bytes([data_pack[0], data_pack[1]]);
        assert_eq!(data_pack_opcode, ErrorType::Deserializing as u16)
    }

    #[test]
    fn data_parse_test() {
        let mut soeprotocol_class = super::Soeprotocol::initialize(false, 0);
        let data_to_parse: [u8; 45] = [
            0, 9, 0, 4, 252, 100, 40, 209, 68, 247, 21, 93, 18, 172, 91, 68, 145, 53, 24, 155, 2,
            113, 179, 28, 217, 33, 80, 76, 9, 235, 87, 98, 233, 235, 220, 124, 107, 61, 62, 132,
            117, 146, 204, 94, 60,
        ];
        let data_parsed: serde_json::Value =
            serde_json::from_str(&soeprotocol_class.parse(data_to_parse.to_vec()))
                .unwrap_or_default();
        let succesful_data:serde_json::Value = serde_json::from_str(r#"{"name":"Data","sequence":4,"data":[252,100,40,209,68,247,21,93,18,172,91,68,145,53,24,155,2,113,179,28,217,33,80,76,9,235,87,98,233,235,220,124,107,61,62,132,117,146,204,94,60]}"#).unwrap_or_default();
        assert_eq!(data_parsed, succesful_data)
    }

    #[test]
    fn data_parse_with_crc_test() {
        let mut soeprotocol_class = super::Soeprotocol::initialize(true, 0);
        let data_to_parse: [u8; 45] = [
            0, 9, 0, 4, 252, 100, 40, 209, 68, 247, 21, 93, 18, 172, 91, 68, 145, 53, 24, 155, 2,
            113, 179, 28, 217, 33, 80, 76, 9, 235, 87, 98, 233, 235, 220, 124, 107, 61, 62, 132,
            117, 146, 204, 94, 60,
        ];
        let data_parsed: serde_json::Value =
            serde_json::from_str(&soeprotocol_class.parse(data_to_parse.to_vec()))
                .unwrap_or_default();
        let succesful_data:serde_json::Value = serde_json::from_str(r#"{"name":"Data","sequence":4,"data":[252,100,40,209,68,247,21,93,18,172,91,68,145,53,24,155,2,113,179,28,217,33,80,76,9,235,87,98,233,235,220,124,107,61,62,132,117,146,204]}"#).unwrap_or_default();
        assert_eq!(data_parsed, succesful_data)
    }

    #[test]
    fn data_parse_with_crc_test_fail() {
        let mut soeprotocol_class = super::Soeprotocol::initialize(true, 0);
        let data_to_parse: [u8; 45] = [
            0, 9, 0, 4, 252, 100, 40, 209, 68, 247, 21, 93, 18, 172, 91, 68, 145, 53, 24, 155, 2,
            113, 179, 28, 217, 33, 80, 76, 9, 235, 87, 98, 233, 235, 220, 124, 107, 61, 62, 132,
            117, 146, 204, 94, 61,
        ];
        let data_parsed: serde_json::Value =
            serde_json::from_str(&soeprotocol_class.parse(data_to_parse.to_vec()))
                .unwrap_or_default();
        let succesful_data:serde_json::Value = serde_json::from_str(r#"{"error": "crc", "expected_crc": 24124, "given_crc": 24125, "name": "Error", "raw": [0, 9, 0, 4, 252, 100, 40, 209, 68, 247, 21, 93, 18, 172, 91, 68, 145, 53, 24, 155, 2,113, 179, 28, 217, 33, 80, 76, 9, 235, 87, 98, 233, 235, 220, 124, 107, 61, 62, 132,117, 146, 204, 94, 61]}"#).unwrap_or_default();
        assert_eq!(data_parsed, succesful_data)
    }

    #[test]
    fn ordered_data_parse_test() {
        let mut soeprotocol_class = super::Soeprotocol::initialize(false, 0);
        let data_to_parse: [u8; 6] = [0, 27, 147, 127, 1, 0];
        let data_parsed: serde_json::Value =
            serde_json::from_str(&soeprotocol_class.parse(data_to_parse.to_vec()))
                .unwrap_or_default();
        let succesful_data: serde_json::Value =
            serde_json::from_str(r#"{"name":"Ordered","order":37759,"data":[1,0]}"#)
                .unwrap_or_default();
        assert_eq!(data_parsed, succesful_data)
    }

    #[test]
    fn ordered_data_with_crc_test() {
        let mut soeprotocol_class = super::Soeprotocol::initialize(true, 0);
        let mut data_to_parse: Vec<u8> = [0, 27, 147, 127, 1, 0].to_vec();
        append_crc(&mut data_to_parse, 0);
        let data_parsed: serde_json::Value =
            serde_json::from_str(&soeprotocol_class.parse(data_to_parse.to_vec()))
                .unwrap_or_default();
        let succesful_data: serde_json::Value =
            serde_json::from_str(r#"{"name":"Ordered","order":37759,"data":[1,0]}"#)
                .unwrap_or_default();
        assert_eq!(data_parsed, succesful_data)
    }

    #[test]
    fn ordered_data_with_crc_test_fail() {
        let mut soeprotocol_class = super::Soeprotocol::initialize(true, 0);
        let data_to_parse: [u8; 6] = [0, 27, 147, 127, 1, 1];
        let data_parsed: serde_json::Value =
            serde_json::from_str(&soeprotocol_class.parse(data_to_parse.to_vec()))
                .unwrap_or_default();
        let succesful_data: serde_json::Value =
            serde_json::from_str(r#"{"name":"Ordered","order":37759,"data":[1,0]}"#)
                .unwrap_or_default();
        assert_ne!(data_parsed, succesful_data)
    }

    #[test]
    fn data_pack_test() {
        let mut soeprotocol_class = super::Soeprotocol::initialize(false, 0);
        let data_to_pack =
            r#"{"sequence":0,"data":[2,1,1,0,0,0,1,1,3,0,0,0,115,111,101,0,0,0,0]}"#.to_string();
        let data_pack: Vec<u8> = soeprotocol_class.pack(SoeOpcode::Data, data_to_pack);
        assert_eq!(
            data_pack,
            [0, 9, 0, 0, 2, 1, 1, 0, 0, 0, 1, 1, 3, 0, 0, 0, 115, 111, 101, 0, 0, 0, 0]
        )
    }

    #[test]
    fn data_ordered_pack_test() {
        let mut soeprotocol_class = super::Soeprotocol::initialize(false, 0);
        let data_to_pack =
            r#"{"sequence":1,"data":[2,1,1,0,0,0,1,1,3,0,0,0,115,111,101,0,0,0,0]}"#.to_string();
        let data_pack: Vec<u8> = soeprotocol_class.pack(SoeOpcode::Ordered, data_to_pack);
        assert_eq!(
            data_pack,
            [0, 27, 0, 1, 2, 1, 1, 0, 0, 0, 1, 1, 3, 0, 0, 0, 115, 111, 101, 0, 0, 0, 0]
        )
    }

    #[test]
    fn data_pack_test_deserializing_error() {
        let mut soeprotocol_class = super::Soeprotocol::initialize(false, 0);
        let data_to_pack = r#"{"data":[2,1,1,0,0,0,1,1,3,0,0,0,115,111,101,0,0,0,0]}"#.to_string();
        let data_pack: Vec<u8> = soeprotocol_class.pack(SoeOpcode::Data, data_to_pack);
        let data_pack_opcode: u16 = u16::from_be_bytes([data_pack[0], data_pack[1]]);
        assert_eq!(data_pack_opcode, ErrorType::Deserializing as u16)
    }

    #[test]
    fn data_fragment_parse_test() {
        let mut soeprotocol_class = super::Soeprotocol::initialize(false, 0);
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
        let data_parsed: serde_json::Value =
            serde_json::from_str(&soeprotocol_class.parse(data_to_parse.to_vec()))
                .unwrap_or_default();
        let succesful_data:serde_json::Value = serde_json::from_str(r#"{"name":"DataFragment","sequence":2,"data":[208,127,31,117,87,54,201,180,188,226,247,253,136,66,78,125,224,112,23,87,147,110,18,68,183,87,20,3,65,116,82,111,93,219,229,20,61,238,143,63,8,137,8,196,128,89,59,4,198,191,207,141,23,164,242,77,176,206,49,45,207,210,17,33,75,177,157,242,169,37,60,87,245,58,2,130,102,146,227,66,193,153,155,105,230,203,120,114,160,223,229,190,129,106,19,25,8,52,55,8,100,68,109,228,178,186,148,108,138,242,136,66,219,25,73,129,110,31,121,32,246,86,156,212,85,217,213,119,165,140,83,95,6,183,184,251,73,102,221,156,240,204,50,217,217,13,218,2,19,44,143,73,168,109,67,176,129,225,187,171,12,146,21,66,252,150,143,142,46,39,72,12,22,222,7,29,63,201,227,251,9,28,0,100,84,153,84,212,163,78,135,33,66,20,195,223,62,214,32,59,6,187,222,99,29,34,87,81,61,63,174,255,1,85,241,6,10,152,237,52,51,126,149,218,125,232,199,40,113,139,187,43,232,209,167,226,91,236,212,165,117,19,118,110,18,0,26,152,33,115,61,208,21]}"#).unwrap_or_default();
        assert_eq!(data_parsed, succesful_data)
    }

    #[test]
    fn data_fragment_parse_with_crc_test() {
        let mut soeprotocol_class = super::Soeprotocol::initialize(false, 0);
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
        let data_parsed: serde_json::Value =
            serde_json::from_str(&soeprotocol_class.parse(data_to_parse.to_vec()))
                .unwrap_or_default();
        let succesful_data:serde_json::Value = serde_json::from_str(r#"{"name":"DataFragment","sequence":2,"data":[208,127,31,117,87,54,201,180,188,226,247,253,136,66,78,125,224,112,23,87,147,110,18,68,183,87,20,3,65,116,82,111,93,219,229,20,61,238,143,63,8,137,8,196,128,89,59,4,198,191,207,141,23,164,242,77,176,206,49,45,207,210,17,33,75,177,157,242,169,37,60,87,245,58,2,130,102,146,227,66,193,153,155,105,230,203,120,114,160,223,229,190,129,106,19,25,8,52,55,8,100,68,109,228,178,186,148,108,138,242,136,66,219,25,73,129,110,31,121,32,246,86,156,212,85,217,213,119,165,140,83,95,6,183,184,251,73,102,221,156,240,204,50,217,217,13,218,2,19,44,143,73,168,109,67,176,129,225,187,171,12,146,21,66,252,150,143,142,46,39,72,12,22,222,7,29,63,201,227,251,9,28,0,100,84,153,84,212,163,78,135,33,66,20,195,223,62,214,32,59,6,187,222,99,29,34,87,81,61,63,174,255,1,85,241,6,10,152,237,52,51,126,149,218,125,232,199,40,113,139,187,43,232,209,167,226,91,236,212,165,117,19,118,110,18,0,26,152,33,115,61,208,21]}"#).unwrap_or_default();
        assert_eq!(data_parsed, succesful_data)
    }

    #[test]
    fn data_fragment_pack_test() {
        let mut soeprotocol_class = super::Soeprotocol::initialize(false, 0);
        let data_to_pack =
            r#"{"sequence":2,"data":[2,1,1,0,0,0,1,1,3,0,0,0,115,111,101,0,0,0,0]}"#.to_string();
        let data_pack: Vec<u8> = soeprotocol_class.pack(SoeOpcode::DataFragment, data_to_pack);
        assert_eq!(
            data_pack,
            [0, 13, 0, 2, 2, 1, 1, 0, 0, 0, 1, 1, 3, 0, 0, 0, 115, 111, 101, 0, 0, 0, 0]
        )
    }

    #[test]
    fn data_fragment_pack_test_deserializing_error() {
        let mut soeprotocol_class = super::Soeprotocol::initialize(false, 0);
        let data_to_pack = r#"{"data":[2,1,1,0,0,0,1,1,3,0,0,0,115,111,101,0,0,0,0]}"#.to_string();
        let data_pack: Vec<u8> = soeprotocol_class.pack(SoeOpcode::DataFragment, data_to_pack);
        let data_pack_opcode: u16 = u16::from_be_bytes([data_pack[0], data_pack[1]]);
        assert_eq!(data_pack_opcode, ErrorType::Deserializing as u16)
    }
}
