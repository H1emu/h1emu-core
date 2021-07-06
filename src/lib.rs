use uuid::Uuid;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn generate_random_guid() -> String {
    let my_uuid: Uuid = Uuid::new_v4();
    let array_bytes = my_uuid.as_bytes();
    let mut rand_id: String = String::new();
    rand_id.push_str("0x");
    for byte in array_bytes {
        let formatted_byte: String = format!("{:X}", byte);
        if formatted_byte.len() < 2 {
            let mut formatted_byte_with_additionnal_zero: String = "0".to_string();
            formatted_byte_with_additionnal_zero.push_str(&formatted_byte);
            rand_id.push_str(&formatted_byte_with_additionnal_zero);
        } else {
            rand_id.push_str(&formatted_byte);
        }
        if rand_id.len() == 16 {
            break;
        }
    }
    return rand_id;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn generate_random_guid_test() {
        assert_ne!(generate_random_guid(), generate_random_guid())
    }
}
