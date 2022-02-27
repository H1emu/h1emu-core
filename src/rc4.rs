#![deny(unconditional_panic)]


use std::mem;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct RC4 {
    s: Vec<u8>,
}

#[wasm_bindgen]
impl RC4 {
    #[wasm_bindgen]
    pub fn Initialize(given_key:Vec<u8>,key_size:u8 ) -> RC4 {
        let mut rc4 = RC4 { s: vec![0; 256] };

        for i in 0..255 {
            rc4.s[i] = i as u8;
        }

        let mut j:u8 = 0;
        for i in 0..key_size {

            j += (given_key.get( i as usize % key_size as usize).unwrap() + rc4.s.get(i as usize).unwrap() + j).rem_euclid(255);
            println!("j : {}",j);
            let saved = rc4.s[ i as usize];

            rc4.s[ i as usize] = rc4.s[ j as usize];

            rc4.s[ j as usize] = saved;
        }
      //  rc4.key = j;
        return rc4
    }

    #[wasm_bindgen]
    pub fn Encrypt(mut self, data: Vec<u8>, data_size: u32) -> Vec<u8> {
        let mut i:u8 = 0;
        let mut j:u8 = 0;

        let mut new_data:Vec<u8> = Vec::new();
        for n in 0..data_size {
            i = (i + 1).rem_euclid(255);
            j = (j + self.s[i as usize]).rem_euclid(255);

            // swap
           // mem::swap(& mut self.sbox[i as usize], & mut self.sbox[j as usize]);

            let saved = self.s[ i as usize];

            self.s[ i as usize] = self.s[ j as usize];

            self.s[ j as usize] = saved;
            drop(saved);

            let rnd = self.s[((self.s[i as usize] + self.s[j as usize])) as usize].rem_euclid(255);

            new_data.push(rnd ^data[n as usize]);
           // new_data[n as usize] = 
        }

        return new_data;
    }
    
    #[wasm_bindgen]
    pub fn Decrypt(mut self, data: Vec<u8>, data_size: u32) -> Vec<u8> {
        return self.Encrypt(data, data_size);
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rc4_enc() {
        let key: [u8; 16] = [17,189,08,107,27,94,240,47,240,236,53,215,63,58,155,95];
        let data: [u8; 13] = [06,01,00,00,00,108,47,75,14,192,126,17,211];
        let data_real_result: [u8; 13] = [29, 182, 15, 82 ,208 ,200, 197, 57, 234, 17, 03 ,193, 244];
        let data_vec = data.to_vec();
        let rc4_obj = RC4::Initialize(key.to_vec(), key.len() as u8);
        let data_result = rc4_obj.Encrypt(data_vec, 13);
       // let mut data_result2 = rc4_obj.Decrypt(data_result, data_result.len() as u32);
        assert_eq!(
            data_result,
            data_real_result.to_vec()
        )
    }
}
