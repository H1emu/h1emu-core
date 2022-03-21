mod crc_table;
use byteorder::{BigEndian, WriteBytesExt};
use crc_table::get_crc_table;
use wasm_bindgen::prelude::*;

pub fn append_crc(data: &mut Vec<u8>, crc_seed: usize) -> () {
    let crc = crc32(&data, crc_seed >> 0);
    data
        .write_u16::<BigEndian>((crc & 0xffff) as u16)
        .unwrap();
}

pub fn crc32(data: &&mut Vec<u8>, crc_seed: usize) -> u32 {
    let crc32_table = get_crc_table();
    let mut crc = crc32_table[!crc_seed & 0xff];
    crc ^= 0x00ffffff;
    let mut index = (crc_seed >> 8) ^ crc as usize;
    crc = (crc >> 8) & 0x00ffffff;
    crc ^= crc32_table[index & 0xff];
    index = (crc_seed >> 16) ^ crc as usize;
    crc = (crc >> 8) & 0x00ffffff;
    crc ^= crc32_table[index & 0xff];
    index = (crc_seed >> 24) ^ crc as usize;
    crc = (crc >> 8) & 0x00ffffff;
    crc ^= crc32_table[index & 0xff];
    for i in 0..data.len() {
        index = data[i] as usize ^ crc as usize;
        crc = (crc >> 8) & 0x00ffffff;
        crc ^= crc32_table[index & 0xff];
    }
    return !crc >> 0;
}

#[wasm_bindgen]
pub fn append_crc_legacy(data: &[u8], crc_seed: usize) -> std::vec::Vec<u8> {
    let mut data_mut = data.to_vec();
    let crc = crc32_legacy(&data_mut, crc_seed >> 0);
    data_mut
        .write_u16::<BigEndian>((crc & 0xffff) as u16)
        .unwrap();
    return data_mut;
}

#[wasm_bindgen]
pub fn crc32_legacy(data: &[u8], crc_seed: usize) -> u32 {
    let crc32_table = get_crc_table();
    let mut crc = crc32_table[!crc_seed & 0xff];
    crc ^= 0x00ffffff;
    let mut index = (crc_seed >> 8) ^ crc as usize;
    crc = (crc >> 8) & 0x00ffffff;
    crc ^= crc32_table[index & 0xff];
    index = (crc_seed >> 16) ^ crc as usize;
    crc = (crc >> 8) & 0x00ffffff;
    crc ^= crc32_table[index & 0xff];
    index = (crc_seed >> 24) ^ crc as usize;
    crc = (crc >> 8) & 0x00ffffff;
    crc ^= crc32_table[index & 0xff];
    for i in 0..data.len() {
        index = data[i] as usize ^ crc as usize;
        crc = (crc >> 8) & 0x00ffffff;
        crc ^= crc32_table[index & 0xff];
    }
    return !crc >> 0;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn crc32_test() {
        let mut data: Vec<u8> = [0, 21, 0, 0, 2].to_vec();
        crc32(&&mut data,0);
        assert_eq!(crc32(&&mut data,0), 1874907695)
    }
    #[test]
    fn append_crc_test() {
        let mut data: Vec<u8> = [
            0, 9, 0, 0, 0, 169, 183, 185, 67, 241, 64, 164, 5, 143, 19, 35, 87, 21, 163, 205, 26,
            83, 24, 212,
        ].to_vec();
        append_crc(&mut data, 0);
        assert_eq!(
            data,
            [
                0, 9, 0, 0, 0, 169, 183, 185, 67, 241, 64, 164, 5, 143, 19, 35, 87, 21, 163, 205,
                26, 83, 24, 212, 220, 81
            ]
        )
    }
    #[test]
    fn crc32_legacy_test() {
        let data: [u8; 5] = [0, 21, 0, 0, 2];
        assert_eq!(crc32_legacy(&data, 0), 1874907695)
    }
    #[test]
    fn append_crc_legacy_test() {
        let data: [u8; 24] = [
            0, 9, 0, 0, 0, 169, 183, 185, 67, 241, 64, 164, 5, 143, 19, 35, 87, 21, 163, 205, 26,
            83, 24, 212,
        ];
        assert_eq!(
            append_crc_legacy(&data, 0),
            [
                0, 9, 0, 0, 0, 169, 183, 185, 67, 241, 64, 164, 5, 143, 19, 35, 87, 21, 163, 205,
                26, 83, 24, 212, 220, 81
            ]
        )
    }
}
