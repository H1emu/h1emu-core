use wasm_bindgen::prelude::*;
// https://en.wikipedia.org/wiki/Jenkins_hash_function#one_at_a_time
#[wasm_bindgen]
pub fn joaat(string: &str) -> u32{
    let mut hash:u32 = 0;
    for &i in string.as_bytes() {
        hash += i as u32;
        hash += hash << 10;
        hash ^= hash >> 6;
    }
    hash += hash << 3;
    hash ^= hash >> 11;
    hash += hash << 15;
    return hash >> 0;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn jenkins_oaat_test() {
        assert_eq!(joaat("HAX"), 2813495259)
    }
}

