use crate::component::Component;
use crate::string::{is_alpha, to_lower};

pub fn compare_components(a: &Component, b: &Component) -> i8 {
    // precedence has highest priority
    if a.precedence < b.precedence {
        return -1;
    }
    if a.precedence > b.precedence {
        return 1;
    }

    // empty strings come before everything
    if a.value.is_empty() && b.value.is_empty() {
        return 0;
    }
    if a.value.is_empty() {
        return -1;
    }
    if b.value.is_empty() {
        return 1;
    }

    // alpha come before numbers
    let a_first = a.value.chars().nth(0).unwrap();
    let b_first = b.value.chars().nth(0).unwrap();
    let a_is_alpha = is_alpha(a_first);
    let b_is_alpha = is_alpha(b_first);

    if a_is_alpha && b_is_alpha {
        if to_lower(a_first) < to_lower(b_first) {
            return -1;
        }
        if to_lower(a_first) > to_lower(b_first) {
            return 1;
        }
        return 0;
    }
    if a_is_alpha {
        return -1;
    }
    if b_is_alpha {
        return 1;
    }

    // numeric comparison (note that leading zeroes are already trimmed here)
    if a.value.len() < b.value.len() {
        return -1;
    }
    if a.value.len() > b.value.len() {
        return 1;
    }

    if a.value < b.value {
        return -1;
    }
    if a.value > b.value {
        return 1;
    }
    return 0;
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
