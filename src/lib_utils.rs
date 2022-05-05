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
