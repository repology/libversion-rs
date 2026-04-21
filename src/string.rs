// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! # Low level ASCII char and string functions

/// Check if a byte is version component separator
pub fn is_separator(c: u8) -> bool {
    !c.is_ascii_alphabetic() && !c.is_ascii_digit()
}

pub fn string_is_equal_to_lowercase(a: &str, b: &str) -> bool {
    a.len() == b.len() && a.bytes().map(|c| c.to_ascii_lowercase()).eq(b.bytes())
}

pub fn string_has_prefix_lowercase(s: &str, prefix: &str) -> bool {
    s.len() >= prefix.len() && string_is_equal_to_lowercase(&s[0..prefix.len()], prefix)
}

pub fn split_alpha(s: &str) -> (&str, &str) {
    let pos = s
        .bytes()
        .position(|c| !c.is_ascii_alphabetic())
        .unwrap_or(s.len());
    (&s[0..pos], &s[pos..])
}

pub fn split_number(s: &str) -> (&str, &str) {
    let pos = s
        .bytes()
        .position(|c| !c.is_ascii_digit())
        .unwrap_or(s.len());
    (&s[0..pos], &s[pos..])
}

pub fn skip_zeroes(s: &str) -> &str {
    let pos = s.bytes().position(|c| c != b'0').unwrap_or(s.len());
    &s[pos..]
}

pub fn skip_separator(s: &str) -> &str {
    let pos = s.bytes().position(|c| !is_separator(c)).unwrap_or(s.len());
    &s[pos..]
}

#[cfg(test)]
mod tests {
    use super::*;

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
    }

    #[test]
    fn test_string_is_equal_to_lowercase() {
        assert!(string_is_equal_to_lowercase("foo", "foo"));
        assert!(string_is_equal_to_lowercase("FOO", "foo"));
        assert!(!string_is_equal_to_lowercase("foo", "bar"));
    }

    #[test]
    fn test_string_has_prefix_ci() {
        assert!(string_has_prefix_lowercase("foo", "foo"));
        assert!(string_has_prefix_lowercase("foobar", "foo"));
        assert!(string_has_prefix_lowercase("FOOBAR", "foo"));
        assert!(!string_has_prefix_lowercase("foo", "bar"));
        assert!(!string_has_prefix_lowercase("foobar", "bar"));
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
