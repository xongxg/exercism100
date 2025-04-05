use std::fmt::format;

pub fn encode(source: &str) -> String {
    let mut res = String::new();
    let mut chars = source.chars().peekable();
    let mut count = 1;

    while let Some(c) = chars.next() {
        if chars.peek() == Some(&c) {
            count += 1;
        } else {
            if count > 1 {
                res.push_str(&count.to_string());
            }
            res.push(c);
            count = 1;
        }
    }

    res
}

pub fn decode(source: &str) -> String {
    let mut res = String::new();
    let mut count = 0;

    for c in source.chars() {
        match c.to_digit(10) {
            Some(d) => {
                count = count * 10 + d;
            }
            None => {
                if count == 0 {
                    count = 1;
                }
                res.push_str(&c.to_string().repeat(count as usize));
                count = 0;
            }
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_empty_string() {
        let input = "";
        let output = encode(input);
        let expected = "";
        assert_eq!(output, expected);
    }
    #[test]
    #[ignore]
    fn encode_single_characters_only_are_encoded_without_count() {
        let input = "XYZ";
        let output = encode(input);
        let expected = "XYZ";
        assert_eq!(output, expected);
    }
    #[test]
    #[ignore]
    fn encode_string_with_no_single_characters() {
        let input = "AABBBCCCC";
        let output = encode(input);
        let expected = "2A3B4C";
        assert_eq!(output, expected);
    }
    #[test]
    #[ignore]
    fn encode_single_characters_mixed_with_repeated_characters() {
        let input = "WWWWWWWWWWWWBWWWWWWWWWWWWBBBWWWWWWWWWWWWWWWWWWWWWWWWB";
        let output = encode(input);
        let expected = "12WB12W3B24WB";
        assert_eq!(output, expected);
    }
    #[test]
    #[ignore]
    fn encode_multiple_whitespace_mixed_in_string() {
        let input = "  hsqq qww  ";
        let output = encode(input);
        let expected = "2 hs2q q2w2 ";
        assert_eq!(output, expected);
    }
    #[test]
    #[ignore]
    fn encode_lowercase_characters() {
        let input = "aabbbcccc";
        let output = encode(input);
        let expected = "2a3b4c";
        assert_eq!(output, expected);
    }
    #[test]
    #[ignore]
    fn decode_empty_string() {
        let input = "";
        let output = decode(input);
        let expected = "";
        assert_eq!(output, expected);
    }
    #[test]
    #[ignore]
    fn decode_single_characters_only() {
        let input = "XYZ";
        let output = decode(input);
        let expected = "XYZ";
        assert_eq!(output, expected);
    }
    #[test]
    #[ignore]
    fn decode_string_with_no_single_characters() {
        let input = "2A3B4C";
        let output = decode(input);
        let expected = "AABBBCCCC";
        assert_eq!(output, expected);
    }
    #[test]
    #[ignore]
    fn decode_single_characters_with_repeated_characters() {
        let input = "12WB12W3B24WB";
        let output = decode(input);
        let expected = "WWWWWWWWWWWWBWWWWWWWWWWWWBBBWWWWWWWWWWWWWWWWWWWWWWWWB";
        assert_eq!(output, expected);
    }
    #[test]
    #[ignore]
    fn decode_multiple_whitespace_mixed_in_string() {
        let input = "2 hs2q q2w2 ";
        let output = decode(input);
        let expected = "  hsqq qww  ";
        assert_eq!(output, expected);
    }
    #[test]
    #[ignore]
    fn decode_lowercase_string() {
        let input = "2a3b4c";
        let output = decode(input);
        let expected = "aabbbcccc";
        assert_eq!(output, expected);
    }
    #[test]
    #[ignore]
    fn consistency_encode_followed_by_decode_gives_original_string() {
        let input = "zzz ZZ  zZ";
        let output = decode(&encode(input));
        let expected = "zzz ZZ  zZ";
        assert_eq!(output, expected);
    }
}
