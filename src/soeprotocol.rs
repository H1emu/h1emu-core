use byteorder::{BigEndian, ReadBytesExt};
use std::io::Cursor;
use wasm_bindgen::prelude::*;
mod soeprotocol_functions;
use soeprotocol_functions::*;
use serde_json::*;
use crate::rc4::RC4;

#[wasm_bindgen]
pub struct Soeprotocol {
    use_crc: bool,
    use_compression : bool,
    use_encryption : bool,
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
  pub fn initialize(use_crc: bool,use_compression : bool,use_encryption : bool) -> Soeprotocol { 
        return Soeprotocol {use_crc,use_compression,use_encryption};
  }
  pub fn pack(&self,packet_name: String, packet: String,crc_seed: u8, rc4: &mut RC4) -> Vec<u8>{
        match packet_name.as_str() {
            "SessionRequest" => return pack_session_request(packet),
            "SessionReply" => return pack_session_reply(packet),
            "Disconnect" => return vec![0,5],
            "Ping" => return vec![0,6],
          //  "NetStatusRequest" => return pack_data(packet),
         //   "NetStatusReply" => return pack_data(packet),
            "Data" => return pack_data(packet,crc_seed,self.use_crc, rc4,self.use_encryption),
            "DataFragment" => return pack_fragment_data(packet,crc_seed,self.use_crc, rc4,self.use_encryption),
            "OutOfOrder" => return pack_out_of_order(packet),
            "Ack" => return pack_ack(packet),
            _ => return vec![]
        }
    }

 pub fn parse(&self,data: Vec<u8>, rc4: &mut RC4) -> String{
        let mut rdr = Cursor::new(&data);

        let opcode = rdr.read_u16::<BigEndian>().unwrap();

        return match opcode {
            0x01 =>  parse_session_request(rdr),
            0x02 =>  parse_session_reply(rdr),
           // 0x03 => TODO,
            0x05 =>  parse_disconnect(rdr),
            0x06 =>  json!({"name":"Ping"}).to_string(),
           // 0x07 =>  json!({}),
           // 0x08 =>  json!({}),
            0x09 =>  parse_data(rdr,self.use_crc, rc4,opcode),
            0x0d =>  parse_data(rdr,self.use_crc,rc4,opcode),
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
        let soeprotocol_class = Soeprotocol {use_crc:true,use_compression:false,use_encryption:true};
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
        let soeprotocol_class = Soeprotocol {use_crc:true,use_compression:false,use_encryption:true};
        let data_to_pack = r#"{"crc_length":3,"session_id":1008176227,"udp_length":512,"protocol":"LoginUdp_9"}"#.to_string();
        let data_pack: Vec<u8> = soeprotocol_class.pack("SessionRequest".to_owned(),data_to_pack,0,&mut rc4_obj);
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
        let soeprotocol_class = Soeprotocol {use_crc:true,use_compression:false,use_encryption:true};
        let data_to_parse:[u8;21] = [0, 2, 60, 23, 140, 99, 0, 0, 0, 0, 2, 1, 0, 0, 0, 2, 0, 0, 0, 0, 3];
        let data_parsed: String = soeprotocol_class.parse(data_to_parse.to_vec(),&mut rc4_obj);
        assert_eq!(
            data_parsed,
            r#"{"name":"SessionReply","session_id":1008176227,"crc_seed":0,"crc_length":2,"encrypt_method":256,"udp_length":512}"#
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
        let soeprotocol_class = Soeprotocol {use_crc:true,use_compression:false,use_encryption:true};
        let data_to_pack =  r#"{"session_id":1008176227,"crc_seed":0,"crc_length":2,"encrypt_method":256,"udp_length":512}"#.to_string();
        let data_pack: Vec<u8> = soeprotocol_class.pack("SessionReply".to_owned(),data_to_pack,0,&mut rc4_obj);
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
        let soeprotocol_class = Soeprotocol {use_crc:false,use_compression:false,use_encryption:false};
        let data_to_parse:[u8;45] = [0,9,0,4,252,100,40,209,68,247,21,93,18,172,91,68,145,53,24,155,2,113,179,28,217,33,80,76,9,235,87,98,233,235,220,124,107,61,62,132,117,146,204,94,60];
        let data_parsed: String = soeprotocol_class.parse(data_to_parse.to_vec(),&mut rc4_obj);
        assert_eq!(
            data_parsed,
            r#"{"name":"Data","channel":0,"sequence":4,"crc":0,"data":[252,100,40,209,68,247,21,93,18,172,91,68,145,53,24,155,2,113,179,28,217,33,80,76,9,235,87,98,233,235,220,124,107,61,62,132,117,146,204]}"#
        )
    }

    #[test]
    fn data_parse_with_encryption_test() {
        let key: [u8; 16] = [
            23, 189,   8, 107, 27, 148,
           240,  47, 240, 236, 83, 215,
            99,  88, 155,  95
         ];
        let mut rc4_obj = RC4::initialize(key.to_vec());
        let soeprotocol_class = Soeprotocol {use_crc:false,use_compression:false,use_encryption:true};
        let data_to_parse:[u8;45] = [0,9,0,4,252,100,40,209,68,247,21,93,18,172,91,68,145,53,24,155,2,113,179,28,217,33,80,76,9,235,87,98,233,235,220,124,107,61,62,132,117,146,204,94,60];
        let data_parsed: String = soeprotocol_class.parse(data_to_parse.to_vec(),&mut rc4_obj);
        assert_eq!(
            data_parsed,
            r#"{"name":"Data","channel":0,"sequence":4,"crc":0,"data":[252,100,40,209,68,247,21,93,18,172,91,68,145,53,24,155,2,113,179,28,217,33,80,76,9,235,87,98,233,235,220,124,107,61,62,132,117,146,204]}"#
        )
    }
    #[test]
    fn data_parse_with_crc_test() {
        let key: [u8; 16] = [
            23, 189,   8, 107, 27, 148,
           240,  47, 240, 236, 83, 215,
            99,  88, 155,  95
         ];
        let mut rc4_obj = RC4::initialize(key.to_vec());
        let soeprotocol_class = Soeprotocol {use_crc:true,use_compression:false,use_encryption:true};
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
        let soeprotocol_class = Soeprotocol {use_crc:false,use_compression:false,use_encryption:false};
        let data_to_pack =  r#"{"sequence":0,"data":[2,1,1,0,0,0,1,1,3,0,0,0,115,111,101,0,0,0,0]}"#.to_string();
        let data_pack: Vec<u8> = soeprotocol_class.pack("Data".to_owned(),data_to_pack,0,&mut rc4_obj);
        assert_eq!(
            data_pack,
            [0, 9, 0, 0, 2, 1, 1, 0, 0, 0, 1, 1, 3, 0, 0, 0, 115, 111, 101, 0, 0, 0, 0]
        )
    }

    #[test]
    fn data_pack_with_encryption_test() {
        let key: [u8; 16] = [
            23, 189,   8, 107, 27, 148,
           240,  47, 240, 236, 83, 215,
            99,  88, 155,  95
         ];
        let mut rc4_obj = RC4::initialize(key.to_vec());
        let soeprotocol_class = Soeprotocol {use_crc:false,use_compression:false,use_encryption:true};
        let data_to_pack =  r#"{"sequence":0,"data":[2,1,1,0,0,0,1,1,3,0,0,0,115,111,101,0,0,0,0]}"#.to_string();
        let data_pack: Vec<u8> = soeprotocol_class.pack("Data".to_owned(),data_to_pack,0,&mut rc4_obj);
        assert_eq!(
            data_pack,
            [0,9,0,0,169,183,185,67,241,64,164,5,143,19,35,87,21,163,205,26,83,24,212]
        )
    }

    #[test]
    fn data_pack_with_crc_test() {
        let key: [u8; 16] = [
            23, 189,   8, 107, 27, 148,
           240,  47, 240, 236, 83, 215,
            99,  88, 155,  95
         ];
        let mut rc4_obj = RC4::initialize(key.to_vec());
        let soeprotocol_class = Soeprotocol {use_crc:true,use_compression:false,use_encryption:true};
        let data_to_pack =  r#"{"sequence":0,"data":[2,1,1,0,0,0,1,1,3,0,0,0,115,111,101,0,0,0,0]}"#.to_string();
        let data_pack: Vec<u8> = soeprotocol_class.pack("Data".to_owned(),data_to_pack,0,&mut rc4_obj);
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
        let soeprotocol_class = Soeprotocol {use_crc:false,use_compression:false,use_encryption:false};
        let data_to_parse:[u8;257] = [0,13,0,2,208,127,31,117,87,54,201,180,188,226,247,253,136,66,78,125,224,112,23,87,147,110,18,68,183,87,20,3,65,116,82,111,93,219,229,20,61,238,143,63,8,137,8,196,128,89,59,4,198,191,207,141,23,164,242,77,176,206,49,45,207,210,17,33,75,177,157,242,169,37,60,87,245,58,2,130,102,146,227,66,193,153,155,105,230,203,120,114,160,223,229,190,129,106,19,25,8,52,55,8,100,68,109,228,178,186,148,108,138,242,136,66,219,25,73,129,110,31,121,32,246,86,156,212,85,217,213,119,165,140,83,95,6,183,184,251,73,102,221,156,240,204,50,217,217,13,218,2,19,44,143,73,168,109,67,176,129,225,187,171,12,146,21,66,252,150,143,142,46,39,72,12,22,222,7,29,63,201,227,251,9,28,0,100,84,153,84,212,163,78,135,33,66,20,195,223,62,214,32,59,6,187,222,99,29,34,87,81,61,63,174,255,1,85,241,6,10,152,237,52,51,126,149,218,125,232,199,40,113,139,187,43,232,209,167,226,91,236,212,165,117,19,118,110,18,0,26,152,33,115,61,208,21];
        let data_parsed: String = soeprotocol_class.parse(data_to_parse.to_vec(),&mut rc4_obj);
        assert_eq!(
            data_parsed,
            r#"{"name":"DataFragment","channel":0,"sequence":2,"crc":0,"data":[208,127,31,117,87,54,201,180,188,226,247,253,136,66,78,125,224,112,23,87,147,110,18,68,183,87,20,3,65,116,82,111,93,219,229,20,61,238,143,63,8,137,8,196,128,89,59,4,198,191,207,141,23,164,242,77,176,206,49,45,207,210,17,33,75,177,157,242,169,37,60,87,245,58,2,130,102,146,227,66,193,153,155,105,230,203,120,114,160,223,229,190,129,106,19,25,8,52,55,8,100,68,109,228,178,186,148,108,138,242,136,66,219,25,73,129,110,31,121,32,246,86,156,212,85,217,213,119,165,140,83,95,6,183,184,251,73,102,221,156,240,204,50,217,217,13,218,2,19,44,143,73,168,109,67,176,129,225,187,171,12,146,21,66,252,150,143,142,46,39,72,12,22,222,7,29,63,201,227,251,9,28,0,100,84,153,84,212,163,78,135,33,66,20,195,223,62,214,32,59,6,187,222,99,29,34,87,81,61,63,174,255,1,85,241,6,10,152,237,52,51,126,149,218,125,232,199,40,113,139,187,43,232,209,167,226,91,236,212,165,117,19,118,110,18,0,26,152,33,115,61]}"#
        )
    }

    #[test]
    fn data_fragment_parse_with_encryption_test() {
        let key: [u8; 16] = [
            23, 189,   8, 107, 27, 148,
           240,  47, 240, 236, 83, 215,
            99,  88, 155,  95
         ];
        let mut rc4_obj = RC4::initialize(key.to_vec());
        let soeprotocol_class = Soeprotocol {use_crc:false,use_compression:false,use_encryption:false};
        let data_to_parse:[u8;257] = [0,13,0,2,208,127,31,117,87,54,201,180,188,226,247,253,136,66,78,125,224,112,23,87,147,110,18,68,183,87,20,3,65,116,82,111,93,219,229,20,61,238,143,63,8,137,8,196,128,89,59,4,198,191,207,141,23,164,242,77,176,206,49,45,207,210,17,33,75,177,157,242,169,37,60,87,245,58,2,130,102,146,227,66,193,153,155,105,230,203,120,114,160,223,229,190,129,106,19,25,8,52,55,8,100,68,109,228,178,186,148,108,138,242,136,66,219,25,73,129,110,31,121,32,246,86,156,212,85,217,213,119,165,140,83,95,6,183,184,251,73,102,221,156,240,204,50,217,217,13,218,2,19,44,143,73,168,109,67,176,129,225,187,171,12,146,21,66,252,150,143,142,46,39,72,12,22,222,7,29,63,201,227,251,9,28,0,100,84,153,84,212,163,78,135,33,66,20,195,223,62,214,32,59,6,187,222,99,29,34,87,81,61,63,174,255,1,85,241,6,10,152,237,52,51,126,149,218,125,232,199,40,113,139,187,43,232,209,167,226,91,236,212,165,117,19,118,110,18,0,26,152,33,115,61,208,21];
        let data_parsed: String = soeprotocol_class.parse(data_to_parse.to_vec(),&mut rc4_obj);
        assert_eq!(
            data_parsed,
            r#"{"name":"DataFragment","channel":0,"sequence":2,"crc":0,"data":[208,127,31,117,87,54,201,180,188,226,247,253,136,66,78,125,224,112,23,87,147,110,18,68,183,87,20,3,65,116,82,111,93,219,229,20,61,238,143,63,8,137,8,196,128,89,59,4,198,191,207,141,23,164,242,77,176,206,49,45,207,210,17,33,75,177,157,242,169,37,60,87,245,58,2,130,102,146,227,66,193,153,155,105,230,203,120,114,160,223,229,190,129,106,19,25,8,52,55,8,100,68,109,228,178,186,148,108,138,242,136,66,219,25,73,129,110,31,121,32,246,86,156,212,85,217,213,119,165,140,83,95,6,183,184,251,73,102,221,156,240,204,50,217,217,13,218,2,19,44,143,73,168,109,67,176,129,225,187,171,12,146,21,66,252,150,143,142,46,39,72,12,22,222,7,29,63,201,227,251,9,28,0,100,84,153,84,212,163,78,135,33,66,20,195,223,62,214,32,59,6,187,222,99,29,34,87,81,61,63,174,255,1,85,241,6,10,152,237,52,51,126,149,218,125,232,199,40,113,139,187,43,232,209,167,226,91,236,212,165,117,19,118,110,18,0,26,152,33,115,61]}"#
        )
    }

    #[test]
    fn data_fragment_parse_with_crc_test() {
        let key: [u8; 16] = [
            23, 189,   8, 107, 27, 148,
           240,  47, 240, 236, 83, 215,
            99,  88, 155,  95
         ];
        let mut rc4_obj = RC4::initialize(key.to_vec());
        let soeprotocol_class = Soeprotocol {use_crc:false,use_compression:false,use_encryption:false};
        let data_to_parse:[u8;257] = [0,13,0,2,208,127,31,117,87,54,201,180,188,226,247,253,136,66,78,125,224,112,23,87,147,110,18,68,183,87,20,3,65,116,82,111,93,219,229,20,61,238,143,63,8,137,8,196,128,89,59,4,198,191,207,141,23,164,242,77,176,206,49,45,207,210,17,33,75,177,157,242,169,37,60,87,245,58,2,130,102,146,227,66,193,153,155,105,230,203,120,114,160,223,229,190,129,106,19,25,8,52,55,8,100,68,109,228,178,186,148,108,138,242,136,66,219,25,73,129,110,31,121,32,246,86,156,212,85,217,213,119,165,140,83,95,6,183,184,251,73,102,221,156,240,204,50,217,217,13,218,2,19,44,143,73,168,109,67,176,129,225,187,171,12,146,21,66,252,150,143,142,46,39,72,12,22,222,7,29,63,201,227,251,9,28,0,100,84,153,84,212,163,78,135,33,66,20,195,223,62,214,32,59,6,187,222,99,29,34,87,81,61,63,174,255,1,85,241,6,10,152,237,52,51,126,149,218,125,232,199,40,113,139,187,43,232,209,167,226,91,236,212,165,117,19,118,110,18,0,26,152,33,115,61,208,21];
        let data_parsed: String = soeprotocol_class.parse(data_to_parse.to_vec(),&mut rc4_obj);
        assert_eq!(
            data_parsed,
            r#"{"name":"DataFragment","channel":0,"sequence":2,"crc":0,"data":[208,127,31,117,87,54,201,180,188,226,247,253,136,66,78,125,224,112,23,87,147,110,18,68,183,87,20,3,65,116,82,111,93,219,229,20,61,238,143,63,8,137,8,196,128,89,59,4,198,191,207,141,23,164,242,77,176,206,49,45,207,210,17,33,75,177,157,242,169,37,60,87,245,58,2,130,102,146,227,66,193,153,155,105,230,203,120,114,160,223,229,190,129,106,19,25,8,52,55,8,100,68,109,228,178,186,148,108,138,242,136,66,219,25,73,129,110,31,121,32,246,86,156,212,85,217,213,119,165,140,83,95,6,183,184,251,73,102,221,156,240,204,50,217,217,13,218,2,19,44,143,73,168,109,67,176,129,225,187,171,12,146,21,66,252,150,143,142,46,39,72,12,22,222,7,29,63,201,227,251,9,28,0,100,84,153,84,212,163,78,135,33,66,20,195,223,62,214,32,59,6,187,222,99,29,34,87,81,61,63,174,255,1,85,241,6,10,152,237,52,51,126,149,218,125,232,199,40,113,139,187,43,232,209,167,226,91,236,212,165,117,19,118,110,18,0,26,152,33,115,61]}"#
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
        let soeprotocol_class = Soeprotocol {use_crc:false,use_compression:false,use_encryption:false};
        let data_to_pack =  r#"{"sequence":2,"data":[2,1,1,0,0,0,1,1,3,0,0,0,115,111,101,0,0,0,0]}"#.to_string();
        let data_pack: Vec<u8> = soeprotocol_class.pack("DataFragment".to_owned(),data_to_pack,0,&mut rc4_obj);
        assert_eq!(
            data_pack,
            [0, 13, 0, 2, 2, 1, 1, 0, 0, 0, 1, 1, 3, 0, 0, 0, 115, 111, 101, 0, 0, 0, 0]
        )
    }

    #[test]
    fn data_fragment_pack_with_encryption_test() {
        
        let key: [u8; 16] = [
            23, 189,   8, 107, 27, 148,
           240,  47, 240, 236, 83, 215,
            99,  88, 155,  95
         ];
        let mut rc4_obj = RC4::initialize(key.to_vec());
        let soeprotocol_class = Soeprotocol {use_crc:false,use_compression:false,use_encryption:true};
        let data_to_pack =  r#"{"sequence":2,"data":[2,1,1,0,0,0,1,1,3,0,0,0,115,111,101,0,0,0,0]}"#.to_string();
        let data_pack: Vec<u8> = soeprotocol_class.pack("DataFragment".to_owned(),data_to_pack,0,&mut rc4_obj);
        assert_eq!(
            data_pack,
            [0,13,0,2,169,183,185,67,241,64,164,5,143,19,35,87,21,163,205,26,83,24,212]
        )
    }

    #[test]
    fn data_fragment_pack_with_crc_test() {
        let key: [u8; 16] = [
            23, 189,   8, 107, 27, 148,
           240,  47, 240, 236, 83, 215,
            99,  88, 155,  95
         ];
        let mut rc4_obj = RC4::initialize(key.to_vec());
        let soeprotocol_class = Soeprotocol {use_crc:true,use_compression:false,use_encryption:true};
        let data_to_pack =  r#"{"sequence":2,"data":[2,1,1,0,0,0,1,1,3,0,0,0,115,111,101,0,0,0,0]}"#.to_string();
        let data_pack: Vec<u8> = soeprotocol_class.pack("DataFragment".to_owned(),data_to_pack,0,&mut rc4_obj);
        assert_eq!(
            data_pack,
            [0,13,0,2,169,183,185,67,241,64,164,5,143,19,35,87,21,163,205,26,83,24,212,247,181]
        )
    }
}
