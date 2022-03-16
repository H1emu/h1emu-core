use byteorder::{BigEndian, WriteBytesExt, ReadBytesExt};
use std::io::Cursor;
use wasm_bindgen::prelude::*;
use serde_json::*;
mod soeprotocol_functions;
use soeprotocol_functions::*;
use crate::rc4::RC4;

#[wasm_bindgen]
pub struct Soeprotocol {

}


#[wasm_bindgen]
impl Soeprotocol {
  pub fn pack(&mut self,packet_name: String, packet: String,crc_seed: u32, _use_compression: bool, rc4: RC4) -> Vec<u8>{
        match packet_name.as_str() {
            "SessionRequest" => return pack_session_request(packet),
            "SessionReply" => return pack_session_reply(packet),
            "Disconnect" => return vec![0,5],
            "Ping" => return vec![0,6],
          //  "NetStatusRequest" => return pack_data(packet),
         //   "NetStatusReply" => return pack_data(packet),
            "Data" => return pack_data(packet,crc_seed, _use_compression, rc4),
            "DataFragment" => return pack_fragment_data(packet,crc_seed, _use_compression, rc4),
            "OutOfOrder" => return pack_out_of_order(packet),
            "Ack" => return pack_ack(packet),
            _ => return vec![]
        }
    }

 pub fn parse(&mut self,data: Vec<u8>, rc4: RC4) -> String{
        let mut rdr = Cursor::new(&data);

        let opcode = rdr.read_u16::<BigEndian>().unwrap();

        match opcode {
            0x01 => return parse_session_request(rdr),
            0x02 => return parse_session_reply(rdr),
          //  0x03 => TODO,
            0x05 => return json!({}).to_string(),
            0x06 => return json!({}).to_string(),
            0x07 => return json!({}).to_string(),
            0x08 => return json!({}).to_string(),
            0x09 => return parse_data(rdr, rc4),
            0x0d => return parse_data(rdr,rc4),
            0x11 => return parse_ack(rdr,rc4),
            0x15 => return parse_ack(rdr,rc4),
            _ => return json!({}).to_string()
        }
    }


    
}


#[wasm_bindgen]
pub fn session_reply(
    session_id: u32,
    crc_seed: u32,
    crc_length: u8,
    compression: u16,
    udp_length: u32,
) -> std::vec::Vec<u8> {
    let mut wtr = vec![];
    wtr.write_u16::<BigEndian>(0x02).unwrap();
    wtr.write_u32::<BigEndian>(session_id).unwrap();
    wtr.write_u32::<BigEndian>(crc_seed).unwrap();
    wtr.write_u8(crc_length).unwrap();
    wtr.write_u16::<BigEndian>(compression).unwrap();
    wtr.write_u32::<BigEndian>(udp_length).unwrap();
    wtr.write_u32::<BigEndian>(3).unwrap();
    return wtr;
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn session_request_parse_test() {
        let key: [u8; 16] = [
            23, 189,   8, 107, 27, 148,
           240,  47, 240, 236, 83, 215,
            99,  88, 155,  95
         ];
        let rc4_obj = RC4::initialize(key.to_vec());
        let mut soeprotocol_class = Soeprotocol {};
        let data_to_parse:[u8;25] = [0,1,0,0,0,3,60,23,140,99,0,0,2,0,76,111,103,105,110,85,100,112,95,57,0];
        let data_parsed: String = soeprotocol_class.parse(data_to_parse.to_vec(),rc4_obj);
        assert_eq!(
            data_parsed,
            r#"{"crc_length":3,"session_id":1008176227,"udp_length":512,"protocol":"LoginUdp_9"}"#
        )
    }

    #[test]
    fn session_request_pack_test() {
        let key: [u8; 16] = [
            23, 189,   8, 107, 27, 148,
           240,  47, 240, 236, 83, 215,
            99,  88, 155,  95
         ];
        let rc4_obj = RC4::initialize(key.to_vec());
        let mut soeprotocol_class = Soeprotocol {};
        let data_to_pack = r#"{"crc_length":3,"session_id":1008176227,"udp_length":512,"protocol":"LoginUdp_9"}"#.to_string();
        let data_pack: Vec<u8> = soeprotocol_class.pack("SessionRequest".to_owned(),data_to_pack,0,false,rc4_obj);
        assert_eq!(
            data_pack,
            [0,1,0,0,0,3,60,23,140,99,0,0,2,0,76,111,103,105,110,85,100,112,95,57,0]
        )
    }

    #[test]
    fn session_reply_parse_test() {
        let key: [u8; 16] = [
            23, 189,   8, 107, 27, 148,
           240,  47, 240, 236, 83, 215,
            99,  88, 155,  95
         ];
        let rc4_obj = RC4::initialize(key.to_vec());
        let mut soeprotocol_class = Soeprotocol {};
        let data_to_parse:[u8;21] = [0, 2, 60, 23, 140, 99, 0, 0, 0, 0, 2, 1, 0, 0, 0, 2, 0, 0, 0, 0, 3];
        let data_parsed: String = soeprotocol_class.parse(data_to_parse.to_vec(),rc4_obj);
        assert_eq!(
            data_parsed,
            r#"{"session_id":1008176227,"crc_seed":0,"crc_length":2,"compression":256,"udp_length":512}"#
        )
    }

    #[test]
    fn session_reply_pack_test() {
        let key: [u8; 16] = [
            23, 189,   8, 107, 27, 148,
           240,  47, 240, 236, 83, 215,
            99,  88, 155,  95
         ];
        let rc4_obj = RC4::initialize(key.to_vec());
        let mut soeprotocol_class = Soeprotocol {};
        let data_to_pack =  r#"{"session_id":1008176227,"crc_seed":0,"crc_length":2,"compression":256,"udp_length":512}"#.to_string();
        let data_pack: Vec<u8> = soeprotocol_class.pack("SessionReply".to_owned(),data_to_pack,0,false,rc4_obj);
        assert_eq!(
            data_pack,
            [0, 2, 60, 23, 140, 99, 0, 0, 0, 0, 2, 1, 0, 0, 0, 2, 0, 0, 0, 0, 3]
        )
    }


    #[test]
    fn data_parse_test() {
        let key: [u8; 16] = [
            23, 189,   8, 107, 27, 148,
           240,  47, 240, 236, 83, 215,
            99,  88, 155,  95
         ];
        let rc4_obj = RC4::initialize(key.to_vec());
        let mut soeprotocol_class = Soeprotocol {};
        let data_to_parse:[u8;45] = [0,9,0,4,252,100,40,209,68,247,21,93,18,172,91,68,145,53,24,155,2,113,179,28,217,33,80,76,9,235,87,98,233,235,220,124,107,61,62,132,117,146,204,94,60];
        let data_parsed: String = soeprotocol_class.parse(data_to_parse.to_vec(),rc4_obj);
        assert_eq!(
            data_parsed,
            r#"{"channel":0,"sequence":4,"crc":24124,"data":[252,100,40,209,68,247,21,93,18,172,91,68,145,53,24,155,2,113,179,28,217,33,80,76,9,235,87,98,233,235,220,124,107,61,62,132,117,146,204]}"#
        )
    }

    #[test]
    fn data_fragment_parse_test() {
        let key: [u8; 16] = [
            23, 189,   8, 107, 27, 148,
           240,  47, 240, 236, 83, 215,
            99,  88, 155,  95
         ];
        let rc4_obj = RC4::initialize(key.to_vec());
        let mut soeprotocol_class = Soeprotocol {};
        let data_to_parse:[u8;259] = [0,13,0,2,208,127,31,117,87,54,201,180,188,226,247,253,136,66,78,125,224,112,23,87,147,110,18,68,183,87,20,3,65,116,82,111,93,219,229,20,61,238,143,63,8,137,8,196,128,89,59,4,198,191,207,141,23,164,242,77,176,206,49,45,207,210,17,33,75,177,157,242,169,37,60,87,245,58,2,130,102,146,227,66,193,153,155,105,230,203,120,114,160,223,229,190,129,106,19,25,8,52,55,8,100,68,109,228,178,186,148,108,138,242,136,66,219,25,73,129,110,31,121,32,246,86,156,212,85,217,213,119,165,140,83,95,6,183,184,251,73,102,221,156,240,204,50,217,217,13,218,2,19,44,143,73,168,109,67,176,129,225,187,171,12,146,21,66,252,150,143,142,46,39,72,12,22,222,7,29,63,201,227,251,9,28,0,100,84,153,84,212,163,78,135,33,66,20,195,223,62,214,32,59,6,187,222,99,29,34,87,81,61,63,174,255,1,85,241,6,10,152,237,52,51,126,149,218,125,232,199,40,113,139,187,43,232,209,167,226,91,236,212,165,117,19,118,110,18,0,26,152,33,115,61,208,21,253,82];
        let data_parsed: String = soeprotocol_class.parse(data_to_parse.to_vec(),rc4_obj);
        assert_eq!(
            data_parsed,
            r#"{"channel":0,"sequence":2,"crc":64850,"data":[208,127,31,117,87,54,201,180,188,226,247,253,136,66,78,125,224,112,23,87,147,110,18,68,183,87,20,3,65,116,82,111,93,219,229,20,61,238,143,63,8,137,8,196,128,89,59,4,198,191,207,141,23,164,242,77,176,206,49,45,207,210,17,33,75,177,157,242,169,37,60,87,245,58,2,130,102,146,227,66,193,153,155,105,230,203,120,114,160,223,229,190,129,106,19,25,8,52,55,8,100,68,109,228,178,186,148,108,138,242,136,66,219,25,73,129,110,31,121,32,246,86,156,212,85,217,213,119,165,140,83,95,6,183,184,251,73,102,221,156,240,204,50,217,217,13,218,2,19,44,143,73,168,109,67,176,129,225,187,171,12,146,21,66,252,150,143,142,46,39,72,12,22,222,7,29,63,201,227,251,9,28,0,100,84,153,84,212,163,78,135,33,66,20,195,223,62,214,32,59,6,187,222,99,29,34,87,81,61,63,174,255,1,85,241,6,10,152,237,52,51,126,149,218,125,232,199,40,113,139,187,43,232,209,167,226,91,236,212,165,117,19,118,110,18,0,26,152,33,115,61,208,21]}"#
        )
    }
}
