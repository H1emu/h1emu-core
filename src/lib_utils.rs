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
    data_u8
}

pub fn read_prefixed_string_le(buffer: &[u8], offset: usize, length: u32) -> String {
    let string_bytes = &buffer[offset + 4..offset + 4 + length as usize];

    String::from_utf8(string_bytes.to_vec()).unwrap()
}
pub fn sat(x: f32) -> f32 {
    if x < 0.0 {
        return 0.0;
    } else if x > 1.0 {
        return 1.0;
    } else {
        return x;
    }
}
