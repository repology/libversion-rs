// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};

use bitflags::bitflags;

use crate::iter::VersionComponentIterator;

mod component;
mod iter;
mod parse;
mod string;

bitflags! {
    #[derive(Clone, Copy, Debug)]
    pub struct VersionFlags: u32 {
        const P_IS_PATCH   = 1;
        const ANY_IS_PATCH = 1 << 1;
        const LOWER_BOUND  = 1 << 2;
        const UPPER_BOUND  = 1 << 3;
    }
}

pub fn version_compare4(
    v1: &str,
    v2: &str,
    v1_flags: VersionFlags,
    v2_flags: VersionFlags,
) -> Ordering {
    let mut v1_it = VersionComponentIterator::new(v1, v1_flags);
    let mut v2_it = VersionComponentIterator::new(v2, v2_flags);

    let mut will_need_extra_component = v1_flags
        .intersects(VersionFlags::LOWER_BOUND | VersionFlags::UPPER_BOUND)
        || v2_flags.intersects(VersionFlags::LOWER_BOUND | VersionFlags::UPPER_BOUND);

    loop {
        let v1_comp = v1_it.next();
        let v2_comp = v2_it.next();

        let res = v1_comp.cmp(&v2_comp);
        if res != Ordering::Equal {
            return res;
        }

        if v1_it.is_exhausted() && v2_it.is_exhausted() {
            if will_need_extra_component {
                will_need_extra_component = false;
            } else {
                return Ordering::Equal;
            }
        }
    }
}

pub fn version_compare2(v1: &str, v2: &str) -> Ordering {
    version_compare4(v1, v2, VersionFlags::empty(), VersionFlags::empty())
}

pub struct Version<T: AsRef<str> = String> {
    pub version: T,
    pub flags: VersionFlags,
}

impl<T: AsRef<str>> Version<T> {
    pub fn new(version: T) -> Self {
        Self {
            version,
            flags: VersionFlags::empty(),
        }
    }

    pub fn with_flags(version: T, flags: VersionFlags) -> Self {
        Self { version, flags }
    }
}

impl<T, U> PartialEq<Version<U>> for Version<T>
where
    T: AsRef<str>,
    U: AsRef<str>,
{
    fn eq(&self, other: &Version<U>) -> bool {
        version_compare4(
            self.version.as_ref(),
            other.version.as_ref(),
            self.flags,
            other.flags,
        ) == Ordering::Equal
    }
}

impl<T, U> PartialOrd<Version<U>> for Version<T>
where
    T: AsRef<str>,
    U: AsRef<str>,
{
    fn partial_cmp(&self, other: &Version<U>) -> Option<Ordering> {
        Some(version_compare4(
            self.version.as_ref(),
            other.version.as_ref(),
            self.flags,
            other.flags,
        ))
    }
}

impl<T: AsRef<str>> Eq for Version<T> {}

impl<T: AsRef<str>> Ord for Version<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        version_compare4(
            self.version.as_ref(),
            other.version.as_ref(),
            self.flags,
            other.flags,
        )
    }
}
