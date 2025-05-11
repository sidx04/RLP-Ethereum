use bytes::BufMut;
pub mod encode;
pub mod header;
pub mod utils;

pub const EMPTY_STRING_CODE: u8 = 0x80;
pub const EMPTY_LIST_CODE: u8 = 0xc0;

pub trait Encodable {
    /// Encodes the given type into the `out` buffer.
    fn encode(&self, out: &mut dyn BufMut);

    /// Returns the length of the encoding of this type in bytes.
    ///
    /// The default implementation computes this by encoding the type.
    /// When possible, we recommender implementers override this with a
    /// specialized implementation.
    #[inline]
    fn length(&self) -> usize {
        let mut out = Vec::new();
        self.encode(&mut out);
        out.len()
    }
}
