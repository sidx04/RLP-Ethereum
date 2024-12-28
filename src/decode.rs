use crate::{Entry, RLPDecodable, RLPDecodingError};

impl RLPDecodable for u8 {
    fn decode(input: Vec<Entry>) -> Result<Self, RLPDecodingError> {
        let length = input.len();

        match length {
            1 => {
                // Single byte in range [0x00, 0x7f]
                let data = input[0];
                match data {
                    Entry::Integer(d) => Ok(d),
                    _ => Err(RLPDecodingError::InvalidData),
                }
            }

            2 => {
                // Single byte > 0x7f
                let data = input[1];
                match data {
                    Entry::Integer(d) => Ok(d),
                    _ => Err(RLPDecodingError::InvalidData),
                }
            }
            _ => Err(RLPDecodingError::InvalidData),
        }
    }
}

impl RLPDecodable for char {
    fn decode(input: Vec<Entry>) -> Result<Self, RLPDecodingError> {
        let first = input[0];

        match first {
            Entry::Integer(_) => {
                let second = input[1];
                match second {
                    Entry::Char(item) => Ok(item),
                    _ => Err(RLPDecodingError::InvalidData),
                }
            }
            _ => Err(RLPDecodingError::InvalidData),
        }
    }
}

impl RLPDecodable for &str {
    fn decode(input: Vec<Entry>) -> Result<Self, RLPDecodingError> {
        todo!()
    }
}

impl RLPDecodable for String {
    fn decode(input: Vec<Entry>) -> Result<Self, RLPDecodingError> {
        todo!()
    }
}

impl RLPDecodable for Vec<&str> {
    fn decode(input: Vec<Entry>) -> Result<Self, RLPDecodingError> {
        todo!()
    }
}

impl RLPDecodable for Vec<u8> {
    fn decode(input: Vec<Entry>) -> Result<Self, RLPDecodingError> {
        todo!()
    }
}

impl RLPDecodable for Vec<Vec<Vec<Vec<&str>>>> {
    fn decode(input: Vec<Entry>) -> Result<Self, RLPDecodingError> {
        todo!()
    }
}
