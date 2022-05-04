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

#[wasm_bindgen]
pub fn eul2quat(angle: Vec<f32>) -> Vec<f32> {
    // Assuming the angles are in radians.
    let heading = angle[0];
    let attitude = angle[1];
    let bank = -angle[2];
    let c1 = f32::cos(heading / 2.0);
    let s1 = f32::sin(heading / 2.0);
    let c2 = f32::cos(attitude / 2.0);
    let s2 = f32::sin(attitude / 2.0);
    let c3 = f32::cos(bank / 2.0);
    let s3 = f32::sin(bank / 2.0);
    let c1c2 = c1 * c2;
    let s1s2 = s1 * s2;
    let qw = c1c2 * c3 - s1s2 * s3;
    let qy = s1 * c2 * c3 + c1 * s2 * s3;
    let qz = c1c2 * s3 + s1s2 * c3;
    let qx = c1 * s2 * c3 - s1 * c2 * s3;
    return [qx, qy, -qz, qw].to_vec();
  }


#[wasm_bindgen]
pub fn is_pos_in_radius(
    radius: f32,
    player_pos: Vec<f32>,
    enemi_pos: Vec<f32>,
) -> bool {
    let player_x = player_pos[0];
    let player_z = player_pos[2];
    let enemi_x = enemi_pos[0];
    let enemi_z = enemi_pos[2];
    
    return  (player_x - radius <= enemi_x && enemi_x <= player_x + radius) && (player_z - radius <= enemi_z && enemi_z <= player_z + radius)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn generate_random_guid_test() {
        assert_ne!(generate_random_guid(), generate_random_guid())
    }

    #[test]
    fn eul2quat_test() {
        assert_eq!(eul2quat([1.0,2.0,3.0].to_vec()), [0.31062245, -0.71828705, 0.44443506, 0.43595284].to_vec())
    }

    #[test]
    fn is_pos_in_radius_test() {
        assert_eq!(is_pos_in_radius(20.0,[0.0,1.0,2.0,0.0].to_vec(),[-19.0,1.0,20.0,0.0].to_vec()), true)
    }
}
