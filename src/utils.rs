use wasm_bindgen::prelude::*;
use rand::random;
#[wasm_bindgen]
pub fn generate_random_guid() -> String {
    let random: [u8; 8] = random();

    let mut str_bytes = vec![0u8; 16];

    const ASCII_ZERO: u8 = '0' as u8;
    const ASCII_NINE: u8 = '9' as u8;
    const ASCII_NUMBERS_LETTERS_OFFSET: u8 = 'a' as u8 - '9' as u8 - 1;

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
    use super::*;
    #[test]
    fn generate_random_guid_test() {
        println!( "{:?}" , generate_random_guid() );
        assert_ne!(generate_random_guid(), generate_random_guid())
    }
}
