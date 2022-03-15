
use byteorder::{BigEndian, WriteBytesExt, ReadBytesExt};
use std::io::Cursor;
use serde_json::*;
use crate::rc4::RC4;

pub fn parse_session_request(mut rdr: Cursor<&std::vec::Vec<u8>>) -> String{
    return json!({
        "crcLength": rdr.read_u32::<BigEndian>().unwrap(),
        "sessionId": rdr.read_u32::<BigEndian>().unwrap(),
        "udpLength": rdr.read_u32::<BigEndian>().unwrap(),
        "protocol": "LoginUdp_9", // TODO
    }).to_string()
}

pub fn parse_session_reply(mut rdr: Cursor<&std::vec::Vec<u8>>) -> String{
    return json!({
        "sessionId": rdr.read_u32::<BigEndian>().unwrap(),
        "crcSeed": rdr.read_u32::<BigEndian>().unwrap(),
        "crcLength": rdr.read_u8().unwrap(),
        "compression": rdr.read_u16::<BigEndian>().unwrap(),
        "udpLength": rdr.read_u32::<BigEndian>().unwrap(),
    }).to_string()
}

pub fn parse_data(mut rdr: Cursor<&std::vec::Vec<u8>>, _rc4: RC4) -> String{
    let sequence =  rdr.read_u16::<BigEndian>().unwrap();
    let data_end = (rdr.get_ref().len() as u64) - 2 as u64;
    rdr.set_position(data_end);
    let crc = rdr.read_u16::<BigEndian>().unwrap();
    let vec = rdr.into_inner();
    let data = &vec[4..data_end as usize];
    println!("{:?}",data);
    return json!({
        "channel": 0,
        "sequence": sequence,
        "crc": crc,
        "data": data,
    }).to_string()
}

pub fn parse_data_fragment(mut rdr: Cursor<&std::vec::Vec<u8>>, _rc4: RC4) -> String{
    let sequence =  rdr.read_u16::<BigEndian>().unwrap();
    let data_end = (rdr.get_ref().len() as u64) - 2 as u64;
    rdr.set_position(data_end);
    let crc = rdr.read_u16::<BigEndian>().unwrap();
    let vec = rdr.into_inner();
    let data = &vec[4..data_end as usize];
    println!("{:?}",data);
    return json!({
        "channel": 0,
        "sequence": sequence,
        "crc": crc,
        "data": data,
    }).to_string()
}


pub fn parse_ack(mut rdr: Cursor<&std::vec::Vec<u8>>, _rc4: RC4) -> String{
    let sequence =  rdr.read_u16::<BigEndian>().unwrap();
    return json!({
        "channel": 0,
        "sequence": sequence,
      }).to_string()
}