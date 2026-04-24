// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::VersionFlags;
use crate::component::Component;
use crate::string::{
    skip_separator, skip_zeroes, split_alpha, split_number, string_has_prefix_lowercase,
    string_is_equal_to_lowercase,
};

#[derive(PartialEq, Debug)]
pub(crate) enum KeywordClass {
    Unknown,
    PreRelease,
    PostRelease,
}

pub(crate) fn classify_keyword(s: &str, flags: VersionFlags) -> KeywordClass {
    if string_is_equal_to_lowercase(s, "alpha")
        || string_is_equal_to_lowercase(s, "beta")
        || string_is_equal_to_lowercase(s, "rc")
        || string_has_prefix_lowercase(s, "pre")
    {
        return KeywordClass::PreRelease;
    }

    if string_is_equal_to_lowercase(s, "errata")
        || string_is_equal_to_lowercase(s, "pl")
        || string_has_prefix_lowercase(s, "patch")
        || string_has_prefix_lowercase(s, "post")
        || (flags.contains(VersionFlags::P_IS_PATCH) && string_is_equal_to_lowercase(s, "p"))
    {
        return KeywordClass::PostRelease;
    }

    KeywordClass::Unknown
}

pub(crate) fn parse_token_to_component(input: &str, flags: VersionFlags) -> (Component<'_>, &str) {
    let (alpha, rest) = split_alpha(input);
    if let Some(first_char) = alpha.as_bytes().first().map(|c| c.to_ascii_lowercase()) {
        (
            match classify_keyword(alpha, flags) {
                KeywordClass::Unknown => {
                    if flags.contains(VersionFlags::ANY_IS_PATCH) {
                        Component::PostRelease(first_char)
                    } else {
                        Component::PreRelease(first_char)
                    }
                }
                KeywordClass::PreRelease => Component::PreRelease(first_char),
                KeywordClass::PostRelease => Component::PostRelease(first_char),
            },
            rest,
        )
    } else {
        let (number, rest) = split_number(skip_zeroes(input));
        (
            if number.is_empty() {
                Component::Zero
            } else {
                Component::NonZero(number)
            },
            rest,
        )
    }
}

pub(crate) fn make_default_component(flags: VersionFlags) -> Component<'static> {
    if flags.contains(VersionFlags::LOWER_BOUND) {
        Component::LowerBound
    } else if flags.contains(VersionFlags::UPPER_BOUND) {
        Component::UpperBound
    } else {
        Component::Zero
    }
}

pub(crate) enum SomeComponents<'a> {
    One(Component<'a>),
    Two(Component<'a>, Component<'a>),
}

pub(crate) fn get_next_version_component(
    s: &str,
    flags: VersionFlags,
) -> (SomeComponents<'_>, &str) {
    let s = skip_separator(s);

    if s.is_empty() {
        return (SomeComponents::One(make_default_component(flags)), s);
    }

    let (component, rest) = parse_token_to_component(s, flags);

    let (alpha, rest_after_alpha) = split_alpha(rest);

    if let Some(first_char) = alpha.as_bytes().first().map(|c| c.to_ascii_lowercase())
        && !rest_after_alpha
            .as_bytes()
            .first()
            .copied()
            .is_some_and(|c| c.is_ascii_digit())
    {
        return (
            SomeComponents::Two(
                component,
                match classify_keyword(alpha, flags) {
                    KeywordClass::Unknown => Component::LetterSuffix(first_char),
                    KeywordClass::PreRelease => Component::PreRelease(first_char),
                    KeywordClass::PostRelease => Component::PostRelease(first_char),
                },
            ),
            rest_after_alpha,
        );
    }

    (SomeComponents::One(component), rest)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classify_keyword() {
        assert_eq!(
            classify_keyword("ALPHA", VersionFlags::empty()),
            KeywordClass::PreRelease
        );
        assert_eq!(
            classify_keyword("ALPHABET", VersionFlags::empty()),
            KeywordClass::Unknown
        );
        assert_eq!(
            classify_keyword("BETA", VersionFlags::empty()),
            KeywordClass::PreRelease
        );
        assert_eq!(
            classify_keyword("BETAKE", VersionFlags::empty()),
            KeywordClass::Unknown
        );
        assert_eq!(
            classify_keyword("RC", VersionFlags::empty()),
            KeywordClass::PreRelease
        );
        assert_eq!(
            classify_keyword("PRE", VersionFlags::empty()),
            KeywordClass::PreRelease
        );
        assert_eq!(
            classify_keyword("PRERELEASE", VersionFlags::empty()),
            KeywordClass::PreRelease
        );
        assert_eq!(
            classify_keyword("POST", VersionFlags::empty()),
            KeywordClass::PostRelease
        );
        assert_eq!(
            classify_keyword("POSTRELEASE", VersionFlags::empty()),
            KeywordClass::PostRelease
        );
        assert_eq!(
            classify_keyword("PATCH", VersionFlags::empty()),
            KeywordClass::PostRelease
        );
        assert_eq!(
            classify_keyword("PATCHLEVEL", VersionFlags::empty()),
            KeywordClass::PostRelease
        );
        assert_eq!(
            classify_keyword("PL", VersionFlags::empty()),
            KeywordClass::PostRelease
        );
        assert_eq!(
            classify_keyword("ERRATA", VersionFlags::empty()),
            KeywordClass::PostRelease
        );

        assert_eq!(
            classify_keyword("FOOBAR", VersionFlags::empty()),
            KeywordClass::Unknown
        );
    }
}
