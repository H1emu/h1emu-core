use super::crc::append_crc;
use super::soeprotocol_packets_structs::*;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::Cursor;


pub fn gen_size_error_json(rdr: Cursor<&std::vec::Vec<u8>>) -> String {
    return format!(
        r#"{{"name":"Error","error":"size","size":{},"raw":{:?}}}"#,
        rdr.get_ref().len(),
        rdr.get_ref().to_vec()
    );
}

pub fn gen_crc_error_json(vec: &Vec<u8>, expected_crc: u16, given_crc: u16) -> String {
    return format!(
        r#"{{"name":"Error","error":"crc","expected_crc":{},"given_crc":{},"raw":{:?}}}"#,
        expected_crc, given_crc, vec
    );
}

pub fn gen_corruption_error_json(
    rdr: Cursor<&std::vec::Vec<u8>>,
    subpacket_length: u32,
    data_end: u64,
) -> String {
    return format!(
        r#"{{"name":"Error","error":"corruption","subpacket_length":{},"data_end":{},"position":{},"raw":{:?}}}"#,
        subpacket_length,
        data_end,
        rdr.position() as usize,
        rdr.get_ref().to_vec()
    );
}

pub fn gen_deserializing_error_json() -> Vec<u8> {
    return vec![]; // maybe encoding a null string with error log would be better
                   /* return json!({
                       "name": "Error",
                       "error": "deserializing",
                       "raw": packet
                   })
                   .to_string();*/
}

