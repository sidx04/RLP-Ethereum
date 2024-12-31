use crate::utils::{check_character, to_binary_bytes};
use crate::{Entry, RLPEncodable};

// Single byte 0x00..0x7f -> if < 128 then encoding is itself, otherwise [0x80 + length, data]
impl RLPEncodable for u8 {
    fn encode(&self) -> Vec<Entry> {
        if *self < 128 {
            vec![Entry::Char(char::from(*self))]
        } else {
            vec![Entry::Integer(129), Entry::Char(char::from(*self))]
        }
    }
}

// Char (encoded as its UTF-8 representation)
// Essentially Char = 1 byte string
// char -> String -> &str
impl RLPEncodable for char {
    fn encode(&self) -> Vec<Entry> {
        self.to_string().encode()
    }
}

// String slice
impl RLPEncodable for &str {
    fn encode(&self) -> Vec<Entry> {
        let mut encoded = Vec::new();

        // Empty string
        if self.is_empty() {
            encoded.push(Entry::Integer(0x80)); // encoded as 0x80 or 128
            return encoded;
        }

        let chars: Vec<char> = self.chars().collect();
        let len = self.len();

        // String longer than 55 bytes
        if len > 55 {
            let binary_length_vector = to_binary_bytes(len as u64);
            encoded.push(Entry::Integer(
                u8::try_from(binary_length_vector.len()).unwrap() + 183,
            ));
            for length_chunk in binary_length_vector {
                encoded.push(Entry::Integer(length_chunk));
            }
        } else {
            // String up to 55 bytes
            encoded.push(Entry::Integer(len as u8 + 128));
        }

        // Add string content
        for val in chars {
            if check_character(&(val as u8)) {
                encoded.push(Entry::Integer(u8::try_from(val).unwrap()));
                continue;
            }
            encoded.push(Entry::Char(val));
        }

        encoded
    }
}

// Owned String, goes to `&str` implementation
impl RLPEncodable for String {
    fn encode(&self) -> Vec<Entry> {
        self.as_str().encode()
    }
}

// Vector (list) of RLP encodable items
impl<T: RLPEncodable> RLPEncodable for Vec<T> {
    fn encode(&self) -> Vec<Entry> {
        // Handle empty vectors
        if self.is_empty() {
            return vec![Entry::Integer(0xc0)];
        }

        let mut encoded = Vec::new();
        let mut contents = Vec::new();
        let mut total_len = 0;

        // Encode each item in the vector
        for item in self {
            let mut entries = item.encode();
            total_len += entries.len(); // Update the total length
            contents.append(&mut entries); // Append encoded entries to contents
        }

        println!("Contents: {:?}", contents);

        // Determine prefix based on total length
        if total_len > 55 {
            // Long list: Add length prefix with length of length
            let length_bytes = to_binary_bytes(total_len as u64);
            encoded.push(Entry::Integer((length_bytes.len() as u8) + 0xf7));
            for b in length_bytes {
                encoded.push(Entry::Integer(b));
            }
        } else {
            // Short list: Add total length directly
            encoded.push(Entry::Integer((total_len as u8) + 0xc0));
        }

        // Append contents to the encoded list
        encoded.append(&mut contents);

        // Print encoded result for debugging (optional)
        println!("Encoded: {encoded:?}");

        encoded
    }
}
