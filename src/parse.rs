use crate::component::*;
use crate::string::*;
use crate::Flags;

#[derive(PartialEq, Debug)]
pub enum KeywordClass {
    Unknown,
    PreRelease,
    PostRelease,
}

pub fn classify_keyword(s: &str, flags: Flags) -> KeywordClass {
    if strings_are_equal_ci(s, "alpha") {
        return KeywordClass::PreRelease;
    } else if strings_are_equal_ci(s, "beta") {
        return KeywordClass::PreRelease;
    } else if strings_are_equal_ci(s, "rc") {
        return KeywordClass::PreRelease;
    } else if string_has_prefix_ci(s, "pre") {
        return KeywordClass::PreRelease;
    } else if string_has_prefix_ci(s, "post") {
        return KeywordClass::PostRelease;
    } else if string_has_prefix_ci(s, "patch") {
        return KeywordClass::PostRelease;
    } else if strings_are_equal_ci(s, "pl") {
        // patchlevel
        return KeywordClass::PostRelease;
    } else if strings_are_equal_ci(s, "errata") {
        return KeywordClass::PostRelease;
    } else if flags.contains(Flags::PIsPatch) && strings_are_equal_ci(s, "p") {
        return KeywordClass::PostRelease;
    }
    return KeywordClass::Unknown;
}

pub fn parse_token_to_component(s: &str, flags: Flags) -> (Component, &str) {
    if is_alpha(s.chars().nth(0).unwrap()) {
        let (alpha, rest) = split_alpha(s);
        return (
            Component {
                precedence: match classify_keyword(alpha, flags) {
                    KeywordClass::Unknown => {
                        if flags.contains(Flags::AnyIsPatch) {
                            ComponentPrecedence::PostRelease
                        } else {
                            ComponentPrecedence::PreRelease
                        }
                    }
                    KeywordClass::PreRelease => ComponentPrecedence::PreRelease,
                    KeywordClass::PostRelease => ComponentPrecedence::PostRelease,
                },
                value: alpha,
            },
            rest,
        );
    } else {
        let s = skip_zeroes(s);
        let (number, rest) = split_number(s);
        return (
            Component {
                precedence: if number.is_empty() {
                    ComponentPrecedence::Zero
                } else {
                    ComponentPrecedence::NonZero
                },
                value: number,
            },
            rest,
        );
    }
}

pub fn make_default_component(flags: Flags) -> Component<'static> {
    return Component {
        precedence: if flags.contains(Flags::LowerBound) {
            ComponentPrecedence::LowerBound
        } else if flags.contains(Flags::UpperBound) {
            ComponentPrecedence::UpperBound
        } else {
            ComponentPrecedence::Zero
        },
        value: "",
    };
}

pub enum SomeComponents<'a> {
    One(Component<'a>),
    Two(Component<'a>, Component<'a>),
}

pub fn get_next_version_component(s: &str, flags: Flags) -> (SomeComponents, &str) {
    let s = skip_separator(s);

    if s.is_empty() {
        return (SomeComponents::One(make_default_component(flags)), s);
    }

    let (component, rest) = parse_token_to_component(s, flags);

    let (alpha, rest_after_alpha) = split_alpha(rest);

    if !alpha.is_empty()
        && !rest_after_alpha
            .chars()
            .nth(0)
            .is_some_and(|c| is_number(c))
    {
        return (
            SomeComponents::Two(
                component,
                Component {
                    precedence: match classify_keyword(alpha, flags) {
                        KeywordClass::Unknown => ComponentPrecedence::LetterSuffix,
                        KeywordClass::PreRelease => ComponentPrecedence::PreRelease,
                        KeywordClass::PostRelease => ComponentPrecedence::PostRelease,
                    },
                    value: alpha,
                },
            ),
            rest_after_alpha,
        );
    }

    return (SomeComponents::One(component), rest);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classify_keyword() {
        assert_eq!(
            classify_keyword("ALPHA", Flags::empty()),
            KeywordClass::PreRelease
        );
        assert_eq!(
            classify_keyword("ALPHABET", Flags::empty()),
            KeywordClass::Unknown
        );
        assert_eq!(
            classify_keyword("BETA", Flags::empty()),
            KeywordClass::PreRelease
        );
        assert_eq!(
            classify_keyword("BETAKE", Flags::empty()),
            KeywordClass::Unknown
        );
        assert_eq!(
            classify_keyword("RC", Flags::empty()),
            KeywordClass::PreRelease
        );
        assert_eq!(
            classify_keyword("PRE", Flags::empty()),
            KeywordClass::PreRelease
        );
        assert_eq!(
            classify_keyword("PRERELEASE", Flags::empty()),
            KeywordClass::PreRelease
        );
        assert_eq!(
            classify_keyword("POST", Flags::empty()),
            KeywordClass::PostRelease
        );
        assert_eq!(
            classify_keyword("POSTRELEASE", Flags::empty()),
            KeywordClass::PostRelease
        );
        assert_eq!(
            classify_keyword("PATCH", Flags::empty()),
            KeywordClass::PostRelease
        );
        assert_eq!(
            classify_keyword("PATCHLEVEL", Flags::empty()),
            KeywordClass::PostRelease
        );
        assert_eq!(
            classify_keyword("PL", Flags::empty()),
            KeywordClass::PostRelease
        );
        assert_eq!(
            classify_keyword("ERRATA", Flags::empty()),
            KeywordClass::PostRelease
        );

        assert_eq!(
            classify_keyword("FOOBAR", Flags::empty()),
            KeywordClass::Unknown
        );
    }
}
