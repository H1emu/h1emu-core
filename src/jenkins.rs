use wasm_bindgen::prelude::*;
// https://en.wikipedia.org/wiki/Jenkins_hash_function#one_at_a_time
#[wasm_bindgen]
pub fn joaat(string: &str) -> u32 {
    let mut hash: u32 = 0;
    for &i in string.as_bytes() {
        hash = hash.wrapping_add(i as u32);
        hash = hash.wrapping_add(hash << 10);
        hash ^= hash >> 6;
    }
    hash = hash.wrapping_add(hash << 3);
    hash ^= hash >> 11;
    hash = hash.wrapping_add(hash << 15);
    hash
}

#[cfg(test)]
mod tests {

    #[test]
    fn jenkins_oaat_test() {
        assert_eq!(super::joaat("HAX"), 2813495259)
    }
    #[test]
    fn jenkins_oaat_wikipedia_tests() {
        assert_eq!(
            super::joaat("The quick brown fox jumps over the lazy dog"),
            0x519e91f5
        );
        assert_eq!(super::joaat("a"), 0xca2e9442)
    }
}
