use byteorder::{BigEndian, LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::Cursor;
use wasm_bindgen::prelude::*;

use super::gatewayprotocol_packets_structs::*;
use super::lib_utils::read_prefixed_string_le;
use super::protocol_errors::gen_deserializing_error_json;

#[wasm_bindgen]
pub struct GatewayProtocol {
    wtr: Vec<u8>,
}

#[wasm_bindgen]
impl GatewayProtocol {
    #[wasm_bindgen(constructor)]
    pub fn initialize() -> GatewayProtocol {
        return GatewayProtocol { wtr: vec![] };
    }
    pub fn parse(&mut self, data: Vec<u8>) -> String {
        let mut rdr = Cursor::new(&data);
        if data.len() < 2 {
            return format!(r#"{{"name":"Unknown","raw":{:?}}}"#, data);
        }
        let opcode = rdr.read_u8().unwrap() & 0x1f;

        return match opcode {
            0x01 => self.parse_login_request(rdr),
            0x02 => self.parse_login_reply(rdr),
            0x03 => r#"{"name":"Logout"}"#.to_string(),
            0x04 => r#"{"name":"ForceDisconnect"}"#.to_string(),
            0x05 => self.parse_tunnel_data(data),
            0x06 => self.parse_tunnel_data(data),
            0x07 => self.parse_channel_is_routable(rdr),
            0x08 => self.parse_channel_is_not_routable(rdr),
            _ => format!(r#"{{"name":"Unknown","raw":{:?}}}"#, data),
        };
    }

    pub fn pack_login_request_packet(
        &mut self,
        character_id: u64,
        ticket: String,
        client_protocol: String,
        client_build: String,
    ) -> Vec<u8> {
        return self.pack_login_request_object(LoginRequestPacket {
            character_id,
            ticket,
            client_protocol,
            client_build,
            error: None,
        });
    }
    pub fn pack_login_reply_packet(&mut self, logged_in: bool) -> Vec<u8> {
        let opcode = 0x02;
        self.wtr.clear();
        self.wtr.write_u16::<BigEndian>(opcode).unwrap();
        self.wtr.write_u8(logged_in as u8).unwrap();
        return self.wtr.clone();
    }
    pub fn pack_tunnel_data_packet(&mut self, mut data: Vec<u8>) -> Vec<u8> {
        // TODO: add opcode channel calculation
        let opcode = 0x05;
        let mut tunnel_data: Vec<u8> = vec![];
        tunnel_data.push(opcode);
        tunnel_data.append(&mut data);
        return tunnel_data;
    }
    pub fn pack_channel_is_routable_packet(&mut self) -> Vec<u8> {
        todo!();
    }
    pub fn pack_channel_is_not_routable_packet(&mut self) -> Vec<u8> {
        todo!();
    }
}

impl GatewayProtocol {
    fn parse_login_request(&mut self, mut rdr: Cursor<&std::vec::Vec<u8>>) -> String {
        let character_id = rdr.read_u64::<BigEndian>().unwrap();
        let rdr_clone = rdr.clone();
        let raw_data = rdr_clone.into_inner();
        let ticket_data_pos = rdr.position();
        let ticket_data_len = rdr.read_u32::<LittleEndian>().unwrap();
        let ticket = read_prefixed_string_le(raw_data, ticket_data_pos as usize, ticket_data_len);
        rdr.set_position(ticket_data_pos + ticket_data_len as u64 + 4);
        let client_protocol_data_pos = rdr.position();
        let client_protocol_data_len = rdr.read_u32::<LittleEndian>().unwrap();
        let client_protocol = read_prefixed_string_le(
            raw_data,
            client_protocol_data_pos as usize,
            client_protocol_data_len,
        );
        rdr.set_position(client_protocol_data_pos + client_protocol_data_len as u64 + 4);
        let client_build_data_pos = rdr.position();
        let client_build_data_len = rdr.read_u32::<LittleEndian>().unwrap();
        let client_build = read_prefixed_string_le(
            raw_data,
            client_build_data_pos as usize,
            client_build_data_len,
        );
        return format!(
            r#"{{"name":"LoginRequest","character_id":{},"ticket":"{}","client_protocol":"{}","client_build":"{}"}}"#,
            character_id, ticket, client_protocol, client_build
        );
    }
    fn parse_login_reply(&mut self, mut rdr: Cursor<&std::vec::Vec<u8>>) -> String {
        todo!();
    }
    fn parse_tunnel_data(&mut self, mut data: std::vec::Vec<u8>) -> String {
        let flags = data.remove(0) >> 5;
        let tunnel_data = data;
        let packet = TunnelPacket {
            name: "TunnelPacketFromExternalConnection",
            flags,
            tunnel_data,
        };
        serde_json::to_string(&packet).unwrap()
    }
    fn parse_channel_is_routable(&mut self, mut rdr: Cursor<&std::vec::Vec<u8>>) -> String {
        todo!();
    }
    fn parse_channel_is_not_routable(&mut self, mut rdr: Cursor<&std::vec::Vec<u8>>) -> String {
        todo!();
    }

    pub fn pack_login_request_object(&mut self, packet: LoginRequestPacket) -> Vec<u8> {
        if packet.error.is_some() {
            return gen_deserializing_error_json();
        }
        self.wtr.clear();
        self.wtr.write_u16::<BigEndian>(0x01).unwrap();
        self.wtr
            .write_u64::<BigEndian>(packet.character_id)
            .unwrap();
        self.wtr
            .write_u32::<BigEndian>(packet.ticket.len() as u32)
            .unwrap();
        // TODO: WIP
        return self.wtr.clone();
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::*;
    #[test]
    fn login_request_parse_test() {
        let mut gatewayprotocol_class = GatewayProtocol::initialize();
        let data_to_parse: [u8; 59] = [
            1, 244, 221, 253, 245, 153, 56, 150, 124, 5, 0, 0, 0, 105, 116, 115, 109, 101, 19, 0,
            0, 0, 67, 108, 105, 101, 110, 116, 80, 114, 111, 116, 111, 99, 111, 108, 95, 49, 48,
            56, 48, 14, 0, 0, 0, 48, 46, 49, 57, 53, 46, 52, 46, 49, 52, 55, 53, 56, 54,
        ];
        let data_parsed: Value =
            serde_json::from_str(&gatewayprotocol_class.parse(data_to_parse.to_vec())).unwrap();
        let succesfull_data_string = r#"{"name":"LoginRequest","character_id":17644538146386908796,"ticket":"itsme","client_protocol":"ClientProtocol_1080","client_build":"0.195.4.147586"}"#;
        let succesful_data: Value = serde_json::from_str(succesfull_data_string).unwrap();
        assert_eq!(data_parsed, succesful_data)
    }
    #[test]
    fn tunnel_data_parse_test() {
        let mut gatewayprotocol_class = GatewayProtocol::initialize();
        let data_to_parse: [u8; 32] = [
            70, 254, 3, 237, 98, 176, 99, 0, 109, 235, 2, 98, 113, 5, 229, 11, 115, 16, 119, 61, 0,
            0, 0, 0, 0, 0, 0, 0, 48, 33, 0, 0,
        ];
        let data_parsed: Value =
            serde_json::from_str(&gatewayprotocol_class.parse(data_to_parse.to_vec())).unwrap();
        let succesfull_data_string = r#"{"name":"TunnelPacketFromExternalConnection","flags":2,"tunnel_data":[254, 3, 237, 98, 176, 99, 0, 109, 235, 2, 98, 113, 5, 229, 11, 115, 16, 119, 61, 0,
            0, 0, 0, 0, 0, 0, 0, 48, 33, 0, 0]}"#;
        let succesful_data: Value = serde_json::from_str(succesfull_data_string).unwrap();
        assert_eq!(data_parsed, succesful_data)
    }
    #[test]
    fn tunnel_data_pack_test() {
        let mut gatewayprotocol_class = GatewayProtocol::initialize();
        let tunnel_data_to_pack = [68, 82, 37, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0];
        let data_pack: Vec<u8> =
            gatewayprotocol_class.pack_tunnel_data_packet(tunnel_data_to_pack.to_vec());
        assert_eq!(data_pack, [5, 68, 82, 37, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0])
    }
    #[test]
    fn login_reply_pack_test() {
        let mut gatewayprotocol_class = GatewayProtocol::initialize();
        let data_pack: Vec<u8> = gatewayprotocol_class.pack_login_reply_packet(true);
        assert_eq!(data_pack, [0, 2, 1])
    }
}
