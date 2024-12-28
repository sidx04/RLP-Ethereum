use rlp::{Entry, RLP};

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_single_byte() {
        // Single byte in range [0x00, 0x7f]
        let rlp = RLP::new(0x7fu8, None);
        let encoded: Vec<Entry> = rlp.encode();
        assert_eq!(RLP::decode(encoded), Ok(0x7fu8));

        // Single byte > 0x7f
        let rlp = RLP::new(0x84u8, None);
        let encoded: Vec<Entry> = rlp.encode();
        assert_eq!(RLP::decode(encoded), Ok(0x84u8));
    }

    #[test]
    fn test_character() {
        let rlp = RLP::new('a', None);
        let encoded = rlp.encode();

        assert_eq!(RLP::decode(encoded), Ok('a'));

        // assert_eq!(rlp.encode(), vec![Entry::Integer(129), Entry::Char('a')]);
    }
}
