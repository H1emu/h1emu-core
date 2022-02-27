use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct RC4 {
    s: Vec<u32>,
    i: u32,
    j: u32,
}

#[wasm_bindgen]
impl RC4 {
    #[wasm_bindgen(constructor)]
    pub fn initialize(given_key:Vec<u32>,key_size:u8 ) -> RC4 {
        let mut rc4 = RC4 { s: vec![0; 256], i: 0 , j :0};

        for k in 0..256 {
            rc4.s[k] = k as u32;
        }

        let mut l:u32 = 0;
        for k in 0..=255 {

            l = (given_key[(k % key_size) as usize] + rc4.s[k as usize] + l)%256;
            let saved = rc4.s[ k as usize];

            rc4.s[ k as usize] = rc4.s[ l as usize];

            rc4.s[ l as usize] = saved;
        }
        return rc4
    }

    #[wasm_bindgen]
    pub fn encrypt(&mut self, data: Vec<u32>, data_size: u32) -> Vec<u32> {

        let mut new_data:Vec<u32> = data;
        for k in 0..data_size {
            self.i = (self.i + 1)%256;
            self.j = (self.j + self.s[self.i as usize])%256;

                
            // swap
           // mem::swap(& mut self.sbox[i as usize], & mut self.sbox[j as usize]);

            let saved = self.s[ self.i as usize];

            self.s[ self.i as usize] = self.s[ self.j as usize];

            self.s[ self.j as usize] = saved;
            println!("k {}",k);
            //println!("i {} , j {}",i,j);
            //println!("i+j {} ",self.s[(self.s[i as usize] + self.s[j as usize]) as usize %256]);
            println!("{}",self.s[(self.s[self.i as usize] + self.s[self.j as usize]) as usize %256]);
            println!("XOR");
            println!("msg {:?}",new_data);
            println!("{}",new_data[k as usize] ^ self.s[(self.s[self.i as usize] + self.s[self.j as usize]) as usize %256]);

            new_data[k as usize] = new_data[k as usize] ^ self.s[(self.s[self.i as usize] + self.s[self.j as usize]) as usize %256];

        }

        return new_data;
    }
    
    #[wasm_bindgen]
    pub fn decrypt(&mut self, data: Vec<u32>, data_size: u32) -> Vec<u32> {
        return self.encrypt(data, data_size);
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rc4_create_key() {
        let key: [u32; 16] = [
            23, 189,   8, 107, 27, 148,
           240,  47, 240, 236, 83, 215,
            99,  88, 155,  95
         ];
        let rc4_obj = RC4::initialize(key.to_vec(), key.len() as u8);
        assert_eq!(
            rc4_obj.s,
            [23,213,73,77,20,5,162,252,41,184,123,134,148,219,211,96,111,61,87,43,18,76,179,120,232,239,46,79,143,66,230,100,80,62,146,13,112,58,60,149,19,190,9,249,12,48,255,182,25,54,30,103,246,191,229,75,129,136,144,63,70,107,215,166,231,6,155,206,81,251,248,101,28,74,159,40,127,183,31,78,130,4,164,132,243,220,177,45,24,124,114,221,122,225,247,57,72,3,242,108,109,178,90,133,67,224,150,188,16,0,235,119,71,22,55,222,140,39,65,99,53,44,10,153,141,92,173,84,189,217,156,218,201,117,93,193,145,181,195,192,98,11,165,154,33,152,32,34,203,228,104,56,126,194,175,38,142,110,214,223,254,202,69,205,198,170,86,244,118,1,29,212,227,157,185,147,27,196,89,105,253,174,168,68,208,102,187,26,216,138,180,226,137,94,161,200,176,113,139,245,91,240,95,204,8,47,241,209,158,52,50,131,128,21,238,51,2,88,35,116,42,250,64,15,85,83,163,14,106,207,59,37,234,237,171,167,236,97,160,233,82,36,151,186,121,172,115,199,197,17,135,169,49,7,210,125]
        )
    }
    #[test]
    fn rc4_enc() {
        let key: [u32; 16] = [
            23, 189,   8, 107, 27, 148,
           240,  47, 240, 236, 83, 215,
            99,  88, 155,  95
         ];
        let data: [u32; 34] = [5,1,0,0,0,0,0,0,0,21,0,0,0,2,1,0,0,0,3,0,0,0,1,0,0,0,4,0,0,0,116,101,115,116];
        let mut rc4_obj = RC4::initialize(key.to_vec(), key.len() as u8);
        let data_result = rc4_obj.encrypt(data.to_vec(), 34);
        assert_eq!(
            data_result,
            [
                174, 183, 184,  67, 241,  64, 165,   4,
                140,   6,  35,  87, 102, 206, 169,  26,
                 83,  24, 215, 183,  41, 116, 190, 249,
                143, 205,  14, 169, 236, 237, 190, 129,
                 81, 214
              ]
        )
    }
}
