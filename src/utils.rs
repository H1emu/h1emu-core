use rand::random;
use wasm_bindgen::prelude::*;
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

pub unsafe fn str_from_u8_nul_utf8_unchecked(utf8_src: &[u8]) -> &str {
    let mut nul_range_end = 1_usize;
    for b in utf8_src {
        if *b == 0 {
            break;
        }
        nul_range_end += 1;
    }
    return ::std::str::from_utf8_unchecked(&utf8_src[0..nul_range_end - 1]);
}

pub fn u8_from_str_nul_utf8_unchecked(data: &str) -> Vec<u8> {
    let mut data_u8 = vec![];

    for b in data.chars() {
        data_u8.push(b as u8);
    }
    data_u8.push(0);
    return data_u8;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn generate_random_guid_test() {
        assert_ne!(generate_random_guid(), generate_random_guid())
    }
}
