
use crate::crc::crc32;
use crate::lib_utils::str_from_u8_nul_utf8_checked;
use crate::soeprotocol::data_packet::DataPacket;
use crate::soeprotocol::{fatal_error_packet::FatalErrorPacket, ping_packet::PingPacket, soeprotocol_packets_structs::SoePacket, unknown_packet::UnknownPacket};

use super::ack_packet::AckPacket;
use super::disconnect_packet::DisconnectPacket;
use super::multi_packets::GroupedPackets;
use super::net_status_reply_packet::NetStatusReplyPacket;
use super::net_status_request_packet::NetStatusRequestPacket;
use super::session_reply_packet::SessionReplyPacket;
use super::{session_request_packet::SessionRequestPacket, soeprotocol_functions::*};
use super::soeprotocol_packets_structs::SoePacketParsed;
use byteorder::{BigEndian, ReadBytesExt};
use serde::{Deserialize, Serialize};
use std::io::Cursor;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Soeprotocol {
    use_crc: bool,
    crc_seed: u32,
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
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
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
}
#[wasm_bindgen]
impl Soeprotocol {
    // wasm lib
    #[wasm_bindgen(constructor)]
    pub fn initialize(use_crc: bool, crc_seed: u32) -> Soeprotocol {
        Soeprotocol { use_crc, crc_seed }
    }

    pub fn parse(&mut self, data: Vec<u8>) -> SoePacketParsed {
        let mut rdr = Cursor::new(&data);
        let opcode: SoeOpcode = if data.len() >= 2 {
            self.get_opcode(&mut rdr)
        } else {
            SoeOpcode::Unknown
        };

        match opcode {
            SoeOpcode::Unknown => SoePacketParsed::new(
                SoeOpcode::Unknown,
                SoePacket::UnknownPacket(UnknownPacket {}),
            ),
            SoeOpcode::SessionRequest => SoePacketParsed::new(
                SoeOpcode::SessionRequest,
                SoePacket::SessionRequestPacket(self.parse_session_request(rdr)),
            ),
            SoeOpcode::SessionReply => SoePacketParsed::new(
                SoeOpcode::SessionReply,
                SoePacket::SessionReplyPacket(self.parse_session_reply(rdr)),
            ),
            SoeOpcode::MultiPacket => SoePacketParsed::new(
                SoeOpcode::MultiPacket,
                SoePacket::GroupedPackets(self.parse_multi(rdr)),
            ),
            SoeOpcode::Disconnect => SoePacketParsed::new(
                SoeOpcode::Disconnect,
                SoePacket::DisconnectPacket(self.parse_disconnect(rdr)),
            ),
            SoeOpcode::Ping => {
                SoePacketParsed::new(SoeOpcode::Ping, SoePacket::PingPacket(PingPacket {}))
            }
            SoeOpcode::NetStatusRequest => SoePacketParsed::new(
                SoeOpcode::NetStatusRequest,
                SoePacket::NetStatusRequestPacket(self.parse_net_status_request(rdr)),
            ),
            SoeOpcode::NetStatusReply => SoePacketParsed::new(
                SoeOpcode::NetStatusReply,
                SoePacket::NetStatusReplyPacket(self.parse_net_status_reply(rdr)),
            ),
            SoeOpcode::Data => SoePacketParsed::new(
                SoeOpcode::Data,
                SoePacket::DataPacket(self.parse_data(rdr, opcode as u16)),
            ),
            SoeOpcode::DataFragment => SoePacketParsed::new(
                SoeOpcode::DataFragment,
                SoePacket::DataPacket(self.parse_data(rdr, opcode as u16)),
            ),
            SoeOpcode::OutOfOrder => SoePacketParsed::new(
                SoeOpcode::OutOfOrder,
                SoePacket::AckPacket(self.parse_ack(rdr, opcode as u16)),
            ),
            SoeOpcode::Ack => SoePacketParsed::new(
                SoeOpcode::Ack,
                SoePacket::AckPacket(self.parse_ack(rdr, opcode as u16)),
            ),
            SoeOpcode::Group => SoePacketParsed::new(
                SoeOpcode::Group,
                SoePacket::GroupedPackets(self.parse_multi(rdr)),
            ),
            SoeOpcode::Ordered => SoePacketParsed::new(
                SoeOpcode::Ordered,
                SoePacket::DataPacket(self.parse_data(rdr, opcode as u16)),
            ),
            SoeOpcode::FatalError => SoePacketParsed::new(
                SoeOpcode::FatalError,
                SoePacket::FatalErrorPacket(FatalErrorPacket {}),
            ),
        }
    }

    fn parse_session_request(
        &mut self,
        mut rdr: Cursor<&std::vec::Vec<u8>>,
    ) -> SessionRequestPacket {
        // if !check_min_size(&rdr, PacketsMinSize::SessionRequest as usize, false) {
        //     return gen_size_error_json(rdr);
        // }

        let crc_length = rdr.read_u32::<BigEndian>().unwrap_or_default();
        let session_id = rdr.read_u32::<BigEndian>().unwrap_or_default();
        let udp_length = rdr.read_u32::<BigEndian>().unwrap_or_default();
        let protocol_data_position = rdr.position() as usize;
        let raw_data = rdr.into_inner();
        let protocol =
            str_from_u8_nul_utf8_checked(&raw_data[protocol_data_position..]).to_string();
        SessionRequestPacket::new(crc_length, session_id, udp_length, protocol)
    }

    fn parse_session_reply(&mut self, mut rdr: Cursor<&std::vec::Vec<u8>>) -> SessionReplyPacket {
        // if rdr.get_ref().len() != PacketsMinSize::SessionReply as usize {
        //     return gen_size_error_json(rdr);
        // }
        let session_id = rdr.read_u32::<BigEndian>().unwrap_or_default();
        let crc_seed = rdr.read_u32::<BigEndian>().unwrap_or_default();
        let crc_length = rdr.read_u8().unwrap_or_default();
        let encrypt_method = rdr.read_u16::<BigEndian>().unwrap_or_default();
        let udp_length = rdr.read_u32::<BigEndian>().unwrap_or_default();
        SessionReplyPacket {
            session_id,
            crc_seed,
            crc_length,
            encrypt_method,
            udp_length,
        }
    }

    fn parse_disconnect(&mut self, mut rdr: Cursor<&std::vec::Vec<u8>>) -> DisconnectPacket {
        // if rdr.get_ref().len() < PacketsMinSize::Disconnect as usize {
        //     return r#"{"name":"Disconnect" ,"session_id":null,"reason":"unknown"}"#.to_string();
        // }
        let session_id = rdr.read_u32::<BigEndian>().unwrap_or_default();
        let reason = disconnect_reason_to_string(rdr.read_u16::<BigEndian>().unwrap_or_default());
        DisconnectPacket::new(session_id, reason)
    }

    fn parse_net_status_request(
        &mut self,
        mut rdr: Cursor<&std::vec::Vec<u8>>,
    ) -> NetStatusRequestPacket {
        // if rdr.get_ref().len() != PacketsMinSize::NetStatusPacket as usize {
        //     return gen_size_error_json(rdr);
        // }
        let client_tick_count = rdr.read_u16::<BigEndian>().unwrap_or_default();
        let last_client_update = rdr.read_u32::<BigEndian>().unwrap_or_default();
        let average_update = rdr.read_u32::<BigEndian>().unwrap_or_default();
        let shortest_update = rdr.read_u32::<BigEndian>().unwrap_or_default();
        let longest_update = rdr.read_u32::<BigEndian>().unwrap_or_default();
        let last_server_update = rdr.read_u32::<BigEndian>().unwrap_or_default();
        let packets_sent = rdr.read_u64::<BigEndian>().unwrap_or_default();
        let packets_received = rdr.read_u64::<BigEndian>().unwrap_or_default();
        let unknown_field = rdr.read_u16::<BigEndian>().unwrap_or_default();
        NetStatusRequestPacket {
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

    fn parse_net_status_reply(
        &mut self,
        mut rdr: Cursor<&std::vec::Vec<u8>>,
    ) -> NetStatusReplyPacket {
        // if rdr.get_ref().len() != PacketsMinSize::NetStatusPacket as usize {
        //     return gen_size_error_json(rdr);
        // }
        let client_tick_count = rdr.read_u16::<BigEndian>().unwrap_or_default();
        let server_tick_count = rdr.read_u32::<BigEndian>().unwrap_or_default();
        let client_packet_sent = rdr.read_u64::<BigEndian>().unwrap_or_default();
        let client_packet_received = rdr.read_u64::<BigEndian>().unwrap_or_default();
        let server_packet_sent = rdr.read_u64::<BigEndian>().unwrap_or_default();
        let server_packet_received = rdr.read_u64::<BigEndian>().unwrap_or_default();
        let unknown_field = rdr.read_u16::<BigEndian>().unwrap_or_default();
        NetStatusReplyPacket {
            client_tick_count,
            server_tick_count,
            client_packet_sent,
            client_packet_received,
            server_packet_sent,
            server_packet_received,
            unknown_field,
        }
    }

    fn parse_multi(&mut self, mut rdr: Cursor<&std::vec::Vec<u8>>) -> GroupedPackets {
        // check size
        // if !check_min_size(
        //     &rdr,
        //     PacketsMinSize::MultiPacket as usize,
        //     self.is_using_crc(),
        // ) {
        //     return gen_size_error_json(rdr);
        // }
        let mut grouped_packet = GroupedPackets::new();
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
            // if crc_value != crc {
            //     return gen_crc_error_json(vec, crc_value, crc);
            // }
            rdr.set_position(2); // reset pos after the opcode
        }
        loop {
            let sub_packet_data_length = read_data_length(&mut rdr);
            if sub_packet_data_length == 0
                || sub_packet_data_length as u64 + rdr.position() > data_end
            {
                // return gen_corruption_error_json(rdr, sub_packet_data_length, data_end);
            }
            let sub_packet_data =
                extract_subpacket_data(&rdr, rdr.position(), sub_packet_data_length);
            rdr.set_position(sub_packet_data_length as u64 + rdr.position());
            let sub_packet_parsed = self.parse(sub_packet_data);
            grouped_packet.add_packet(sub_packet_parsed.get_packet());
            if rdr.position() == data_end {
                break;
            }
        }
        if was_crc_enabled {
            self.enable_crc();
        }
        grouped_packet
    }

    fn parse_data(&mut self, mut rdr: Cursor<&std::vec::Vec<u8>>, opcode: u16) -> DataPacket {
        // if !check_min_size(&rdr, PacketsMinSize::DataPacket as usize, self.use_crc) {
        //     return gen_size_error_json(rdr);
        // }
        //
        // TODO: il va falloir ajouter un moyen d'identifier les différents types de data
        // let name = if opcode == 0x09 {
        //     "Data"
        // } else {
        //     "DataFragment"
        // };
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
            // if crc_value != crc {
            //     return gen_crc_error_json(&vec, crc_value, crc);
            // }
        }
        DataPacket::new(data.to_vec(), sequence, opcode)
    }

    fn parse_ack(&mut self, mut rdr: Cursor<&std::vec::Vec<u8>>, opcode: u16) -> AckPacket {
        // if !check_min_size(&rdr, PacketsMinSize::Ack as usize, self.use_crc) {
        //     return gen_size_error_json(rdr);
        // }
        // TODO: pareil pour les acks ajoute juste l'opcode comme field
        // let name = if opcode == 0x15 { "Ack" } else { "OutOfOrder" };
        let sequence = rdr.read_u16::<BigEndian>().unwrap_or_default();
        if self.use_crc {
            let crc = rdr.read_u16::<BigEndian>().unwrap_or_default();
            let data_end: u64 = get_data_end(&rdr, self.use_crc);
            let vec = rdr.into_inner();
            let packet_without_crc = &vec[0..data_end as usize];
            let crc_value =
                (crc32(&&mut packet_without_crc.to_vec(), self.crc_seed as usize) & 0xffff) as u16;
            // if crc_value != crc {
            //     return gen_crc_error_json(vec, crc_value, crc);
            // }
        }
        AckPacket { sequence, opcode }
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
