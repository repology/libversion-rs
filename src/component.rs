use crate::string::{is_alpha, to_lower};

#[derive(PartialOrd, PartialEq, Eq, Ord, Debug)]
pub enum ComponentPrecedence {
    LowerBound,
    PreRelease,
    Zero,
    PostRelease,
    NonZero,
    LetterSuffix,
    UpperBound,
}

#[derive(Debug)]
pub struct Component<'a> {
    pub precedence: ComponentPrecedence,
    pub value: &'a str,
}

impl Ord for Component<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.precedence.cmp(&other.precedence).then_with(|| {
            let self_first_char = self.value.chars().nth(0);
            if self_first_char.is_some_and(|ch| is_alpha(ch)) {
                // string comparison: one of args is alphabetic, other is too
                // compare lowercase (which provides us case insensitivity) of their
                // first letters
                let other_first_char = other.value.chars().nth(0).unwrap();
                to_lower(self_first_char.unwrap()).cmp(&to_lower(other_first_char))
            } else {
                // numeric comparison: compare lengths, then values, which
                // allows numeric comparison of arbitrary long numbers
                // note that leading zeroes are already skipped here
                self.value
                    .len()
                    .cmp(&other.value.len())
                    .then_with(|| self.value.cmp(&other.value))
            }
        })
    }
}

impl PartialOrd for Component<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Component<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(&other) == std::cmp::Ordering::Equal
    }
}

impl Eq for Component<'_> {}
