use crate::{
    utils::{decode_chars_to_string, extract_item_entries},
    Entry, RLPDecodable, RLPDecodingError,
};

impl RLPDecodable for u8 {
    fn decode(input: Vec<Entry>) -> Result<Self, RLPDecodingError> {
        let length = &input.len();

        match length {
            1 => {
                // Single byte in range [0x00, 0x7f]
                let data = &input[0];
                match data {
                    Entry::Char(item) => Ok(*item as u8),
                    _ => Err(RLPDecodingError::InvalidData),
                }
            }

            2 => {
                // Single byte > 0x7f
                let data = &input[1];
                match data {
                    Entry::Integer(d) => Ok(*d),
                    _ => Err(RLPDecodingError::InvalidData),
                }
            }
            _ => Err(RLPDecodingError::InvalidData),
        }
    }
}

impl RLPDecodable for char {
    fn decode(input: Vec<Entry>) -> Result<Self, RLPDecodingError> {
        for entry in input.into_iter() {
            match entry {
                // return character as it is
                Entry::Char(character) => return Ok(character),
                // we dont care about other types, even if an integer entry is
                // present to specify length, it can be ignored
                _ => continue,
            }
        }
        Err(RLPDecodingError::InvalidData)
    }
}

impl RLPDecodable for &str {
    fn decode(input: Vec<Entry>) -> Result<Self, RLPDecodingError> {
        // pass it onto `impl` of `String`
        String::decode(input).map(|s| {
            let leaked: &'static mut String = Box::leak(Box::new(s));
            leaked.as_str()
        })
    }
}

impl RLPDecodable for String {
    fn decode(input: Vec<Entry>) -> Result<Self, RLPDecodingError> {
        let prefix = &input[0];

        let mut res = String::new();

        // emptry string
        if prefix == &Entry::Integer(0x80) {
            Ok(res)
        }
        // string is less than or equal to 55 bytes long
        else if prefix > &Entry::Integer(0x80) && prefix <= &Entry::Integer(0xb7) {
            decode_chars_to_string(&input, 1, &mut res)
        }
        // string is less than or equal to 55 bytes long
        else if prefix > &Entry::Integer(0xb7) {
            let hops = match prefix {
                Entry::Integer(d) => (*d - 0xb7) as usize,
                _ => 0,
            };
            decode_chars_to_string(&input, hops, &mut res)
        } else {
            Err(RLPDecodingError::InvalidData)
        }
    }
}

impl<T> RLPDecodable for Vec<T>
where
    T: RLPDecodable,
{
    fn decode(input: Vec<Entry>) -> Result<Self, RLPDecodingError> {
        if input.is_empty() {
            return Err(RLPDecodingError::InvalidData);
        }

        let prefix = &input[0];

        match prefix {
            Entry::Integer(prefix_value) => {
                if *prefix_value == 0xc0 {
                    // Empty list
                    return Ok(Vec::new());
                }

                let _ = if *prefix_value <= 0xf7 {
                    // Short list -> prefix directly indicates the total length of the list
                    (*prefix_value - 0xc0) as usize
                } else {
                    // Long list -> prefix indicates the length of the length
                    // followed by the length itself
                    let length_of_length = (*prefix_value - 0xf7) as usize;
                    if input.len() < length_of_length + 1 {
                        return Err(RLPDecodingError::InvalidData);
                    }

                    let mut length = 0;
                    for i in 0..length_of_length {
                        if let Entry::Integer(byte) = input[i + 1] {
                            length = (length << 8) + byte as usize;
                        } else {
                            return Err(RLPDecodingError::InvalidData);
                        }
                    }

                    length
                };

                let mut result = Vec::new();
                let mut current_index = if *prefix_value <= 0xf7 {
                    1
                } else {
                    1 + (prefix_value - 0xf7) as usize
                };

                while current_index < input.len() {
                    let item_entries = extract_item_entries(&input, current_index)?;
                    let decoded_item = T::decode(item_entries.clone())?;
                    result.push(decoded_item);
                    current_index += item_entries.len();
                }

                Ok(result)
            }
            _ => Err(RLPDecodingError::InvalidData),
        }
    }
}
