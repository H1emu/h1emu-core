use rand::random;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
pub fn generate_random_guid() -> String {
    let random: [u8; 8] = random();

    let mut str_bytes = vec![0u8; 16];

    const ASCII_ZERO: u8 = b'0';
    const ASCII_NINE: u8 = b'9';
    const ASCII_NUMBERS_LETTERS_OFFSET: u8 = b'a' - b'9' - 1;

    for i in 0..8 {
        let mut leading = random[i] / 16 + ASCII_ZERO;
        let mut trailing = random[i] % 16 + ASCII_ZERO;

        leading += ((leading > ASCII_NINE) as u8) * ASCII_NUMBERS_LETTERS_OFFSET;
        trailing += ((trailing > ASCII_NINE) as u8) * ASCII_NUMBERS_LETTERS_OFFSET;

        str_bytes[2 * i] = leading;
        str_bytes[2 * i + 1] = trailing;
    }

    unsafe { String::from_utf8_unchecked(str_bytes) }
}

#[cfg(test)]
mod tests {

    #[test]
    fn generate_random_guid_test() {
        assert_ne!(super::generate_random_guid(), super::generate_random_guid())
    }
}
