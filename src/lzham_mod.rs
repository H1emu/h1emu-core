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
        assert_eq!(lzham_decomp(&[0x11, 2, 1, 0, 0]), "hello world".as_bytes())
    }
    #[test]
    fn lzham_comp_test() {
        assert_eq!(lzham_comp("hello world".as_bytes()), &[0x11, 2, 1, 0, 0])
    }
}

