
use byteorder::{BigEndian, WriteBytesExt, ReadBytesExt};
use std::io::Cursor;
use serde_json::*;
use crate::utils::{str_from_u8_nul_utf8_unchecked,u8_from_str_nul_utf8_unchecked};
use serde::{Serialize,Deserialize};
use crate::crc::append_crc;
use crate::rc4::RC4;
use crate::Soeprotocol;

pub fn parse_session_request(mut rdr: Cursor<&std::vec::Vec<u8>>) -> String{
    let crc_length = rdr.read_u32::<BigEndian>().unwrap();
    let session_id = rdr.read_u32::<BigEndian>().unwrap();
    let udp_length = rdr.read_u32::<BigEndian>().unwrap();
    let protocol_data_position = rdr.position() as usize;
    let raw_data = rdr.into_inner();
    unsafe {
        let protocol = str_from_u8_nul_utf8_unchecked(&raw_data[protocol_data_position..]);
        return json!({
            "name": "SessionRequest",
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
        "name": "SessionReply",
        "session_id": rdr.read_u32::<BigEndian>().unwrap(),
        "crc_seed": rdr.read_u32::<BigEndian>().unwrap(),
        "crc_length": rdr.read_u8().unwrap(),
        "encrypt_method": rdr.read_u16::<BigEndian>().unwrap(),
        "udp_length": rdr.read_u32::<BigEndian>().unwrap(),
    }).to_string()
}


fn disconnect_reason_to_string(reason_id: u16) -> String {
    match reason_id {
        0 => "DisconnectReasonIcmpError".to_string(),
        1 => "DisconnectReasonTimeout".to_string(),
        2 => "DisconnectReasonNone".to_string(),
        3 => "DisconnectReasonOtherSideTerminated".to_string(),
        4 => "DisconnectReasonManagerDeleted".to_string(),
        5 => "DisconnectReasonConnectFail".to_string(),
        6 => "DisconnectReasonApplication".to_string(),
        7 => "DisconnectReasonUnreachableConnection".to_string(),
        8 => "DisconnectReasonUnacknowledgedTimeout".to_string(),
        9 => "DisconnectReasonNewConnectionAttempt".to_string(),
        10 => "DisconnectReasonConnectionRefused".to_string(),
        11 => "DisconnectReasonConnectErro".to_string(),
        12 => "DisconnectReasonConnectingToSelf".to_string(),
        13 => "DisconnectReasonReliableOverflow".to_string(),
        14 => "DisconnectReasonApplicationReleased".to_string(),
        15 => "DisconnectReasonCorruptPacket".to_string(),
        16 => "DisconnectReasonProtocolMismatch".to_string(),
        _ => "unknown".to_string()
    }
}

fn get_data_end(mut rdr: & Cursor<&std::vec::Vec<u8>>, use_crc:bool) -> u64{
    if use_crc {
        return (rdr.get_ref().len() as u64) - 2 as u64;
    } else {
        return rdr.get_ref().len() as u64;
    };
}

pub fn parse_disconnect(mut rdr: Cursor<&std::vec::Vec<u8>>) -> String{
    return json!({
        "name": "Disconnect",
        "session_id": rdr.read_u32::<BigEndian>().unwrap(),
        "reason": disconnect_reason_to_string(rdr.read_u16::<BigEndian>().unwrap()),
        "unk": rdr.read_u16::<BigEndian>().unwrap(),
    }).to_string()
}

#[derive(Serialize, Deserialize)]
struct SessionReplyPacket {
    session_id: u32,
    crc_seed: u32,
    crc_length: u8,
    encrypt_method: u16,
    udp_length: u32
}

pub fn pack_session_reply(packet: String) -> Vec<u8>{
    let mut wtr = vec![];
    let packet_json: SessionReplyPacket = serde_json::from_str(&packet).unwrap();

    wtr.write_u16::<BigEndian>(0x02).unwrap();
    wtr.write_u32::<BigEndian>(packet_json.session_id).unwrap();
    wtr.write_u32::<BigEndian>(packet_json.crc_seed).unwrap();
    wtr.write_u8(packet_json.crc_length).unwrap();
    wtr.write_u16::<BigEndian>(packet_json.encrypt_method).unwrap();
    wtr.write_u32::<BigEndian>(packet_json.udp_length).unwrap();
    wtr.write_u32::<BigEndian>(3).unwrap();
    return wtr;
}

fn read_data_length(rdr: &mut Cursor<&std::vec::Vec<u8>>) -> u64{
    let initial_rdr_position = rdr.position();
    let mut data_length = rdr.read_u8().unwrap() as u64;
    if data_length > 0xFF {
        rdr.set_position(initial_rdr_position);
        data_length = rdr.read_u16::<BigEndian>().unwrap() as u64;
        if data_length > 0xFFFF{
            rdr.set_position(initial_rdr_position);
            data_length = rdr.read_u32::<BigEndian>().unwrap() as u64;
        }
    }
    return data_length;
}

fn extract_subpacket_data(rdr: &Cursor<&std::vec::Vec<u8>>,data_start_position:u64 ,sub_packet_data_length : u64) -> Vec<u8>{
    let copy_rdr = rdr.clone();
    let full_data_vec = copy_rdr.into_inner();
    return full_data_vec[data_start_position as usize..(data_start_position + sub_packet_data_length) as usize].to_vec();
}

pub fn parse_multi(mut rdr: Cursor<&std::vec::Vec<u8>>,soeprotocol : &mut Soeprotocol,rc4 :&mut RC4) -> String{
    let mut sub_packets: Vec<Value> = vec![];
    let data_end:u64 = get_data_end(&rdr,soeprotocol.use_crc);
    let was_crc_enabled = soeprotocol.use_crc;
    if was_crc_enabled {
        soeprotocol.disable_crc();
    }
    loop {
        let sub_packet_data_length = read_data_length(&mut rdr);
        let sub_packet_data = extract_subpacket_data(&rdr,rdr.position(),sub_packet_data_length);
        rdr.set_position(sub_packet_data_length + rdr.position());
        println!("sub_packet_data_length: {}",sub_packet_data_length);
        println!("sub_packet_data: {:?}",sub_packet_data);
        let sub_packet = soeprotocol.parse(sub_packet_data,rc4);
        sub_packets.push(serde_json::from_str(&sub_packet).unwrap());
        if rdr.position() == data_end {
            break;
        }
    }
    if was_crc_enabled {
        soeprotocol.enable_crc();
    }
    return json!({
        "sub_packets": sub_packets,
    }).to_string()
}

#[derive(Serialize, Deserialize)]
struct SubBasePacket {
    name: String
}
// pack multi packets
pub fn pack_multi(packet:String,soeprotocol : &mut Soeprotocol,crc_seed : u8,rc4 :&mut RC4) -> Vec<u8>{
    let packets: Vec<String> = serde_json::from_str(&packet).unwrap();
    let mut wtr = vec![];
    let was_crc_enabled = soeprotocol.use_crc;
    if was_crc_enabled {
        soeprotocol.disable_crc();
    }
    for packet in packets {
        let packet_json: SubBasePacket = serde_json::from_str(&packet).unwrap();
        let mut packet_data = soeprotocol.pack(packet_json.name,packet,crc_seed,rc4);
        wtr.append(&mut packet_data);
    }
    if was_crc_enabled {
        soeprotocol.enable_crc();
        append_crc(&mut wtr, crc_seed)
    }
    return wtr;
}
pub fn parse_data(mut rdr: Cursor<&std::vec::Vec<u8>>,use_crc: bool ,mut _rc4: &mut RC4,opcode : u16) -> String{
    let name = if opcode == 0x09 {
        "Data"
    } else {
        "DataFragment"
    };
    let sequence =  rdr.read_u16::<BigEndian>().unwrap();

    let data_end:u64 = get_data_end(&rdr,use_crc);
    let mut crc: u16 = 0;
     if  use_crc {
        rdr.set_position(data_end);
        crc = rdr.read_u16::<BigEndian>().unwrap();
    } 
    let vec = rdr.into_inner();
    let data = &vec[4..data_end as usize]; // for now since it's only mean to be used in h1emu, the data isn't deciphered but will at some point.
    return json!({
        "name": name,
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

pub fn pack_data(packet: String,crc_seed: u8, use_crc: bool, mut rc4: &mut RC4, use_encryption: bool) -> Vec<u8>{
    let mut wtr = vec![];
    let mut packet_json: DataPacket = serde_json::from_str(&packet).unwrap();

    wtr.write_u16::<BigEndian>(0x09).unwrap();
    write_packet_data(&mut wtr, &mut packet_json, crc_seed, use_crc, &mut rc4, use_encryption);
    return wtr;
}

pub fn pack_fragment_data(packet: String,crc_seed: u8, use_crc: bool,mut rc4: &mut RC4, use_encryption: bool) -> Vec<u8>{
    let mut wtr = vec![];
    let mut packet_json: DataPacket = serde_json::from_str(&packet).unwrap();

    wtr.write_u16::<BigEndian>(0x0d).unwrap();
    write_packet_data(&mut wtr, &mut packet_json, crc_seed, use_crc, &mut rc4, use_encryption);
    return wtr;
}

fn write_packet_data(wtr : &mut Vec<u8>,data_packet : &mut DataPacket,crc_seed: u8, use_crc: bool, rc4:&mut RC4, use_encryption: bool) -> (){
    wtr.write_u16::<BigEndian>(data_packet.sequence).unwrap();
    if use_encryption {
        wtr.append(&mut rc4.encrypt(data_packet.data.to_owned()));
    }
    else {
        wtr.append(&mut data_packet.data);
    }
    if use_crc {
        append_crc(wtr, crc_seed);
    }
}

pub fn parse_ack(mut rdr: Cursor<&std::vec::Vec<u8>>,opcode : u16, _use_crc:bool) -> String{
    let name = if opcode == 0x15 {
        "Ack"
    } else {
        "OutOfOrder"
    };
    let sequence =  rdr.read_u16::<BigEndian>().unwrap();
    // read and verify crc if needed
    return json!({
        "name": name,
        "channel": 0,
        "sequence": sequence,
      }).to_string()
}


#[derive(Serialize, Deserialize)]
struct AckPacket {
    sequence: u16,
}

pub fn pack_out_of_order(packet: String,crc_seed: u8,use_crc: bool) -> Vec<u8>{
    let mut wtr = vec![];
    let packet_json: AckPacket = serde_json::from_str(&packet).unwrap();

    wtr.write_u16::<BigEndian>(0x11).unwrap();
    wtr.write_u16::<BigEndian>(packet_json.sequence).unwrap();
    if use_crc {
        append_crc(&mut wtr, crc_seed);
    }
    return wtr;
}



pub fn pack_ack(packet: String,crc_seed: u8,use_crc: bool) -> Vec<u8>{
    let mut wtr = vec![];
    let packet_json: AckPacket = serde_json::from_str(&packet).unwrap();

    wtr.write_u16::<BigEndian>(0x15).unwrap();
    wtr.write_u16::<BigEndian>(packet_json.sequence).unwrap();
    if use_crc {
        append_crc(&mut wtr, crc_seed);
    }
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
        let mut data_packet = DataPacket {
            data: data_to_pack,
            sequence: 0,
        };
       write_packet_data(&mut wtr, &mut data_packet,0,false, &mut rc4_obj,false);
        assert_eq!(
            wtr,
            [0, 0, 2, 1, 1, 0, 0, 0, 1, 1, 3, 0, 0, 0, 115, 111, 101, 0, 0, 0, 0].to_vec()
        )
    }

    #[test]
    fn write_packet_data_with_encryption_test() {
        let key: [u8; 16] = [
            23, 189,   8, 107, 27, 148,
           240,  47, 240, 236, 83, 215,
            99,  88, 155,  95
         ];
        let mut rc4_obj = RC4::initialize(key.to_vec());
        let data_to_pack:Vec<u8> = [2,1,1,0,0,0,1,1,3,0,0,0,115,111,101,0,0,0,0].to_vec();
        let mut wtr = vec![];
        let mut data_packet = DataPacket {
            data: data_to_pack,
            sequence: 0,
        };
       write_packet_data(&mut wtr, &mut data_packet,0,false, &mut rc4_obj,true);
        assert_eq!(
            wtr,
            [0,0,169,183,185,67,241,64,164,5,143,19,35,87,21,163,205,26,83,24,212].to_vec()
        )
    }

    #[test]
    fn write_packet_data_with_crc_test() {
        let key: [u8; 16] = [
            23, 189,   8, 107, 27, 148,
           240,  47, 240, 236, 83, 215,
            99,  88, 155,  95
         ];
        let mut rc4_obj = RC4::initialize(key.to_vec());
        let data_to_pack:Vec<u8> = [2,1,1,0,0,0,1,1,3,0,0,0,115,111,101,0,0,0,0].to_vec();
        let mut wtr = vec![];
        let mut data_packet = DataPacket {
            data: data_to_pack,
            sequence: 0,
        };
       write_packet_data(&mut wtr, &mut data_packet,0,true, &mut rc4_obj,false);
        assert_eq!(
            wtr,
            [0, 0, 2, 1, 1, 0, 0, 0, 1, 1, 3, 0, 0, 0, 115, 111, 101, 0, 0, 0, 0, 9, 51].to_vec()
        )
    }
}
