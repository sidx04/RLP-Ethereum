use bytes::BufMut;

use crate::{EMPTY_LIST_CODE, EMPTY_STRING_CODE, encode::length_of_length};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Header {
    pub is_list: bool,
    pub payload_len: usize,
}

impl Header {
    #[inline]
    pub fn encode(&self, out: &mut dyn BufMut) {
        if self.payload_len < 56 {
            let code = if self.is_list {
                EMPTY_LIST_CODE
            } else {
                EMPTY_STRING_CODE
            };
            out.put_u8(code + self.payload_len as u8);
        } else {
            let len_be = self.payload_len.to_be_bytes();
            let code = if self.is_list { 0xf7 } else { 0xb7 };
            out.put_u8(code + len_be.len() as u8);
            out.put_slice(&len_be);
        }
    }

    /// Returns the length of the encoded header.
    #[inline]
    pub const fn length(&self) -> usize {
        length_of_length(self.payload_len)
    }

    /// Returns the total length of the encoded header and payload.
    #[inline]
    pub const fn length_with_payload(&self) -> usize {
        self.length() + self.payload_len
    }
}
