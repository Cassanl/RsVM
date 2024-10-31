use std::num::ParseIntError;

/// parse 32bits hex chunks
/// hexes format is "00 00 00 00"
pub fn hex_to_byte_arr(hexes: &str) -> Result<[u8; 4], ParseIntError> {
    let mut byte_array: [u8; 4] = [0; 4];
    let hexes: Vec<&str> = hexes.split(" ").collect();
    let mut index = 0;
    for hex in &hexes {
        match u8::from_str_radix(hex, 16) {
            Ok(value) => {
                byte_array[index] = value;
                index += 1;
            }
            Err(err) => return Err(err),
        }
    }
    Ok(byte_array)
}
