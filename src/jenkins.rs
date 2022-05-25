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
    hash = hash.wrapping_add(hash <<15);
    return hash >> 0;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn jenkins_oaat_test() {
        assert_eq!(joaat("HAX"), 2813495259)
    }
    #[test]
    fn jenkins_oaat_wikipedia_tests() {
        assert_eq!(joaat("The quick brown fox jumps over the lazy dog"), 0x519e91f5);
        assert_eq!(joaat("a"), 0xca2e9442)
    }
}
