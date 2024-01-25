use crate::component::Component;
use crate::string::{is_alpha, to_lower};

macro_rules! return_if_nonequal {
    ($a:expr, $b:expr) => {
        {
            if $a < $b {
                return std::cmp::Ordering::Less;
            }
            if $a > $b {
                return std::cmp::Ordering::Greater;
            }
        }
    }
}

pub fn compare_components(a: &Component, b: &Component) -> std::cmp::Ordering {
    // precedence has highest priority
    return_if_nonequal!(a.precedence, b.precedence);

    // empty strings come before everything
    if a.value.is_empty() && b.value.is_empty() {
        return std::cmp::Ordering::Equal;
    }
    if a.value.is_empty() {
        return std::cmp::Ordering::Less;
    }
    if b.value.is_empty() {
        return std::cmp::Ordering::Greater;
    }

    // alpha come before numbers
    let a_first = a.value.chars().nth(0).unwrap();
    let b_first = b.value.chars().nth(0).unwrap();
    let a_is_alpha = is_alpha(a_first);
    let b_is_alpha = is_alpha(b_first);

    if a_is_alpha && b_is_alpha {
        return_if_nonequal!(to_lower(a_first), to_lower(b_first));
        return std::cmp::Ordering::Equal;
    }
    if a_is_alpha {
        return std::cmp::Ordering::Less;
    }
    if b_is_alpha {
        return std::cmp::Ordering::Greater;
    }

    // numeric comparison (note that leading zeroes are already trimmed here)
    return_if_nonequal!(a.value.len(), b.value.len());
    return_if_nonequal!(a.value, b.value);
    return std::cmp::Ordering::Equal;
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
