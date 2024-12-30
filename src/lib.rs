pub mod decode;
pub mod encode;
pub mod errors;
pub mod utils;

use std::fmt::Debug;

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub struct RLP<T: Debug + RLPEncodable> {
    data: T,
    encoded: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Entry {
    Integer(u8),
    Char(char),
    List(Vec<Entry>),
}

#[derive(Debug, PartialEq, Eq)]
pub enum RLPDecodingError {
    InvalidData,
    UnexpectedEOF,
    UnsupportedType,
}

pub trait RLPEncodable {
    fn encode(&self) -> Vec<Entry>;
}

pub trait RLPDecodable: Sized {
    fn decode(input: Vec<Entry>) -> Result<Self, RLPDecodingError>;
}

impl<T: Debug + RLPEncodable + RLPDecodable> RLP<T> {
    /// Creates a new RLP instance.
    ///
    /// ## Arguments
    ///
    /// * `data`: The underlying data to be stored in the RLP instance.
    /// * `encoded`: An optional boolean indicating whether the data is already encoded.
    ///
    /// ## Returns
    ///
    /// A new `RLP` instance.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use rlp::RLP;
    ///
    /// let rlp1 = RLP::new(vec![1, 2, 3], None); // Unencoded data
    /// let rlp2 = RLP::new(vec![4, 5, 6], Some(true)); // Pre-encoded data
    ///
    /// ```
    pub fn new(data: T, encoded: Option<bool>) -> Self {
        RLP {
            data,
            encoded: encoded.unwrap_or(false),
        }
    }

    /// Encodes the underlying data of the RLP instance into an RLP-encoded vector of entries.
    ///
    /// This function will only perform encoding if the data has not already been encoded.
    ///
    /// ## Returns
    ///
    /// A vector of `Entry` representing the RLP-encoded data. If the data was already encoded,
    /// an empty vector is returned.
    ///
    /// ```rust
    /// use rlp::{RLP, Entry};
    ///
    /// let rlp = RLP::new("cat", None);
    /// let encoded = rlp.encode();
    /// assert_eq!(encoded[0], Entry::Integer(131));
    /// assert_eq!(encoded[1], Entry::Char('c'));
    /// assert_eq!(encoded[2], Entry::Char('a'));
    /// assert_eq!(encoded[3], Entry::Char('t'));
    ///
    /// let rlp = RLP::new(0x7fu8, None);
    /// assert_eq!(rlp.encode(), vec![Entry::Integer(0x7f)]);
    /// ```
    pub fn encode(&self) -> Vec<Entry> {
        if self.encoded == Option::is_some(&Some(true)) {
            return Vec::new();
        }
        self.data.encode()
    }

    pub fn decode(input: Vec<Entry>) -> Result<T, RLPDecodingError> {
        let data: T = T::decode(input)?;
        Ok(data)
    }
}
