use bitflags::bitflags;
use crate::iter::VersionComponentIterator;
use crate::compare::compare_components;

mod string;
mod parse;
mod component;
mod compare;
mod iter;

bitflags! {
    #[derive(Clone, Copy)]
    pub struct Flags: u32 {
        const PIsPatch   = 0b00000001;
        const AnyIsPatch = 0b00000010;
        const LowerBound = 0b00000100;
        const UpperBound = 0b00001000;
    }
}

pub fn version_compare4(v1: &str, v2: &str, v1_flags: Flags, v2_flags: Flags) -> i8 {
    let mut v1_it = VersionComponentIterator::new(v1, v1_flags);
    let mut v2_it = VersionComponentIterator::new(v2, v2_flags);

    let mut v1_need_extra_component = v1_flags.intersects(Flags::LowerBound | Flags::UpperBound);
    let mut v2_need_extra_component = v2_flags.intersects(Flags::LowerBound | Flags::UpperBound);

    loop {
        let v1_comp = v1_it.next();
        let v2_comp = v2_it.next();

        let res = compare_components(&v1_comp, &v2_comp);
        if res != 0 {
            return res;
        }

        if v1_it.is_exhausted() && v2_it.is_exhausted() {
            if !v1_need_extra_component && !v2_need_extra_component {
                return 0;
            }
            if v1_need_extra_component {
                v1_need_extra_component = false;
            }
            if v2_need_extra_component {
                v2_need_extra_component = false;
            }
        }
    }

    return 0;
}

pub fn version_compare2(v1: &str, v2: &str) -> i8 {
    return version_compare4(v1, v2, Flags::empty(), Flags::empty())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_compare() {
        assert_eq!(version_compare2("1.0", "1.0"), 0);
        assert_eq!(version_compare2("1.0", "1.1"), -1);
        assert_eq!(version_compare2("1.1", "1.0"), 1);
    }
}
