use nom::{bytes::complete::take_while, IResult};

/// Get a substring between start (inclusive) and end (exclusive) indices of a `&str`
pub(crate) fn substring(input: &str, start: usize, end: usize) -> String {
    assert!(end > start);
    input.chars().skip(start).take(end - start).collect()
}

/// Parse a `&str` until a non-digit character is reached
pub(crate) fn take_digits(s: &str) -> IResult<&str, &str> {
    take_while(|c: char| c.is_digit(10))(s)
}

/// Parse a `&str` until a non-space character is reached
pub(crate) fn take_whitespace(s: &str) -> IResult<&str, &str> {
    take_while(|c: char| c == ' ')(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_substring() {
        assert_eq!(String::from(""), substring("", 0, 1));
        assert_eq!(String::from("2"), substring("2 3 4", 0, 1));
        assert_eq!(String::from("bcd"), substring("abcde", 1, 4));
        assert_eq!(String::from("beef"), substring("beef", 0, 4));
        assert_eq!(String::from(""), substring("apple", 5, 6));
        assert_eq!(String::from("apple"), substring("apple", 0, 999));
    }

    #[test]
    fn test_take_digits() {
        assert_eq!(take_digits(""), Ok(("", "")));
        assert_eq!(take_digits("1+2"), Ok(("+2", "1")));
        assert_eq!(take_digits("1 + 2"), Ok((" + 2", "1")));
        assert_eq!(take_digits("12345"), Ok(("", "12345")));
        assert_eq!(take_digits("abcde"), Ok(("abcde", "")));
    }

    #[test]
    fn test_take_whitespace() {
        assert_eq!(take_whitespace(""), Ok(("", "")));
        assert_eq!(take_whitespace(" "), Ok(("", " ")));
        assert_eq!(take_whitespace("  "), Ok(("", "  ")));
        assert_eq!(take_whitespace(" 1 "), Ok(("1 ", " ")));
    }
}
