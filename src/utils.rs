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

pub fn extract_item_entries(
    input: &[Entry],
    start_index: usize,
) -> Result<Vec<Entry>, RLPDecodingError> {
    if start_index >= input.len() {
        return Err(RLPDecodingError::InvalidData);
    }

    let prefix = &input[start_index];

    match prefix {
        Entry::Integer(prefix_value) => {
            if *prefix_value < 0x80 {
                // Single byte
                Ok(vec![Entry::Integer(*prefix_value)])
            } else if *prefix_value <= 0xb7 {
                // Short string
                let length = (*prefix_value - 0x80) as usize;
                if start_index + length >= input.len() {
                    return Err(RLPDecodingError::InvalidData);
                }
                println!(
                    "Item Entries (Short String) {:?}",
                    input[start_index..=start_index + length].to_vec()
                );
                Ok(input[start_index..=start_index + length].to_vec())
            } else if *prefix_value <= 0xbf {
                // Long string
                let length_of_length = (*prefix_value - 0xb7) as usize;
                if start_index + length_of_length >= input.len() {
                    return Err(RLPDecodingError::InvalidData);
                }

                let mut length = 0;
                for i in 0..length_of_length {
                    if let Entry::Integer(byte) = input[start_index + 1 + i] {
                        length = (length << 8) + byte as usize;
                    } else {
                        return Err(RLPDecodingError::InvalidData);
                    }
                }

                if start_index + length_of_length + length >= input.len() {
                    return Err(RLPDecodingError::InvalidData);
                }

                Ok(input[start_index..=start_index + length_of_length + length].to_vec())
            } else if *prefix_value <= 0xf7 {
                // Short list
                let length = (*prefix_value - 0xc0) as usize;
                if start_index + length >= input.len() {
                    return Err(RLPDecodingError::InvalidData);
                }
                Ok(input[start_index..=start_index + length].to_vec())
            } else {
                // Long list
                let length_of_length = (*prefix_value - 0xf7) as usize;
                if start_index + length_of_length >= input.len() {
                    return Err(RLPDecodingError::InvalidData);
                }

                let mut length = 0;
                for i in 0..length_of_length {
                    if let Entry::Integer(byte) = input[start_index + 1 + i] {
                        length = (length << 8) + byte as usize;
                    } else {
                        return Err(RLPDecodingError::InvalidData);
                    }
                }

                if start_index + length_of_length + length >= input.len() {
                    return Err(RLPDecodingError::InvalidData);
                }

                Ok(input[start_index..=start_index + length_of_length + length].to_vec())
            }
        }
        Entry::Char(_) => {
            // Single character
            Ok(vec![input[start_index].clone()])
        }
    }
}
