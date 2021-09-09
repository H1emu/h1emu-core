mod crc_table;
use crc_table::get_crc_table;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn crc32(data: &[u8], crc_seed: usize) -> u32{
  let crc32_table = get_crc_table();
  let mut crc = crc32_table[!crc_seed & 0xff];
  println!("{}",crc);
  crc ^= 0x00ffffff;
  let mut index = (crc_seed >> 8)  ^ crc as usize;
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
        let data:[u8;5] =  [0, 21, 0, 0, 2];
        assert_eq!(crc32(&data, 0), 1874907695)
    }
}

