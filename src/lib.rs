use crate::compare::compare_components;
use crate::iter::VersionComponentIterator;
use bitflags::bitflags;

mod compare;
mod component;
mod iter;
mod parse;
mod string;

bitflags! {
    #[derive(Clone, Copy, Debug)]
    pub struct Flags: u32 {
        const PIsPatch   = 0b00000001;
        const AnyIsPatch = 0b00000010;
        const LowerBound = 0b00000100;
        const UpperBound = 0b00001000;
    }
}

pub fn version_compare4(v1: &str, v2: &str, v1_flags: Flags, v2_flags: Flags) -> std::cmp::Ordering {
    let mut v1_it = VersionComponentIterator::new(v1, v1_flags);
    let mut v2_it = VersionComponentIterator::new(v2, v2_flags);

    let mut v1_need_extra_component = v1_flags.intersects(Flags::LowerBound | Flags::UpperBound);
    let mut v2_need_extra_component = v2_flags.intersects(Flags::LowerBound | Flags::UpperBound);

    loop {
        let v1_comp = v1_it.next();
        let v2_comp = v2_it.next();

        let res = compare_components(&v1_comp, &v2_comp);
        if res != std::cmp::Ordering::Equal {
            return res;
        }

        if v1_it.is_exhausted() && v2_it.is_exhausted() {
            if !v1_need_extra_component && !v2_need_extra_component {
                return std::cmp::Ordering::Equal;
            }
            if v1_need_extra_component {
                v1_need_extra_component = false;
            }
            if v2_need_extra_component {
                v2_need_extra_component = false;
            }
        }
    }
}

pub fn version_compare2(v1: &str, v2: &str) -> std::cmp::Ordering {
    return version_compare4(v1, v2, Flags::empty(), Flags::empty());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_compare() {
        assert_eq!(version_compare2("1.0", "1.0"), std::cmp::Ordering::Equal);
        assert_eq!(version_compare2("1.0", "1.1"), std::cmp::Ordering::Less);
        assert_eq!(version_compare2("1.1", "1.0"), std::cmp::Ordering::Greater);
    }
}
