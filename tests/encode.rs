#[cfg(test)]
mod tests {

    use std::vec;

    use bytes::BytesMut;
    use rlp_rs::{EMPTY_STRING_CODE, Encodable};

    fn encode_to_vec<T: Encodable>(val: T) -> Vec<u8> {
        let mut buf = BytesMut::new();
        val.encode(&mut buf);
        buf.to_vec()
    }

    #[test]
    fn encode_small_integer() {
        let val: u8 = 0x5f;
        let encoded = encode_to_vec(val);
        assert_eq!(encoded, vec![0x5f])
    }

    #[test]
    fn encode_large_integer() {
        let val: u16 = 0x0400;
        let encoded = encode_to_vec(val);
        assert_eq!(encoded, vec![EMPTY_STRING_CODE + 2, 0x04, 0x00]);
    }

    #[test]
    fn encode_zero_byte() {
        let val: usize = 0;
        let encoded = encode_to_vec(val);
        assert_eq!(encoded, vec![EMPTY_STRING_CODE]);
    }

    #[test]
    fn encode_string() {
        let val: &str = "dog";
        let encoded = encode_to_vec(val);
        assert_eq!(encoded, vec![0x83, 0x64, 0x6F, 0x67]);
    }

    #[test]
    fn encode_empty() {
        let val = "";
        let encoded = encode_to_vec(val);
        assert_eq!(encoded, vec![0x80]);

        let val: usize = 0x00;
        let encoded = encode_to_vec(val);
        assert_eq!(encoded, vec![0x80])
    }

    #[test]
    fn encode_list() {
        let val: Vec<&str> = vec!["cat", "dog"];
        let encoded = encode_to_vec(val);
        assert_eq!(
            encoded,
            vec![
                0xc8, 0x83, 'c' as u8, 'a' as u8, 't' as u8, 0x83, 'd' as u8, 'o' as u8, 'g' as u8
            ]
        );
    }

    #[test]
    fn encode_nested_vecs() {
        let val: Vec<Vec<&str>> = vec![vec!["cat", "dog"], vec!["red", "blue"]];
        let encoded = encode_to_vec(val);
        eprintln!("{:02x?}", encoded);
    }
}
