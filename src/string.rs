pub fn is_alpha(c: u8) -> bool {
    c >= b'a' && c <= b'z' || c >= b'A' && c <= b'Z'
}

pub fn is_number(c: u8) -> bool {
    c >= b'0' && c <= b'9'
}

pub fn is_separator(c: u8) -> bool {
    !is_alpha(c) && !is_number(c) && c != b'\0'
}

pub fn to_lower(c: u8) -> u8 {
    if c >= b'A' && c <= b'Z' {
        c - b'A' + b'a'
    } else {
        c
    }
}

pub fn strings_are_equal_ci(a: &str, b: &str) -> bool {
    let mut a_it = a.bytes();
    let mut b_it = b.bytes();

    if a.len() != b.len() {
        return false;
    }

    loop {
        match (a_it.next(), b_it.next()) {
            (Some(a_val), Some(b_val)) => {
                if to_lower(a_val) != to_lower(b_val) {
                    return false;
                }
            }
            (None, None) => {
                return true;
            }
            _ => {
                return false;
            }
        }
    }
}

pub fn string_has_prefix_ci(s: &str, prefix: &str) -> bool {
    if s.len() < prefix.len() {
        return false;
    }

    return strings_are_equal_ci(&s[0..prefix.len()], prefix);
}

pub fn split_alpha(s: &str) -> (&str, &str) {
    for (i, c) in s.bytes().enumerate() {
        if !is_alpha(c) {
            return (&s[0..i], &s[i..]);
        }
    }

    return (s, &s[0..0]);
}

pub fn split_number(s: &str) -> (&str, &str) {
    for (i, c) in s.bytes().enumerate() {
        if !is_number(c) {
            return (&s[0..i], &s[i..]);
        }
    }

    return (s, &s[0..0]);
}

pub fn skip_zeroes(s: &str) -> &str {
    for (i, c) in s.bytes().enumerate() {
        if c != b'0' {
            return &s[i..];
        }
    }

    return &s[0..0];
}

pub fn skip_separator(s: &str) -> &str {
    for (i, c) in s.bytes().enumerate() {
        if !is_separator(c) {
            return &s[i..];
        }
    }

    return &s[0..0];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_alpha() {
        assert!(is_alpha(b'a'));
        assert!(is_alpha(b'z'));
        assert!(is_alpha(b'A'));
        assert!(is_alpha(b'Z'));
        assert!(!is_alpha(b'0'));
        assert!(!is_alpha(b'.'));
        assert!(!is_alpha(b'-'));
        assert!(!is_alpha(b' '));
        assert!(!is_alpha(b'\0'));
    }

    #[test]
    fn test_is_number() {
        assert!(is_number(b'0'));
        assert!(is_number(b'9'));
        assert!(!is_number(b'a'));
        assert!(!is_number(b'A'));
        assert!(!is_number(b'.'));
        assert!(!is_number(b'-'));
        assert!(!is_number(b' '));
        assert!(!is_number(b'\0'));
    }

    #[test]
    fn test_is_separator() {
        assert!(is_separator(b'.'));
        assert!(is_separator(b'-'));
        assert!(is_separator(b' '));
        assert!(!is_separator(b'0'));
        assert!(!is_separator(b'9'));
        assert!(!is_separator(b'a'));
        assert!(!is_separator(b'z'));
        assert!(!is_separator(b'A'));
        assert!(!is_separator(b'Z'));
        assert!(!is_separator(b'\0'));
    }

    #[test]
    fn test_to_lower() {
        assert_eq!(to_lower(b'A'), b'a');
        assert_eq!(to_lower(b'Z'), b'z');
        assert_eq!(to_lower(b'a'), b'a');
        assert_eq!(to_lower(b'z'), b'z');
        assert_eq!(to_lower(b'0'), b'0');
        assert_eq!(to_lower(b'-'), b'-');
        assert_eq!(to_lower(b'\0'), b'\0');
    }

    #[test]
    fn test_strings_are_equal_ci() {
        assert!(strings_are_equal_ci("foo", "foo"));
        assert!(strings_are_equal_ci("foo", "FOO"));
        assert!(!strings_are_equal_ci("aaa", "bbb"));
        assert!(!strings_are_equal_ci("bbb", "aaa"));
        assert!(!strings_are_equal_ci("foo", "foox"));
        assert!(!strings_are_equal_ci("foox", "foo"));
    }

    #[test]
    fn test_string_has_prefix_ci() {
        assert!(string_has_prefix_ci("foo", "foo"));
        assert!(string_has_prefix_ci("foo", "FOO"));
        assert!(!string_has_prefix_ci("aaa", "bbb"));
        assert!(!string_has_prefix_ci("bbb", "aaa"));
        assert!(!string_has_prefix_ci("foo", "foox"));
        assert!(string_has_prefix_ci("foox", "foo"));
    }

    #[test]
    fn test_skip_zeroes() {
        assert_eq!(skip_zeroes("0001"), "1");
        assert_eq!(skip_zeroes("1000"), "1000");
        assert_eq!(skip_zeroes("123"), "123");
        assert_eq!(skip_zeroes("000"), "");
    }

    #[test]
    fn test_skip_separator() {
        assert_eq!(skip_separator("-1-"), "1-");
        assert_eq!(skip_separator("1-1"), "1-1");
        assert_eq!(skip_separator("---"), "");
        assert_eq!(skip_separator("abc"), "abc");
    }
}
