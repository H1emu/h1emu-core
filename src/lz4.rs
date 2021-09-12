use wasm_bindgen::prelude::*;
use lz4_compression::prelude::*;

#[wasm_bindgen]
pub fn lz4_decomp(data: &[u8]) -> Vec<u8>{
  return decompress(&data).unwrap();
}

#[wasm_bindgen]
pub fn lz4_comp(data: &[u8]) -> Vec<u8>{
  return compress(&data);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn lz4_decomp_test() {
        assert_eq!(lz4_decomp(&[0x11, 2, 1, 0, 0]), [2, 2, 2, 2, 2, 2])
    }
    #[test]
    fn lz4_comp_test() {
        assert_eq!(lz4_comp(&[2, 2, 2, 2, 2, 2]), &[0x11, 2, 1, 0, 0])
    }
}

