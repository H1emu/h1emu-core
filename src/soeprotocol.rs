use byteorder::{BigEndian, ReadBytesExt};
use std::io::Cursor;
use wasm_bindgen::prelude::*;
#[path = "./soeprotocol/soeprotocol_functions.rs"]
mod soeprotocol_functions;
use serde_json::*;
use soeprotocol_functions::*;

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
    crc_seed: u8,
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

#[wasm_bindgen]
impl Soeprotocol {
    #[wasm_bindgen(constructor)]
    pub fn initialize(use_crc: bool, crc_seed: u8) -> Soeprotocol {
        let ping_packet = CachedPacket {
            parsed: json!({"name":"Ping"}).to_string(),
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
            "SessionRequest" => return pack_session_request(packet),
            "SessionReply" => return pack_session_reply(packet),
            "MultiPacket" => return pack_multi(packet, self),
            "Disconnect" => return vec![0, 5],
            "Ping" => self.cached_packets.ping.packed.to_owned(),
            //  "NetStatusRequest" => return pack_data(packet),
            //   "NetStatusReply" => return pack_data(packet),
            "Data" => return pack_data(packet, self.crc_seed, self.use_crc),
            "DataFragment" => return pack_fragment_data(packet, self.crc_seed, self.use_crc),
            "OutOfOrder" => return pack_out_of_order(packet, self.crc_seed, self.use_crc),
            "Ack" => return pack_ack(packet, self.crc_seed, self.use_crc),
            _ => return vec![],
        }
    }

    pub fn parse(&mut self, data: Vec<u8>) -> String {
        let mut rdr = Cursor::new(&data);
        let opcode = rdr.read_u16::<BigEndian>().unwrap();

        return match opcode {
            0x01 => parse_session_request(rdr),
            0x02 => parse_session_reply(rdr),
            0x03 => parse_multi(rdr, self),
            0x05 => parse_disconnect(rdr),
            0x06 => self.cached_packets.ping.parsed.to_owned(),
            //  0x07 => json!({"name":"NetStatusRequest"}).to_string(),
            //  0x08 => json!({"name":"NetStatusReply"}).to_string(),
            0x09 => parse_data(rdr, opcode, self.crc_seed, self.use_crc),
            0x0d => parse_data(rdr, opcode, self.crc_seed, self.use_crc),
            0x11 => parse_ack(rdr, opcode, self.crc_seed, self.use_crc),
            0x15 => parse_ack(rdr, opcode, self.crc_seed, self.use_crc),
            _ => "".to_string(),
        };
    }
    pub fn get_crc_seed(&self) -> u8 {
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
        let data_parsed: Value = serde_json::from_str(&soeprotocol_class.parse(data_to_parse.to_vec())).unwrap();
        let succesful_data: Value = serde_json::from_str(r#"{"name":"SessionRequest","crc_length":3,"session_id":1008176227,"udp_length":512,"protocol":"LoginUdp_9"}"#).unwrap();
        assert_eq!(
            data_parsed,
            succesful_data
        )
    }


    #[test]
    fn session_request_parse_size_error_test() {
        let mut soeprotocol_class = Soeprotocol::initialize(true, 0);
        let data_to_parse: [u8; 12] = [0, 1, 111, 103, 105, 110, 85, 100, 112,95, 57, 2];
        let data_parsed: Value = serde_json::from_str(&soeprotocol_class.parse(data_to_parse.to_vec())).unwrap();
        let succesful_data: Value = serde_json::from_str(r#"{"error": "size", "name": "Error", "raw": [0, 1, 111, 103, 105, 110, 85, 100, 112,95, 57, 2], "size": 12}"#).unwrap();
        assert_eq!(
            data_parsed,
            succesful_data
        )
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
    fn session_reply_parse_size_error_test() {
        let mut soeprotocol_class = Soeprotocol::initialize(true, 0);
        let data_to_parse: [u8; 10] = [
            0, 2, 111, 103, 105, 110, 85, 100, 112,
            95
        ];
        let data_parsed: Value = serde_json::from_str(&soeprotocol_class.parse(data_to_parse.to_vec())).unwrap();
        let succesful_data: Value = serde_json::from_str(r#"{"error": "size", "name": "Error", "raw": [0,2,111,103,105,110,85,100,112,95], "size": 10}"#).unwrap();
        assert_eq!(
            data_parsed,
            succesful_data
        )
    }

    #[test]
    fn session_reply_parse_test() {
        let mut soeprotocol_class = Soeprotocol::initialize(true, 0);
        let data_to_parse: [u8; 21] = [
            0, 2, 60, 23, 140, 99, 0, 0, 0, 0, 2, 1, 0, 0, 0, 2, 0, 0, 0, 0, 3,
        ];
        let data_parsed: Value = serde_json::from_str(&soeprotocol_class.parse(data_to_parse.to_vec())).unwrap();
        let succesful_data: Value = serde_json::from_str(r#"{"name":"SessionReply","session_id":1008176227,"crc_seed":0,"crc_length":2,"encrypt_method":256,"udp_length":512}"#).unwrap();
        assert_eq!(
            data_parsed,succesful_data
        )
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
        let data_to_parse: [u8; 3] = [
            0, 17, 111
        ];
        let data_parsed: Value = serde_json::from_str(&soeprotocol_class.parse(data_to_parse.to_vec())).unwrap();
        let succesful_data: Value = serde_json::from_str(r#"{"error": "size", "name": "Error", "raw": [0, 17, 111], "size": 3}"#).unwrap();
        assert_eq!(
            data_parsed,
            succesful_data
        )
    }

    #[test]
    fn outoforder_parse_test() {
        let mut soeprotocol_class = Soeprotocol::initialize(true, 0);
        let data_to_parse: [u8; 6] = [0, 17, 0, 1, 38, 184];
        let data_parsed: Value = serde_json::from_str(&soeprotocol_class.parse(data_to_parse.to_vec())).unwrap();
        let succesful_data: Value = serde_json::from_str(r#"{"name":"OutOfOrder","sequence":1}"#).unwrap();
        assert_eq!(
            data_parsed,
            succesful_data
        )
    }

    #[test]
    fn outoforder_parse_test_crc_fail() {
        let mut soeprotocol_class = Soeprotocol::initialize(true, 0);
        let data_to_parse: [u8; 6] = [0, 17, 0, 1, 142, 100];
        let data_parsed: Value = serde_json::from_str(&soeprotocol_class.parse(data_to_parse.to_vec())).unwrap();
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
    fn outoforder_pack_with_crc_test() {
        let mut soeprotocol_class = Soeprotocol::initialize(true, 0);
        let data_to_pack: String = r#"{"name":"OutOfOrder","sequence":1}"#.to_owned();
        let data_pack: Vec<u8> = soeprotocol_class.pack("OutOfOrder".to_owned(), data_to_pack);
        assert_eq!(data_pack, [0, 17, 0, 1, 38, 184])
    }

    #[test]
    fn ack_parse_size_error_test() {
        let mut soeprotocol_class = Soeprotocol::initialize(true, 0);
        let data_to_parse: [u8; 3] = [
            0, 21, 111
        ];
        let data_parsed: Value = serde_json::from_str(&soeprotocol_class.parse(data_to_parse.to_vec())).unwrap();
        let succesful_data: Value = serde_json::from_str(r#"{"error": "size", "name": "Error", "raw": [0, 21, 111], "size": 3}"#).unwrap();
        assert_eq!(
            data_parsed,
            succesful_data
        )
    }

    #[test]
    fn ack_parse_test() {
        let mut soeprotocol_class = Soeprotocol::initialize(true, 0);
        let data_to_parse: [u8; 6] = [0, 21, 0, 1, 142, 100];
        let data_parsed: Value = serde_json::from_str(&soeprotocol_class.parse(data_to_parse.to_vec())).unwrap();
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
    fn ack_pack_with_crc_test() {
        let mut soeprotocol_class = Soeprotocol::initialize(true, 0);
        let data_to_pack: String = r#"{"name":"Ack","sequence":1}"#.to_owned();
        let data_pack: Vec<u8> = soeprotocol_class.pack("Ack".to_owned(), data_to_pack);
        assert_eq!(data_pack, [0, 21, 0, 1, 142, 100])
    }

    #[test]
    fn multi_parse_size_error_test() {
        let mut soeprotocol_class = Soeprotocol::initialize(true, 0);
        let data_to_parse: [u8; 4] = [
            0, 3, 4, 0
        ];
        let data_parsed: Value = serde_json::from_str(&soeprotocol_class.parse(data_to_parse.to_vec())).unwrap();
        let succesful_data: Value = serde_json::from_str(r#"{"error": "size", "name": "Error", "raw": [0, 3, 4, 0], "size": 4}"#).unwrap();
        assert_eq!(
            data_parsed,
            succesful_data
        )
    }

    #[test]
    fn multi_parse_corrupted_test() {
        let mut soeprotocol_class = Soeprotocol::initialize(true, 0);
        let data_to_parse: [u8; 75] = [0, 3, 54, 0, 21, 0, 206, 67, 0, 9, 0, 1, 0, 25, 41, 141, 45, 189, 85, 241, 64, 165, 71,228, 114, 81, 54, 5, 184, 205, 104, 0, 125, 184, 210, 74, 0, 247, 152, 225, 169, 102,204, 158, 233, 202, 228, 34, 202, 238, 136, 31, 3, 121, 222, 106, 11, 247, 177, 138,145, 21, 221, 187, 36, 170, 37, 171, 6, 32, 11, 180, 97, 10, 246];
        let data_parsed: Value = serde_json::from_str(&soeprotocol_class.parse(data_to_parse.to_vec())).unwrap();
        let succesful_data: Value = serde_json::from_str(r#"{"error": "corruption", "name": "Error", "raw":[0, 3, 54, 0, 21, 0, 206, 67, 0, 9, 0, 1, 0, 25, 41, 141, 45, 189, 85, 241, 64, 165, 71,228, 114, 81, 54, 5, 184, 205, 104, 0, 125, 184, 210, 74, 0, 247, 152, 225, 169, 102,204, 158, 233, 202, 228, 34, 202, 238, 136, 31, 3, 121, 222, 106, 11, 247, 177, 138,145, 21, 221, 187, 36, 170, 37, 171, 6, 32, 11, 180, 97, 10, 246]}"#).unwrap();
        assert_eq!(
            data_parsed,
            succesful_data
        )
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
        let data_parsed: Value = serde_json::from_str(&soeprotocol_class.parse(data_to_parse.to_vec())).unwrap();
        let succesful_data: Value = serde_json::from_str(r#"{"name":"MultiPacket","sub_packets":[{"name":"Ack","sequence":206},{"name":"Data","sequence":1,"data":[0,25,41,141,45,189,85,241,64,165,71,228,114,81,54,5,184,205,104,0,125,184,210,74,0,247,152,225,169,102,204,158,233,202,228,34,202,238,136,31,3,121,222,106,11,247,177,138,145,21,221,187,36,170,37,171,6,32,11,180,97,10,246]}]}"#).unwrap();
        assert_eq!(
            data_parsed,
            succesful_data
        )
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
        let data_parsed: Value = serde_json::from_str(&soeprotocol_class.parse(data_to_parse.to_vec())).unwrap();
        let succesful_data: Value = serde_json::from_str(r#"{"name":"MultiPacket","sub_packets":[{"name":"Ack","sequence":206},{"name":"Data","sequence":1,"data":[0,25,41,141,45,189,85,241,64,165,71,228,114,81,54,5,184,205,104,0,125,184,210,74,0,247,152,225,169,102,204,158,233,202,228,34,202,238,136,31,3,121,222,106,11,247,177,138,145,21,221,187,36,170,37,171,6,32,11,180,97,10,246]}]}"#).unwrap();
        assert_eq!(
            data_parsed,
            succesful_data
        )
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
        let data_parsed: Value = serde_json::from_str(&soeprotocol_class.parse(data_to_parse.to_vec())).unwrap();
        let succesful_data: Value = serde_json::from_str(r#"{"name":"Data","sequence":4,"data":[252,100,40,209,68,247,21,93,18,172,91,68,145,53,24,155,2,113,179,28,217,33,80,76,9,235,87,98,233,235,220,124,107,61,62,132,117,146,204,94,60]}"#).unwrap();
        assert_eq!(
            data_parsed,
            succesful_data
        )
    }

    #[test]
    fn data_parse_with_crc_test() {
        let mut soeprotocol_class = Soeprotocol::initialize(true, 0);
        let data_to_parse: [u8; 45] = [
            0, 9, 0, 4, 252, 100, 40, 209, 68, 247, 21, 93, 18, 172, 91, 68, 145, 53, 24, 155, 2,
            113, 179, 28, 217, 33, 80, 76, 9, 235, 87, 98, 233, 235, 220, 124, 107, 61, 62, 132,
            117, 146, 204, 94, 60,
        ];
        let data_parsed: Value = serde_json::from_str(&soeprotocol_class.parse(data_to_parse.to_vec())).unwrap();
        let succesful_data: Value = serde_json::from_str(r#"{"name":"Data","sequence":4,"data":[252,100,40,209,68,247,21,93,18,172,91,68,145,53,24,155,2,113,179,28,217,33,80,76,9,235,87,98,233,235,220,124,107,61,62,132,117,146,204]}"#).unwrap();
        assert_eq!(
            data_parsed,
            succesful_data
        )
    }

    #[test]
    fn data_parse_with_crc_test_fail() {
        let mut soeprotocol_class = Soeprotocol::initialize(true, 0);
        let data_to_parse: [u8; 45] = [0, 9, 0, 4, 252, 100, 40, 209, 68, 247, 21, 93, 18, 172, 91, 68, 145, 53, 24, 155, 2,113, 179, 28, 217, 33, 80, 76, 9, 235, 87, 98, 233, 235, 220, 124, 107, 61, 62, 132,117, 146, 204, 94, 61];
        let data_parsed: Value = serde_json::from_str(&soeprotocol_class.parse(data_to_parse.to_vec())).unwrap();
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
        let data_parsed: Value = serde_json::from_str(&soeprotocol_class.parse(data_to_parse.to_vec())).unwrap();
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
        let data_parsed: Value = serde_json::from_str(&soeprotocol_class.parse(data_to_parse.to_vec())).unwrap();
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
