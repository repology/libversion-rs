// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: Apache-2.0 OR MIT

#[derive(Debug, PartialEq, Eq)]
pub enum Component<'a> {
    LowerBound,
    PreRelease(u8),
    Zero,
    PostRelease(u8),
    NonZero(&'a str),
    LetterSuffix(u8),
    UpperBound,
}

impl Component<'_> {
    fn discriminant(&self) -> u8 {
        match self {
            Self::LowerBound => 0,
            Self::PreRelease(_) => 1,
            Self::Zero => 2,
            Self::PostRelease(_) => 3,
            Self::NonZero(_) => 4,
            Self::LetterSuffix(_) => 5,
            Self::UpperBound => 6,
        }
    }
}

impl Ord for Component<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.discriminant()
            .cmp(&other.discriminant())
            .then_with(|| match (self, other) {
                (Component::PreRelease(a), Component::PreRelease(b)) => a.cmp(b),
                (Component::PostRelease(a), Component::PostRelease(b)) => a.cmp(b),
                (Component::NonZero(a), Component::NonZero(b)) => {
                    a.len().cmp(&b.len()).then_with(|| a.cmp(b))
                }
                (Component::LetterSuffix(a), Component::LetterSuffix(b)) => a.cmp(b),
                _ => std::cmp::Ordering::Equal,
            })
    }
}

impl PartialOrd for Component<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn test_component_order() {
        let ordered = [
            Component::LowerBound,
            Component::PreRelease(b'a'),
            Component::PreRelease(b'b'),
            Component::PreRelease(b'z'),
            Component::Zero,
            Component::PostRelease(b'a'),
            Component::PostRelease(b'b'),
            Component::PostRelease(b'z'),
            Component::NonZero("0"),
            Component::NonZero("1"),
            Component::NonZero("9"),
            Component::NonZero("10"),
            Component::NonZero("99"),
            Component::NonZero("100"),
            Component::LetterSuffix(b'a'),
            Component::LetterSuffix(b'b'),
            Component::LetterSuffix(b'z'),
            Component::UpperBound,
        ];

        for (lhs, rhs) in ordered.iter().tuple_windows() {
            assert!(
                lhs < rhs,
                "Component ordering violation, {lhs:?} must be lower than {rhs:?}"
            );
        }
    }
}
