use crate::{utils::decode_chars_to_string, Entry, RLPDecodable, RLPDecodingError};

impl RLPDecodable for u8 {
    fn decode(input: Vec<Entry>) -> Result<Self, RLPDecodingError> {
        let length = &input.len();

        match length {
            1 => {
                // Single byte in range [0x00, 0x7f]
                let data = &input[0];
                match data {
                    Entry::Integer(d) => Ok(*d),
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
        let prefix = &input[0];

        match prefix {
            Entry::Integer(_) => {
                let entry = &input[1];
                match entry {
                    Entry::Char(item) => Ok(*item),
                    _ => Err(RLPDecodingError::InvalidData),
                }
            }
            _ => Err(RLPDecodingError::InvalidData),
        }
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

        println!("{:?}", input);

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
        todo!()
    }
}
