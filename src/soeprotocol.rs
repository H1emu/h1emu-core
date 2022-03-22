use byteorder::{BigEndian, ReadBytesExt};
use std::io::Cursor;
use wasm_bindgen::prelude::*;
mod soeprotocol_functions;
use soeprotocol_functions::*;
use serde_json::*;
use crate::rc4::RC4;

#[wasm_bindgen]
pub struct Soeprotocol {

}


#[wasm_bindgen]
impl Soeprotocol {
  #[wasm_bindgen(constructor)]
  pub fn initialize() -> Soeprotocol { 
        return Soeprotocol {};
  }
  pub fn pack(&mut self,packet_name: String, packet: String,crc_seed: u32, _use_compression: bool, rc4: &mut RC4) -> Vec<u8>{
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

 pub fn parse(&mut self,data: Vec<u8>, rc4: &mut RC4) -> String{
        let mut rdr = Cursor::new(&data);

        let opcode = rdr.read_u16::<BigEndian>().unwrap();

        return match opcode {
            0x01 =>  parse_session_request(rdr),
            0x02 =>  parse_session_reply(rdr),
           // 0x03 => TODO,
            0x05 =>  json!({"name":"Disconnect"}).to_string(),
            0x06 =>  json!({"name":"Ping"}).to_string(),
           // 0x07 =>  json!({}),
           // 0x08 =>  json!({}),
            0x09 =>  parse_data(rdr, rc4,opcode),
            0x0d =>  parse_data(rdr,rc4,opcode),
            0x11 =>  parse_ack(rdr,opcode),
            0x15 =>  parse_ack(rdr,opcode),
            _ => "".to_string()
        };
    }




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
        let mut rc4_obj = RC4::initialize(key.to_vec());
        let mut soeprotocol_class = Soeprotocol {};
        let data_to_parse:[u8;25] = [0,1,0,0,0,3,60,23,140,99,0,0,2,0,76,111,103,105,110,85,100,112,95,57,0];
        let data_parsed: String = soeprotocol_class.parse(data_to_parse.to_vec(),&mut rc4_obj);
        assert_eq!(
            data_parsed,
            r#"{"name":"SessionRequest","crc_length":3,"session_id":1008176227,"udp_length":512,"protocol":"LoginUdp_9"}"#
        )
    }

    #[test]
    fn session_request_pack_test() {
        let key: [u8; 16] = [
            23, 189,   8, 107, 27, 148,
           240,  47, 240, 236, 83, 215,
            99,  88, 155,  95
         ];
        let mut rc4_obj = RC4::initialize(key.to_vec());
        let mut soeprotocol_class = Soeprotocol {};
        let data_to_pack = r#"{"crc_length":3,"session_id":1008176227,"udp_length":512,"protocol":"LoginUdp_9"}"#.to_string();
        let data_pack: Vec<u8> = soeprotocol_class.pack("SessionRequest".to_owned(),data_to_pack,0,false,&mut rc4_obj);
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
        let mut rc4_obj = RC4::initialize(key.to_vec());
        let mut soeprotocol_class = Soeprotocol {};
        let data_to_parse:[u8;21] = [0, 2, 60, 23, 140, 99, 0, 0, 0, 0, 2, 1, 0, 0, 0, 2, 0, 0, 0, 0, 3];
        let data_parsed: String = soeprotocol_class.parse(data_to_parse.to_vec(),&mut rc4_obj);
        assert_eq!(
            data_parsed,
            r#"{"name":"SessionReply","session_id":1008176227,"crc_seed":0,"crc_length":2,"compression":256,"udp_length":512}"#
        )
    }

    #[test]
    fn session_reply_pack_test() {
        let key: [u8; 16] = [
            23, 189,   8, 107, 27, 148,
           240,  47, 240, 236, 83, 215,
            99,  88, 155,  95
         ];
        let mut rc4_obj = RC4::initialize(key.to_vec());
        let mut soeprotocol_class = Soeprotocol {};
        let data_to_pack =  r#"{"session_id":1008176227,"crc_seed":0,"crc_length":2,"compression":256,"udp_length":512}"#.to_string();
        let data_pack: Vec<u8> = soeprotocol_class.pack("SessionReply".to_owned(),data_to_pack,0,false,&mut rc4_obj);
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
        let mut rc4_obj = RC4::initialize(key.to_vec());
        let mut soeprotocol_class = Soeprotocol {};
        let data_to_parse:[u8;45] = [0,9,0,4,252,100,40,209,68,247,21,93,18,172,91,68,145,53,24,155,2,113,179,28,217,33,80,76,9,235,87,98,233,235,220,124,107,61,62,132,117,146,204,94,60];
        let data_parsed: String = soeprotocol_class.parse(data_to_parse.to_vec(),&mut rc4_obj);
        assert_eq!(
            data_parsed,
            r#"{"name":"Data","channel":0,"sequence":4,"crc":24124,"data":[252,100,40,209,68,247,21,93,18,172,91,68,145,53,24,155,2,113,179,28,217,33,80,76,9,235,87,98,233,235,220,124,107,61,62,132,117,146,204]}"#
        )
    }

    #[test]
    fn data_pack_test() {
        let key: [u8; 16] = [
            23, 189,   8, 107, 27, 148,
           240,  47, 240, 236, 83, 215,
            99,  88, 155,  95
         ];
        let mut rc4_obj = RC4::initialize(key.to_vec());
        let mut soeprotocol_class = Soeprotocol {};
        let data_to_pack =  r#"{"sequence":0,"data":[2, 1, 1, 0, 0, 0, 1, 1, 3, 0, 0, 0, 115, 111, 101, 0, 0, 0, 0]}"#.to_string();
        let data_pack: Vec<u8> = soeprotocol_class.pack("Data".to_owned(),data_to_pack,0,false,&mut rc4_obj);
        assert_eq!(
            data_pack,
            [0,9,0,0,169,183,185,67,241,64,164,5,143,19,35,87,21,163,205,26,83,24,212,18,57]
        )
    }

    #[test]
    fn data_fragment_parse_test() {
        let key: [u8; 16] = [
            23, 189,   8, 107, 27, 148,
           240,  47, 240, 236, 83, 215,
            99,  88, 155,  95
         ];
        let mut rc4_obj = RC4::initialize(key.to_vec());
        let mut soeprotocol_class = Soeprotocol {};
        let data_to_parse:[u8;259] = [0,13,0,2,208,127,31,117,87,54,201,180,188,226,247,253,136,66,78,125,224,112,23,87,147,110,18,68,183,87,20,3,65,116,82,111,93,219,229,20,61,238,143,63,8,137,8,196,128,89,59,4,198,191,207,141,23,164,242,77,176,206,49,45,207,210,17,33,75,177,157,242,169,37,60,87,245,58,2,130,102,146,227,66,193,153,155,105,230,203,120,114,160,223,229,190,129,106,19,25,8,52,55,8,100,68,109,228,178,186,148,108,138,242,136,66,219,25,73,129,110,31,121,32,246,86,156,212,85,217,213,119,165,140,83,95,6,183,184,251,73,102,221,156,240,204,50,217,217,13,218,2,19,44,143,73,168,109,67,176,129,225,187,171,12,146,21,66,252,150,143,142,46,39,72,12,22,222,7,29,63,201,227,251,9,28,0,100,84,153,84,212,163,78,135,33,66,20,195,223,62,214,32,59,6,187,222,99,29,34,87,81,61,63,174,255,1,85,241,6,10,152,237,52,51,126,149,218,125,232,199,40,113,139,187,43,232,209,167,226,91,236,212,165,117,19,118,110,18,0,26,152,33,115,61,208,21,253,82];
        let data_parsed: String = soeprotocol_class.parse(data_to_parse.to_vec(),&mut rc4_obj);
        assert_eq!(
            data_parsed,
            r#"{"name":"DataFragment","channel":0,"sequence":2,"crc":64850,"data":[208,127,31,117,87,54,201,180,188,226,247,253,136,66,78,125,224,112,23,87,147,110,18,68,183,87,20,3,65,116,82,111,93,219,229,20,61,238,143,63,8,137,8,196,128,89,59,4,198,191,207,141,23,164,242,77,176,206,49,45,207,210,17,33,75,177,157,242,169,37,60,87,245,58,2,130,102,146,227,66,193,153,155,105,230,203,120,114,160,223,229,190,129,106,19,25,8,52,55,8,100,68,109,228,178,186,148,108,138,242,136,66,219,25,73,129,110,31,121,32,246,86,156,212,85,217,213,119,165,140,83,95,6,183,184,251,73,102,221,156,240,204,50,217,217,13,218,2,19,44,143,73,168,109,67,176,129,225,187,171,12,146,21,66,252,150,143,142,46,39,72,12,22,222,7,29,63,201,227,251,9,28,0,100,84,153,84,212,163,78,135,33,66,20,195,223,62,214,32,59,6,187,222,99,29,34,87,81,61,63,174,255,1,85,241,6,10,152,237,52,51,126,149,218,125,232,199,40,113,139,187,43,232,209,167,226,91,236,212,165,117,19,118,110,18,0,26,152,33,115,61,208,21]}"#
        )
    }

    #[test]
    fn data_fragment_pack_test() {
        let key: [u8; 16] = [
            23, 189,   8, 107, 27, 148,
           240,  47, 240, 236, 83, 215,
            99,  88, 155,  95
         ];
        let mut rc4_obj = RC4::initialize(key.to_vec());
        let mut soeprotocol_class = Soeprotocol {};
        let data_to_pack =  r#"{"sequence":1,"data":[14,1,0,0,0,1,0,0,0,2,0,0,0,0,10,0,0,0,83,111,108,111,83,101,114,118,101,114,195,0,0,0,4,0,0,0,121,101,97,104,82,5,0,0,0,0,0,0,153,0,0,0,60,83,101,114,118,101,114,73,110,102,111,32,82,101,103,105,111,110,61,34,67,104,97,114,97,99,116,101,114,67,114,101,97,116,101,46,82,101,103,105,111,110,85,115,34,32,83,117,98,114,101,103,105,111,110,61,34,85,73,46,83,117,98,114,101,103,105,111,110,85,83,34,32,73,115,82,101,99,111,109,109,101,110,100,101,100,61,34,49,34,32,73,115,82,101,99,111,109,109,101,110,100,101,100,86,83,61,34,48,34,32,73,115,82,101,99,111,109,109,101,110,100,101,100,78,67,61,34,48,34,32,73,115,82,101,99,111,109,109,101,110,100,101,100,84,82,61,34,48,34,32,47,62,3,0,0,0,140,1,0,0,60,80,111,112,117,108,97,116,105,111,110,32,83,101,114,118,101,114,67,97,112,97,99,105,116,121,61,34,48,34,32,80,105,110,103,65,100,100,114,101,115,115,61,34,49,50,55,46,48,46,48,46,49,58,49,49,49,55,34,32,82,117,108,101,115,101,116,115,61,34,80,101,114,109,97,100,101,97,116,104,34,62,60,102,97,99,116,105,111,110,108,105,115,116,32,73,115,76,105,115,116,61,34,49,34,62,60,102,97,99,116,105,111,110,32,73,100,61,34,49,34,32,80,101,114,99,101,110,116,61,34,48,34,32,84,97,114,103,101,116,80,111,112,80,99,116,61,34,48,34,32,82,101,119,97,114,100,66,117,102,102,61,34,53,50,34,32,88,80,66,117,102,102,61,34,53,50,34,32,80,101,114,99,101,110,116,65,118,103,61,34,48,34,47,62,60,102,97,99,116,105,111,110,32,73,100,61,34,50,34,32,80,101,114,99,101,110,116,61,34,48,34,32,84,97,114,103,101,116,80,111,112,80,99,116,61,34,49,34,32,82,101,119,97,114,100,66,117,102,102,61,34,48,34,32,88,80,66,117,102,102,61,34,48,34,32,80,101,114,99,101,110,116,65,118,103,61,34,48,34,47,62,60,102,97,99,116,105,111,110,32,73,100,61,34,51,34,32,80,101,114,99,101,110,116,61,34,48,34,32,84,97,114,103,101,116,80,111,112,80,99,116,61,34,49,34,32,82,101,119,97,114,100,66,117,102,102,61,34,48,34,32,88,80,66,117,102,102,61,34,48,34,32,80,101,114,99,101,110,116,65,118,103,61,34,49,34,47,62,60,47,102,97,99,116,105,111,110,108,105,115,116,62,60,47,80,111,112,117,108,97,116,105,111,110,62,1]}"#.to_string();
        let data_pack: Vec<u8> = soeprotocol_class.pack("DataFragment".to_owned(),data_to_pack,0,false,&mut rc4_obj);
        assert_eq!(
            data_pack,
            [0, 13, 0, 1, 165, 183, 184, 67, 241, 65, 165, 4, 140, 17, 35, 87, 102, 204, 162, 26, 83, 24, 135, 216, 69, 27, 236, 156, 253, 187, 111, 219, 47, 237, 202, 228, 38, 162, 135, 236, 3, 102, 24, 182, 56, 212, 
            133, 143, 145, 21, 221, 222, 211, 245, 112, 248, 26, 42, 193, 29, 124, 147, 80, 46, 33, 201, 192, 41, 73, 40, 9, 108, 55, 178, 60, 121, 82, 240, 64, 137, 6, 236, 147, 248, 93, 146, 128, 166, 114, 185, 211, 144, 86, 
            77, 216, 64, 78, 147, 231, 19, 70, 247, 63, 127, 37, 78, 255, 221, 159, 56, 126, 115, 110, 102, 176, 152, 57, 61, 66, 193, 117, 170, 58, 39, 132, 175, 250, 88, 147, 235, 42, 195, 78, 54, 132, 249, 245, 104, 61, 194, 237, 171, 54, 163, 222, 92, 243, 19, 74, 223, 211, 87, 126, 32, 82, 146, 70, 80, 52, 132, 108, 239, 69, 79, 231, 2, 65, 76, 31, 124, 196, 143, 180, 164, 119, 61, 11, 42, 237, 235, 130, 47, 99, 76, 200, 217, 18, 29, 6, 244, 124, 60, 7, 28, 159, 42, 87, 34, 209, 164, 216, 117, 136, 44, 26, 10, 150, 238, 103, 39, 2, 174, 11, 119, 5, 196, 171, 158, 197, 161, 83, 63, 116, 222, 143, 134, 172, 137, 180, 101, 12, 213, 97, 178, 6, 192, 42, 108, 113, 118, 12, 215, 173, 57, 124, 89, 59, 171, 1, 156, 132, 235, 198, 121, 22, 254, 214, 175, 139, 108, 15, 118, 214, 6, 17, 12, 170, 149, 130, 106, 188, 159, 20, 193, 44, 233, 56, 31, 142, 86, 142, 34, 217, 191, 140, 222, 181, 48, 155, 149, 249, 217, 194, 83, 196, 145, 86, 253, 198, 224, 244, 130, 241, 21, 106, 65, 22, 49, 86, 19, 137, 70, 175, 56, 185, 45, 50, 139, 40, 19, 39, 165, 244, 28, 163, 1, 222, 0, 21, 243, 170, 139, 94, 116, 158, 208, 79, 86, 67, 170, 219, 194, 8, 69, 214, 183, 30, 146, 130, 248, 244, 224, 15, 58, 125, 92, 58, 153, 52, 38, 236, 221, 119, 23, 194, 230, 213, 102, 160, 197, 79, 62, 203, 78, 89, 225, 115, 134, 116, 209, 255, 164, 100, 137, 32, 181, 0, 6, 93, 108, 162, 167, 54, 145, 189, 131, 137, 186, 53, 93, 7, 233, 168, 205, 222, 232, 93, 134, 190, 104, 127, 104, 117, 125, 253, 73, 78, 224, 32, 118, 221, 40, 156, 222, 58, 156, 35, 201, 124, 142, 8, 104, 78, 251, 48, 152, 12, 230, 245, 186, 180, 143, 15, 128, 127, 72, 154, 42, 241, 216, 178, 215, 36, 20, 192, 175, 75, 194, 231, 248, 234, 155, 78, 79, 232, 194, 66, 133, 70, 198, 90, 74, 212, 76, 116, 231, 248, 54, 29, 126, 146, 82, 249, 201, 218, 71, 1, 174, 77, 151, 249, 30, 157, 230, 181, 167, 184, 2, 163, 34, 135, 163, 188, 78, 71, 108, 176, 103, 198, 243, 14, 68, 246, 31, 108, 120, 
            124, 68, 27, 37, 153, 37, 190, 168, 40, 16, 122, 78, 83, 181, 64, 99, 85, 151, 82, 137, 175, 120, 17, 71, 118, 225, 139, 67, 15, 184, 116, 211, 183, 113, 139, 106, 46, 171, 176, 49, 144, 219, 38, 244, 20, 96, 197, 165, 219, 144, 134, 2, 255, 229, 203, 65, 96, 151, 67, 45, 27, 139, 43, 43, 32, 60, 246, 64, 184, 207, 87, 255, 253, 136, 33, 202, 5, 158, 127, 219, 69, 150, 189, 62, 93, 175, 11, 194, 80, 246, 99, 11, 246, 55, 25, 130, 103, 65, 221, 11]
        )
    }
}
