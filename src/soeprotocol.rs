use byteorder::{BigEndian, WriteBytesExt};
use wasm_bindgen::prelude::*;

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
    fn session_reply_test() {
        let data: Vec<u8> = session_reply(255993749, 0, 2, 256, 512);
        assert_eq!(
            data,
            [0, 2, 15, 66, 39, 149, 0, 0, 0, 0, 2, 1, 0, 0, 0, 2, 0, 0, 0, 0, 3]
        )
    }
}
