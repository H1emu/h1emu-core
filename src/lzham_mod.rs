use wasm_bindgen::prelude::*;
use lzham::{compress, decompress};

#[wasm_bindgen]
pub fn lzham_decomp(data: &[u8]) -> Vec<u8>{
  let mut original_data = data;
  let mut decomp = Vec::new();
  decompress(&mut original_data,&mut decomp,data.len());
  return decomp;
}

#[wasm_bindgen]
pub fn lzham_comp(data: &[u8]) -> Vec<u8>{
  let mut original_data = data;
  let mut comp = Vec::new();
  compress(&mut original_data,&mut comp);
  return comp;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn lzham_comp_decomp_test() {
        let hello_world_compressed_bytes = [128, 0, 2, 130, 128, 104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100, 192, 26, 11, 4, 93];
        let hello_world_uncompressed_bytes = [104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100, 8, 25, 127, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(lzham_decomp(&hello_world_compressed_bytes), hello_world_uncompressed_bytes)
    }
    #[test]
    fn lzham_comp_test() {
      let hello_world_compressed_bytes = [128, 0, 2, 130, 128, 104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100, 192, 26, 11, 4, 93];
        assert_eq!(lzham_comp("hello world".as_bytes()), &hello_world_compressed_bytes)
    }
}

