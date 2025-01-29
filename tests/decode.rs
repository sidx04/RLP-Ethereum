use rlp::{Entry, RLP};

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_single_byte() {
        // Single byte in range [0x00, 0x7f]
        let rlp = RLP::new(0x7fu8, None);
        let encoded: Vec<Entry> = rlp.encode();
        assert_eq!(RLP::decode(encoded), Ok('\u{7f}'));

        // Single byte > 0x7f
        let rlp = RLP::new(0x84u8, None);
        let encoded: Vec<Entry> = rlp.encode();
        assert_eq!(RLP::decode(encoded), Ok('\u{84}'));
    }

    #[test]
    fn test_character() {
        let rlp = RLP::new('a', None);
        let encoded = rlp.encode();
        assert_eq!(RLP::decode(encoded), Ok('a'));
    }

    #[test]
    fn test_string() {
        // empty string
        let rlp = RLP::new("".to_owned(), None);
        let encoded = rlp.encode();
        assert_eq!(RLP::decode(encoded), Ok(""));

        // string less than 55 bytes in length
        let rlp = RLP::new("cat".to_owned(), None);
        let encoded = rlp.encode();
        assert_eq!(RLP::decode(encoded), Ok("cat"));
    }

    #[test]
    fn test_long_string() {
        // 1024 bytes string
        let rlp = RLP::new(
            "Lorem ipsum dolor sit amet, consectetur adipisicing elit",
            None,
        );
        let encoded = rlp.encode();
        assert_eq!(
            RLP::decode(encoded),
            Ok("Lorem ipsum dolor sit amet, consectetur adipisicing elit")
        );

        let rlp = RLP::new(
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", None,
        );
        let encoded = rlp.encode();
        assert_eq!(
            RLP::decode(encoded),
            Ok("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa")
        );

        let rlp = RLP::new(
            "012345678910111213141516171819202122232425262728293031323334353637383940",
            None,
        );
        let encoded = rlp.encode();
        assert_eq!(
            RLP::decode(encoded),
            Ok("012345678910111213141516171819202122232425262728293031323334353637383940")
        );
    }

    #[test]
    fn test_vec() {
        let rlp = RLP::new(vec!["cat", "dog"], None);
        let encoded = rlp.encode();
        assert_eq!(RLP::decode(encoded), Ok(vec!["cat", "dog"]));

        // let rlp = RLP::new(vec![1, 2], None);
        // let encoded = rlp.encode();
        // assert_eq!(RLP::decode(encoded), Ok(vec![1, 2]));

        // let rlp = RLP::new(vec![vec![1, 2], vec![3, 4]], None);
        // let encoded = rlp.encode();
        // assert_eq!(RLP::decode(encoded), Ok(vec![vec![1, 2], vec![3, 4]]));

        // let rlp: RLP<Vec<Vec<Vec<Vec<&str>>>>> =
        //     RLP::new(vec![vec![], vec![vec![]], vec![vec![], vec![vec![]]]], None);
        // let encoded = rlp.encode();
        // let output: Vec<Vec<Vec<Vec<&str>>>> =
        //     vec![vec![], vec![vec![]], vec![vec![], vec![vec![]]]];
        // assert_eq!(RLP::decode(encoded), Ok(output));
    }
}
