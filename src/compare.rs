use crate::component::Component;
use crate::string::{is_alpha, to_lower};

pub fn compare_components(a: &Component, b: &Component) -> std::cmp::Ordering {
    a.precedence.cmp(&b.precedence).then_with(|| {
        let a_first = a.value.chars().nth(0);
        if a_first.is_some_and(|c| is_alpha(c)) {
            // string comparison: one of args is alphabetic, other is too
            // compare lowercase (which provides us case insensitivity) of their
            // first letters
            let b_first = b.value.chars().nth(0).unwrap();
            to_lower(a_first.unwrap()).cmp(&to_lower(b_first))
        } else {
            // numeric comparison: compare lengths, then values, which
            // allows numeric comparison of arbitrary long numbers
            // note that leading zeroes are already skipped here
            a.value
                .len()
                .cmp(&b.value.len())
                .then_with(|| a.value.cmp(&b.value))
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        //assert_eq!(version_compare2("1.0", "1.0"), 0);
        //assert_eq!(version_compare2("1.0", "1.1"), -1);
        //assert_eq!(version_compare2("1.1", "1.0"), 1);
    }
}
