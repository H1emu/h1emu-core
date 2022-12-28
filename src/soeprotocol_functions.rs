use super::crc::append_crc;
use super::soeprotocol_packets_structs::*;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::Cursor;

pub enum PacketsMinSize {
    SessionRequest = 14,
    SessionReply = 21,
    Disconnect = 6,
    NetStatusPacket = 42,
    MultiPacket = 7,
    DataPacket = 5,
    Ack = 4,
}

pub fn check_min_size(rdr: &Cursor<&std::vec::Vec<u8>>, min_size: usize, use_crc: bool) -> bool {
    if use_crc {
        return rdr.get_ref().len() >= min_size + 2;
    } else {
        return rdr.get_ref().len() >= min_size;
    }
}

pub fn disconnect_reason_to_string(reason_id: u16) -> String {
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
        _ => "unknown".to_string(),
    }
}

pub fn get_data_end(rdr: &Cursor<&std::vec::Vec<u8>>, use_crc: bool) -> u64 {
    if use_crc {
        return (rdr.get_ref().len() as u64) - 2 as u64;
    } else {
        return rdr.get_ref().len() as u64;
    };
}

pub fn write_data_length(wtr: &mut Vec<u8>, data_length: usize) -> () {
    if data_length <= 0xFF {
        wtr.write_u8(data_length as u8).unwrap();
    } else if data_length <= 0xFFFF {
        wtr.write_u16::<BigEndian>(data_length as u16).unwrap();
    } else {
        wtr.write_u32::<BigEndian>(data_length as u32).unwrap();
    }
}

pub fn read_data_length(rdr: &mut Cursor<&std::vec::Vec<u8>>) -> u32 {
    let initial_rdr_position = rdr.position();
    let mut data_length: u32 = rdr.read_u8().unwrap() as u32;
    if data_length > 0xFF {
        rdr.set_position(initial_rdr_position);
        data_length = rdr.read_u16::<BigEndian>().unwrap() as u32;
        if data_length > 0xFFFF {
            rdr.set_position(initial_rdr_position);
            data_length = rdr.read_u32::<BigEndian>().unwrap() as u32;
        }
    }
    return data_length;
}

pub fn extract_subpacket_data(
    rdr: &Cursor<&std::vec::Vec<u8>>,
    data_start_position: u64,
    sub_packet_data_length: u32,
) -> Vec<u8> {
    let full_data_vec = rdr.get_ref().to_vec();
    return full_data_vec[data_start_position as usize
        ..(data_start_position + sub_packet_data_length as u64) as usize]
        .to_vec();
}

pub fn write_packet_data(
    wtr: &mut Vec<u8>,
    data_packet: &mut DataPacket,
    crc_seed: u32,
    use_crc: bool,
) -> () {
    wtr.write_u16::<BigEndian>(data_packet.sequence).unwrap();
    wtr.append(&mut data_packet.data);
    if use_crc {
        append_crc(wtr, crc_seed);
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn write_packet_data_test() {
        let data_to_pack: Vec<u8> = [
            2, 1, 1, 0, 0, 0, 1, 1, 3, 0, 0, 0, 115, 111, 101, 0, 0, 0, 0,
        ]
        .to_vec();
        let mut wtr = vec![];
        let mut data_packet = DataPacket {
            data: data_to_pack,
            sequence: 0,
            error: None,
        };
        write_packet_data(&mut wtr, &mut data_packet, 0, false);
        assert_eq!(
            wtr,
            [0, 0, 2, 1, 1, 0, 0, 0, 1, 1, 3, 0, 0, 0, 115, 111, 101, 0, 0, 0, 0].to_vec()
        )
    }

    #[test]
    fn write_packet_data_with_crc_test() {
        let data_to_pack: Vec<u8> = [
            2, 1, 1, 0, 0, 0, 1, 1, 3, 0, 0, 0, 115, 111, 101, 0, 0, 0, 0,
        ]
        .to_vec();
        let mut wtr = vec![];
        let mut data_packet = DataPacket {
            data: data_to_pack,
            sequence: 0,
            error: None,
        };
        write_packet_data(&mut wtr, &mut data_packet, 0, true);
        assert_eq!(
            wtr,
            [0, 0, 2, 1, 1, 0, 0, 0, 1, 1, 3, 0, 0, 0, 115, 111, 101, 0, 0, 0, 0, 9, 51].to_vec()
        )
    }
}
