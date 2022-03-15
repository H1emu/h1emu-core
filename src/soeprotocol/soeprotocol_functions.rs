
use byteorder::{BigEndian, WriteBytesExt, ReadBytesExt};
use std::io::Cursor;
use serde_json::*;
use serde::{Serialize,Deserialize};
use crate::rc4::RC4;

pub fn parse_session_request(mut rdr: Cursor<&std::vec::Vec<u8>>) -> String{
    return json!({
        "crcLength": rdr.read_u32::<BigEndian>().unwrap(),
        "sessionId": rdr.read_u32::<BigEndian>().unwrap(),
        "udpLength": rdr.read_u32::<BigEndian>().unwrap(),
        "protocol": "LoginUdp_9", // TODO
    }).to_string()
}

#[derive(Serialize, Deserialize)]
struct SessionRequestPacket {
    session_id: u32,
    crc_seed: u32,
    udp_length: u32
}

pub fn pack_session_request(packet: String) -> Vec<u8>{
    let mut wtr = vec![];
    let packet_json: SessionRequestPacket = serde_json::from_str(&packet).unwrap();

    wtr.write_u16::<BigEndian>(0x01).unwrap();
    wtr.write_u32::<BigEndian>(packet_json.session_id).unwrap();
    wtr.write_u32::<BigEndian>(packet_json.crc_seed).unwrap();
    wtr.write_u32::<BigEndian>(packet_json.udp_length).unwrap();
  // protocol
    return wtr;
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

#[derive(Serialize, Deserialize)]
struct SessionReplyPacket {
    session_id: u32,
    crc_seed: u32,
    crc_length: u8,
    compression: u16,
    udp_length: u32
}

pub fn pack_session_reply(packet: String) -> Vec<u8>{
    let mut wtr = vec![];
    let packet_json: SessionReplyPacket = serde_json::from_str(&packet).unwrap();

    wtr.write_u16::<BigEndian>(0x02).unwrap();
    wtr.write_u32::<BigEndian>(packet_json.session_id).unwrap();
    wtr.write_u32::<BigEndian>(packet_json.crc_seed).unwrap();
    wtr.write_u8(packet_json.crc_length).unwrap();
    wtr.write_u16::<BigEndian>(packet_json.compression).unwrap();
    wtr.write_u32::<BigEndian>(packet_json.udp_length).unwrap();
    wtr.write_u32::<BigEndian>(3).unwrap();
    return wtr;
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

#[derive(Serialize, Deserialize)]
struct DataPacket {
    data: Vec<u8>,
    sequence: u16,
}

pub fn pack_data(packet: String) -> Vec<u8>{
    let mut wtr = vec![];
    let packet_json: DataPacket = serde_json::from_str(&packet).unwrap();

    wtr.write_u16::<BigEndian>(0x09).unwrap();
    wtr.write_u16::<BigEndian>(packet_json.sequence).unwrap();

    //write data

    // append crc
    return wtr;
}

pub fn parse_ack(mut rdr: Cursor<&std::vec::Vec<u8>>, _rc4: RC4) -> String{
    let sequence =  rdr.read_u16::<BigEndian>().unwrap();
    return json!({
        "channel": 0,
        "sequence": sequence,
      }).to_string()
}


#[derive(Serialize, Deserialize)]
struct OutOfOrderPacket {
    sequence: u16,
}

pub fn pack_OutOfOrder(packet: String) -> Vec<u8>{
    let mut wtr = vec![];
    let packet_json: DataPacket = serde_json::from_str(&packet).unwrap();

    wtr.write_u16::<BigEndian>(0x11).unwrap();
    wtr.write_u16::<BigEndian>(packet_json.sequence).unwrap();
    // append crc
    return wtr;
}

#[derive(Serialize, Deserialize)]
struct AckPacket {
    sequence: u16,
}

pub fn pack_ack(packet: String) -> Vec<u8>{
    let mut wtr = vec![];
    let packet_json: DataPacket = serde_json::from_str(&packet).unwrap();

    wtr.write_u16::<BigEndian>(0x15).unwrap();
    wtr.write_u16::<BigEndian>(packet_json.sequence).unwrap();
    // append crc
    return wtr;
}

