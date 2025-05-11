use std::borrow::Borrow;

use bytes::BufMut;

use crate::{EMPTY_STRING_CODE, Encodable, header::Header};

macro_rules! impl_encodable_for_uint {
  ($($t:ty),*) => {
      $(
          impl Encodable for $t {
              #[inline]
              fn length(&self) -> usize {
                  let x = *self;
                  if x < EMPTY_STRING_CODE as $t {
                      1
                  } else {
                      1 + (<$t>::BITS as usize / 8) - (x.leading_zeros() as usize / 8)
                  }
              }

              #[inline]
              fn encode(&self, out: &mut dyn BufMut) {
                  let x = *self;
                  if x == 0 {
                      out.put_u8(EMPTY_STRING_CODE);
                  } else if x < EMPTY_STRING_CODE as $t {
                      out.put_u8(x as u8);
                  } else {
                      let be_bytes;
                      be_bytes = x.to_be_bytes();
                      out.put_u8(EMPTY_STRING_CODE + be_bytes.len() as u8);
                      out.put_slice(&be_bytes);
                  }
              }

          }
      )*
  };
}

impl_encodable_for_uint!(u8, u16, u32, u64, u128, usize);

impl Encodable for [u8] {
    #[inline]
    fn length(&self) -> usize {
        let mut len = self.len();
        if len != 1 || self[0] >= EMPTY_STRING_CODE {
            len += length_of_length(len)
        }
        len
    }

    #[inline]
    fn encode(&self, out: &mut dyn BufMut) {
        if self.len() != 1 || self[0] >= EMPTY_STRING_CODE {
            Header {
                is_list: false,
                payload_len: self.len(),
            }
            .encode(out);
            out.put_slice(self);
        }
    }
}

impl<const N: usize> Encodable for [u8; N] {
    #[inline]
    fn length(&self) -> usize {
        self[..].len()
    }
    #[inline]
    fn encode(&self, out: &mut dyn BufMut) {
        self[..].encode(out);
    }
}

impl Encodable for &str {
    #[inline]
    fn length(&self) -> usize {
        self.as_bytes().length()
    }

    #[inline]
    fn encode(&self, out: &mut dyn BufMut) {
        self.as_bytes().encode(out)
    }
}

impl Encodable for bool {
    #[inline]
    fn length(&self) -> usize {
        1
    }

    #[inline]
    fn encode(&self, out: &mut dyn BufMut) {
        out.put_u8(if *self { 1 } else { EMPTY_STRING_CODE });
    }
}

impl<T> Encodable for Vec<T>
where
    T: Encodable,
{
    #[inline]
    fn length(&self) -> usize {
        get_list_length(self)
    }
    #[inline]
    fn encode(&self, out: &mut dyn BufMut) {
        encode_list(self, out);
    }
}

pub const fn length_of_length(payload_length: usize) -> usize {
    if payload_length < 56 {
        1
    } else {
        1 + (usize::BITS as usize / 8) - payload_length.leading_zeros() as usize / 8
    }
}

/// Calculate the length of a list.
#[inline]
pub fn get_list_length<B, T>(list: &[B]) -> usize
where
    B: Borrow<T>,
    T: ?Sized + Encodable,
{
    let payload_len = rlp_list_header(list).payload_len;
    payload_len + length_of_length(payload_len)
}

#[inline]
fn rlp_list_header<B, T>(values: &[B]) -> Header
where
    B: Borrow<T>,
    T: ?Sized + Encodable,
{
    let mut header = Header {
        is_list: true,
        payload_len: 0,
    };
    for v in values {
        header.payload_len += v.borrow().length();
    }
    header
}

/// Encode a list
pub fn encode_list<B, T>(values: &[B], out: &mut dyn BufMut)
where
    B: Borrow<T>,
    T: ?Sized + Encodable,
{
    rlp_list_header(values).encode(out);
    for v in values {
        v.borrow().encode(out);
    }
}
