use crate::{Entry, RLPDecodingError};

pub fn to_binary_bytes(num: u64) -> Vec<u8> {
    let mut bytes = vec![];
    let mut value = num;
    while value > 0 {
        bytes.push((value & 0xff) as u8); // Extract the lowest 8 bits
        value >>= 8; // Shift value to the right by 8 bits
    }
    bytes.reverse(); // Reverse to get big-endian order
    bytes
}

// for controls command characters (https://en.wikipedia.org/wiki/List_of_Unicode_characters#Control_codes)
pub fn check_character(num: &u8) -> bool {
    let num = *num;
    (num <= 31) || (num >= 127 && num < 160)
}

pub fn decode_chars_to_string(
    input: &Vec<Entry>,
    hops: usize,
    res: &mut String,
) -> Result<String, RLPDecodingError> {
    for entry in input[hops..input.len()].into_iter() {
        match entry {
            Entry::Char(item) => res.push(*item),
            _ => continue,
        }
    }
    Ok(res.to_owned())
}
