use std::io::Cursor;

use wasm_bindgen::prelude::wasm_bindgen;

pub fn gen_size_error_json(rdr: Cursor<&std::vec::Vec<u8>>) -> String {
    format!(
        r#"{{"name":"Error","error":"size","size":{},"raw":{:?}}}"#,
        rdr.get_ref().len(),
        rdr.get_ref().to_vec()
    )
}

pub fn gen_crc_error_json(vec: &Vec<u8>, expected_crc: u16, given_crc: u16) -> String {
    format!(
        r#"{{"name":"Error","error":"crc","expected_crc":{},"given_crc":{},"raw":{:?}}}"#,
        expected_crc, given_crc, vec
    )
}

pub fn gen_corruption_error_json(
    rdr: Cursor<&std::vec::Vec<u8>>,
    subpacket_length: u32,
    data_end: u64,
) -> String {
    format!(
        r#"{{"name":"Error","error":"corruption","subpacket_length":{},"data_end":{},"position":{},"raw":{:?}}}"#,
        subpacket_length,
        data_end,
        rdr.position() as usize,
        rdr.get_ref().to_vec()
    )
}

#[wasm_bindgen]
pub enum ErrorType {
    Deserializing = 0x99,
}

pub fn gen_deserializing_error_json(error: serde_json::Error) -> Vec<u8> {
    let error_string = format!("{}", error);
    // create a vec starting with the error type and then the error stringify
    let mut error_vec: Vec<u8> = vec![0x00, ErrorType::Deserializing as u8];
    error_vec.append(&mut error_string.as_bytes().to_vec());
    error_vec
}
