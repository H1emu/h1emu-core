
use byteorder::{BigEndian, WriteBytesExt, ReadBytesExt};
use std::io::Cursor;
use serde_json::*;
use crate::utils::{str_from_u8_nul_utf8_unchecked,u8_from_str_nul_utf8_unchecked};
use serde::{Serialize,Deserialize};
use crate::crc::append_crc;
use crate::rc4::RC4;

pub fn parse_session_request(mut rdr: Cursor<&std::vec::Vec<u8>>) -> String{
    let crc_length = rdr.read_u32::<BigEndian>().unwrap();
    let session_id = rdr.read_u32::<BigEndian>().unwrap();
    let udp_length = rdr.read_u32::<BigEndian>().unwrap();
    let protocol_data_position = rdr.position() as usize;
    let raw_data = rdr.into_inner();
    unsafe {
        let protocol = str_from_u8_nul_utf8_unchecked(&raw_data[protocol_data_position..]);
        return json!({
            "crc_length": crc_length,
            "session_id": session_id,
            "udp_length": udp_length,
            "protocol": protocol, 
        }).to_string()
    }   
}

#[derive(Serialize, Deserialize)]
struct SessionRequestPacket {
    session_id: u32,
    crc_length: u32,
    udp_length: u32,
    protocol: String
}

pub fn pack_session_request(packet: String) -> Vec<u8>{
    let mut wtr = vec![];
    let packet_json: SessionRequestPacket = serde_json::from_str(&packet).unwrap();
    wtr.write_u16::<BigEndian>(0x01).unwrap();
    wtr.write_u32::<BigEndian>(packet_json.crc_length).unwrap();
    wtr.write_u32::<BigEndian>(packet_json.session_id).unwrap();
    wtr.write_u32::<BigEndian>(packet_json.udp_length).unwrap();
    wtr.append(& mut u8_from_str_nul_utf8_unchecked(packet_json.protocol.as_str()));
    return wtr;
}

pub fn parse_session_reply(mut rdr: Cursor<&std::vec::Vec<u8>>) -> String{
    return json!({
        "session_id": rdr.read_u32::<BigEndian>().unwrap(),
        "crc_seed": rdr.read_u32::<BigEndian>().unwrap(),
        "crc_length": rdr.read_u8().unwrap(),
        "compression": rdr.read_u16::<BigEndian>().unwrap(),
        "udp_length": rdr.read_u32::<BigEndian>().unwrap(),
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

pub fn parse_data(mut rdr: Cursor<&std::vec::Vec<u8>>, _rc4: &mut RC4) -> String{
    let sequence =  rdr.read_u16::<BigEndian>().unwrap();
    let data_end = (rdr.get_ref().len() as u64) - 2 as u64;
    rdr.set_position(data_end);
    let crc = rdr.read_u16::<BigEndian>().unwrap();
    let vec = rdr.into_inner();
    let data = &vec[4..data_end as usize]; // for now since it's only mean to be used in h1emu, the data isn't deciphered but will at some point.
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

pub fn pack_data(packet: String,crc_seed: u32, _use_compression: bool, mut rc4: &mut RC4) -> Vec<u8>{
    let mut wtr = vec![];
    let packet_json: DataPacket = serde_json::from_str(&packet).unwrap();

    wtr.write_u16::<BigEndian>(0x09).unwrap();
    write_packet_data(&mut wtr, packet_json, crc_seed, _use_compression, &mut rc4);
    return wtr;
}

pub fn pack_fragment_data(packet: String,crc_seed: u32, _use_compression: bool, rc4: &mut RC4) -> Vec<u8>{
    let mut wtr = vec![];
    let packet_json: DataPacket = serde_json::from_str(&packet).unwrap();

    wtr.write_u16::<BigEndian>(0x0d).unwrap();
    write_packet_data(&mut wtr, packet_json, crc_seed, _use_compression, rc4);
    return wtr;
}

fn write_packet_data(wtr : &mut Vec<u8>,data_packet : DataPacket,crc_seed: u32, _use_compression: bool, rc4:&mut RC4) -> (){
    wtr.write_u16::<BigEndian>(data_packet.sequence).unwrap();
    wtr.append(&mut rc4.encrypt(data_packet.data));
    append_crc(wtr, crc_seed as usize);
}

pub fn parse_ack(mut rdr: Cursor<&std::vec::Vec<u8>>) -> String{
    let sequence =  rdr.read_u16::<BigEndian>().unwrap();
    return json!({
        "channel": 0,
        "sequence": sequence,
      }).to_string()
}


#[derive(Serialize, Deserialize)]
struct AckPacket {
    sequence: u16,
}

pub fn pack_out_of_order(packet: String) -> Vec<u8>{
    let mut wtr = vec![];
    let packet_json: AckPacket = serde_json::from_str(&packet).unwrap();

    wtr.write_u16::<BigEndian>(0x11).unwrap();
    wtr.write_u16::<BigEndian>(packet_json.sequence).unwrap();
    return wtr;
}



pub fn pack_ack(packet: String) -> Vec<u8>{
    let mut wtr = vec![];
    let packet_json: AckPacket = serde_json::from_str(&packet).unwrap();

    wtr.write_u16::<BigEndian>(0x15).unwrap();
    wtr.write_u16::<BigEndian>(packet_json.sequence).unwrap();
    return wtr;
}


#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn write_packet_data_test() {
        let key: [u8; 16] = [
            23, 189,   8, 107, 27, 148,
           240,  47, 240, 236, 83, 215,
            99,  88, 155,  95
         ];
        let mut rc4_obj = RC4::initialize(key.to_vec());
        let data_to_pack:Vec<u8> = [2,1,1,0,0,0,1,1,3,0,0,0,115,111,101,0,0,0,0].to_vec();
        let mut wtr = vec![];
        let data_packet = DataPacket {
            data: data_to_pack,
            sequence: 0,
        };
       write_packet_data(&mut wtr, data_packet,0,false, &mut rc4_obj);
        assert_eq!(
            wtr,
            [0,0,169,183,185,67,241,64,164,5,143,19,35,87,21,163,205,26,83,24,212, 12, 197].to_vec()
        )
    }
}
