pub fn is_alpha(c: char) -> bool {
    return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z');
}

pub fn is_number(c: char) -> bool {
    return c >= '0' && c <= '9';
}

pub fn is_separator(c: char) -> bool {
    return !is_alpha(c) && !is_number(c) && c != '\0';
}

pub fn to_lower(c: char) -> char {
    if c >= 'A' && c <= 'Z' {
        return char::from_u32(c as u32 - 'A' as u32 + 'a' as u32).unwrap_or(c);
    } else {
        return c;
    }
}

pub fn strings_are_equal_ci(a: &str, b: &str) -> bool {
    let mut a_it = a.chars();
    let mut b_it = b.chars();

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

pub fn skip_alpha(s: &str) -> &str {
    for (i, c) in s.chars().enumerate() {
        if !is_alpha(c) {
            return &s[i..];
        }
    }

    return &s[0..0];
}

pub fn split_alpha(s: &str) -> (&str, &str) {
    for (i, c) in s.chars().enumerate() {
        if !is_alpha(c) {
            return (&s[0..i], &s[i..]);
        }
    }

    return (s, &s[0..0]);
}

pub fn skip_number(s: &str) -> &str {
    let chars = s.chars();

    for (i, c) in s.chars().enumerate() {
        if !is_number(c) {
            return &s[i..];
        }
    }

    return &s[0..0];
}

pub fn split_number(s: &str) -> (&str, &str) {
    for (i, c) in s.chars().enumerate() {
        if !is_number(c) {
            return (&s[0..i], &s[i..]);
        }
    }

    return (s, &s[0..0]);
}

pub fn skip_zeroes(s: &str) -> &str {
    let chars = s.chars();

    for (i, c) in s.chars().enumerate() {
        if c != '0' {
            return &s[i..];
        }
    }

    return &s[0..0];
}

pub fn skip_separator(s: &str) -> &str {
    let chars = s.chars();

    for (i, c) in s.chars().enumerate() {
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
        assert!(is_alpha('a'));
        assert!(is_alpha('z'));
        assert!(is_alpha('A'));
        assert!(is_alpha('Z'));
        assert!(!is_alpha('0'));
        assert!(!is_alpha('.'));
        assert!(!is_alpha('-'));
        assert!(!is_alpha(' '));
        assert!(!is_alpha('\0'));
    }

    #[test]
    fn test_is_number() {
        assert!(is_number('0'));
        assert!(is_number('9'));
        assert!(!is_number('a'));
        assert!(!is_number('A'));
        assert!(!is_number('.'));
        assert!(!is_number('-'));
        assert!(!is_number(' '));
        assert!(!is_number('\0'));
    }

    #[test]
    fn test_is_separator() {
        assert!(is_separator('.'));
        assert!(is_separator('-'));
        assert!(is_separator(' '));
        assert!(!is_separator('0'));
        assert!(!is_separator('9'));
        assert!(!is_separator('a'));
        assert!(!is_separator('z'));
        assert!(!is_separator('A'));
        assert!(!is_separator('Z'));
        assert!(!is_separator('\0'));
    }

    #[test]
    fn test_to_lower() {
        assert_eq!(to_lower('A'), 'a');
        assert_eq!(to_lower('Z'), 'z');
        assert_eq!(to_lower('a'), 'a');
        assert_eq!(to_lower('z'), 'z');
        assert_eq!(to_lower('0'), '0');
        assert_eq!(to_lower('-'), '-');
        assert_eq!(to_lower('\0'), '\0');
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
    fn test_skip_alpha() {
        assert_eq!(skip_alpha("foo123"), "123");
        assert_eq!(skip_alpha("123foo"), "123foo");
        assert_eq!(skip_alpha("123"), "123");
        assert_eq!(skip_alpha("foo"), "");
    }

    #[test]
    fn test_skip_number() {
        assert_eq!(skip_number("foo123"), "foo123");
        assert_eq!(skip_number("123foo"), "foo");
        assert_eq!(skip_number("123"), "");
        assert_eq!(skip_number("foo"), "foo");
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
