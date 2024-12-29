use rlp::{Entry, RLP};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_byte() {
        // Single byte in range [0x00, 0x7f]
        let rlp = RLP::new(0x7fu8, None);
        assert_eq!(rlp.encode(), vec![Entry::Integer(0x7f)]);

        // Single byte > 0x7f
        let rlp = RLP::new(0x80u8, None);
        assert_eq!(
            rlp.encode(),
            vec![Entry::Integer(0x81), Entry::Integer(0x80)]
        );
    }

    #[test]
    fn test_bytes() {
        // Bytes format
        let rlp = RLP::new("\x04\x00", None);

        // the bytes '\x04\x00' = [ 0x82, 0x04, 0x00 ]
        assert_eq!(
            rlp.encode(),
            vec![
                Entry::Integer(0x82),
                Entry::Integer(0x04),
                Entry::Integer(0x00)
            ]
        );
    }

    #[test]
    fn test_character() {
        // ASCII character
        let rlp = RLP::new('a', None);
        assert_eq!(rlp.encode(), vec![Entry::Integer(129), Entry::Char('a')]);

        // Unicode character
        let rlp = RLP::new('λ', None);
        assert_eq!(rlp.encode(), vec![Entry::Integer(130), Entry::Char('λ')]);
    }

    #[test]
    fn test_string() {
        // Empty string
        let rlp = RLP::new("", None);
        assert_eq!(rlp.encode(), vec![Entry::Integer(128)]);

        // Short string
        let rlp = RLP::new("cat", None);
        let encoded = rlp.encode();
        assert_eq!(encoded[0], Entry::Integer(131)); // 0x83 (string of length 3)
        assert_eq!(encoded[1], Entry::Char('c'));
        assert_eq!(encoded[2], Entry::Char('a'));
        assert_eq!(encoded[3], Entry::Char('t'));

        // Long string (>55 bytes)
        let long_string = "Lorem ipsum dolor sit amet, consectetur adipiscing elit.";
        let rlp = RLP::new(long_string, None);
        let encoded = rlp.encode();
        assert_eq!(encoded[0], Entry::Integer(184)); // 0xb8 (string length > 55)
        assert_eq!(encoded[1], Entry::Integer(56))
    }

    #[test]
    fn test_long_string_1024_bytes() {
        let rlp = RLP::new(
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
", None,
        );
        let encoded = rlp.encode();
        assert_eq!(encoded[0], Entry::Integer(185));
    }

    #[test]
    fn test_string_vec() {
        // Empty string vector
        let rlp = RLP::new(vec![""], None);
        let encoded = rlp.encode();
        assert_eq!(encoded[0], Entry::Integer(193)); // 0xc1 for list of length 1
        assert_eq!(encoded[1], Entry::Integer(128)); // 0x80 for empty string

        // Vector of strings
        let rlp = RLP::new(vec!["cat", "dog"], None);
        let encoded = rlp.encode();

        // 0xc8 is the first byte for list of length 8
        assert_eq!(encoded[0], Entry::Integer(200));

        // First string: "cat"
        assert_eq!(encoded[1], Entry::Integer(131));
        assert_eq!(encoded[2], Entry::Char('c'));
        assert_eq!(encoded[3], Entry::Char('a'));
        assert_eq!(encoded[4], Entry::Char('t'));

        // Second string: "dog"
        assert_eq!(encoded[5], Entry::Integer(131));
        assert_eq!(encoded[6], Entry::Char('d'));
        assert_eq!(encoded[7], Entry::Char('o'));
        assert_eq!(encoded[8], Entry::Char('g'));
    }

    #[test]
    fn test_long_string_vec() {
        let rlp = RLP::new(vec![
    "This is a very long string that exceeds",
    "Lorem ipsum dolor sit amet, consectetur",
    "A long string of random characters: qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM1234567890",
    "A sentence with a lot of words to make it longer",
    "A very long sentence that is much longer than 55 characters",
    "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890",
    "A string with some special characters: !@#$%^&*()_+-=[]{}|;':\",./<>?",
    "A string with numbers and letters: 1234567890abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ",
    "A very long string that is longer than 55 characters and contains many words.",
    "A string with a lot of spaces to make it longer",
    "A very long string that is longer than 55 characters and contains many letters.",
    "A string with a lot of numbers to make it longer",
    "A very long string that is longer than 55 characters and contains many symbols.",
    "A string with a lot of punctuation to make it longer",
    "A very long string that is longer than 55 characters and contains many spaces.",
    "A string with a lot of newlines to make it longer",
    "A very long string that is longer than 55 characters and contains many tabs.",
    "A string with a lot of carriage returns to make it longer",
    "A very long string that is longer than 55 characters and contains many control characters.",
    "A string with a lot of binary data to make it longer",
    "A very long string that is longer than 55 characters and contains many hexadecimal characters.",
    "A string with a lot of octal characters to make it longer",
    "A very long string that is longer than 55 characters and contains many base64 characters.",
    "A string with a lot of ASCII characters to make it longer",
    "A very long string that is longer than 55 characters and contains many Unicode characters.",
    "A string with a lot of emoji characters to make it longer",
    "A very long string that is longer than 55 characters and contains many mathematical symbols.",
    "A string with a lot of Greek letters to make it longer",
    "A very long string that is longer than 55 characters and contains many Cyrillic characters.",
    "A string with a lot of Japanese characters to make it longer",
    "A very long string that is longer than 55 characters and contains many Chinese characters.",
    "A string with a lot of Korean characters to make it longer",
    "A very long string that is longer than 55 characters and contains many Arabic characters.",
    "A string with a lot of Hebrew characters to make it longer",
    "A very long string that is longer than 55 characters and contains many Devanagari characters.",
    "A string with a lot of Bengali characters to make it longer",
    "A very long string that is longer than 55 characters and contains many Tamil characters.",
    "A string with a lot of Telugu characters to make it longer",
    "A very long string that is longer than 55 characters and contains many Kannada characters.",
    "A string with a lot of Malayalam characters to make it longer",
    "A very long string that is longer than 55 characters and contains many Gujarati characters.",
    "A string with a lot of Punjabi characters to make it longer",
    "A very long string that is longer than 55 characters and contains many Oriya characters.",
    "A string with a lot of Assamese characters to make it longer",
    "A very long string that is longer than 55 characters and contains many Maithili characters.",
    "A string with a lot of Dogri characters to make it longer",
    "A very long string that is longer than 55 characters and contains many Kashmiri characters.",
    "A string with a lot of Sindhi characters to make it longer",
    "A very long string that is longer than 55 characters and contains many Sanskrit characters.",
    "A string with a lot of Urdu characters to make it longer",
    "A very long string that is longer than 55 characters and contains many Persian characters.",
    "A string with a lot of Arabic characters to make it longer",
    "A very long string that is longer than 55 characters and contains many Pashto characters.",
    "A string with a lot of Balochi characters to make it longer",
    "A very long string that is longer than 55 characters and contains many Sindhi characters.",
    "A string with a lot of Brahui characters to make it longer",
    "A very long string that is longer than 55 characters and contains many Kurdish characters.",
    "A string with a lot of Azeri characters to make it longer",
    "A very long string that is longer than 55 characters and contains many Turkish characters.",
    "A string with a lot of Armenian characters to make it longer",
    "A very long string that is longer than 55 characters and contains many Georgian characters.",
], None);
        let encoded = rlp.encode();
        assert_eq!(encoded[0], Entry::Integer(249));
    }

    #[test]
    fn test_empty_vector() {
        let data: Vec<&str> = vec![];
        let rlp = RLP::new(data, None);
        let encoded = rlp.encode();
        assert_eq!(encoded, vec![Entry::Integer(0xc0)]);
    }

    #[test]
    fn test_nested_vecs() {
        let data: Vec<Vec<Vec<Vec<&str>>>> = vec![vec![], vec![vec![]], vec![vec![], vec![vec![]]]];
        let rlp = RLP::new(data, None);
        assert_eq!(
            rlp.encode(),
            vec![
                Entry::Integer(0xc7),
                Entry::Integer(0xc0),
                Entry::Integer(0xc1),
                Entry::Integer(0xc0),
                Entry::Integer(0xc3),
                Entry::Integer(0xc0),
                Entry::Integer(0xc1),
                Entry::Integer(0xc0)
            ]
        );

        let data: Vec<Vec<&str>> = vec![vec![]];
        let rlp = RLP::new(data, None);
        assert_eq!(
            rlp.encode(),
            vec![Entry::Integer(0xc1), Entry::Integer(0xc0)]
        );

        let data: Vec<Vec<char>> = vec![vec!['a']];
        let rlp = RLP::new(data, None);
        assert_eq!(
            rlp.encode(),
            vec![
                Entry::Integer(0xc3),
                Entry::Integer(0xc2),
                Entry::Integer(0x81),
                Entry::Char('a')
            ]
        );

        let data: Vec<Vec<u8>> = vec![vec![1, 2], vec![3, 4]];
        let rlp = RLP::new(data, None);
        assert_eq!(
            rlp.encode(),
            vec![
                Entry::Integer(0xc6),
                Entry::Integer(0xc2),
                Entry::Integer(0x01),
                Entry::Integer(0x02),
                Entry::Integer(0xc2),
                Entry::Integer(0x03),
                Entry::Integer(0x04),
            ]
        );
    }
}
