// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Advanced version string comparison library.
//!
//! Need to compare software, package or whatever versions? Comparing
//! `1.0` and `1.1` could be easy, but are you ready for more
//! complex cases like `1.2-x.3~alpha4`? **libversion** is, which
//! is proven by using the library in [Repology](https://repology.org/)
//! project which relies on comparing software version strings, even
//! if they are written in different formats.
//!
//! # Features
//!
//! A short list of version features libversion handles for you:
//!
//! * Simple versions, obviously: `0.9 < 1.0 < 1.1`
//! * Omitting insignificant components: `1.0 == 1.0.0`
//! * Leading zeroes: `1.001 == 1.1`
//! * Unusual separators: `1_2~3 == 1.2.3`
//! * Letter suffixes: `1.2 < 1.2a < 1.2b < 1.3`
//! * Alphanumeric prerelease components:
//!   * `1.0alpha1 == 1.0.alpha1 == 1.0a1 == 1.0.a1`
//!   * `1.0alpha1 < 1.0alpha2 < 1.0beta1 < 1.0rc1 < 1.0`
//! * Awareness of prerelease keywords: while `1.0 < 1.0a-1` (_a_ treated
//!   as version addendum), but `1.0alpha-1 < 1.0` (_alpha_ is treated
//!   as prerelease marker)
//! * Awareness of _patch_, _post_ and _pl_ keywords: while `1.0alpha1 < 1.0`
//!   (_alpha_ is pre-release), `1.0 < 1.0patch1 < 1.1` (_patch_ is post-release)
//! * Customizable handling of _p_ keyword (it may mean either _patch_ or _pre_,
//!   and since libversion cannot guess, this is controlled with an external flag)
//!
//! Version comparison has `O(N)` complexity, implements a total order,
//! is case insensitive, does not allocate, does not fail. Versions strings
//! are treated as ASCII, Unicode characters will be treated as separators.
//!
//! # Examples
//!
//! The library provides both C-ish API resembling one of C `libversion`
//!
//! ```
//! use libversion::{VersionFlags, version_compare2, version_compare4};
//! use std::cmp::Ordering;
//!
//! // Version comparison
//! assert_eq!(version_compare2("0.99", "1.01"), Ordering::Less);
//! assert_eq!(version_compare2("1.1", "1.01.0"), Ordering::Equal);
//! assert_eq!(version_compare2("1.0alpha1", "1.0.beta.1"), Ordering::Less);
//! assert_eq!(version_compare2("1.0.beta.2", "1.0.rc1"), Ordering::Less);
//! assert_eq!(version_compare2("1.0", "1.0-rc1"), Ordering::Greater);
//!
//! // Ordering
//! let mut versions = vec!["1.0rc1", "1.0beta1", "1.0", "1.0alpha1", "0.99"];
//! versions.sort_by(|a, b| version_compare2(a, b));
//! assert_eq!(versions, vec!["0.99", "1.0alpha1", "1.0beta1", "1.0rc1", "1.0"]);
//!
//! // With flags to tune behavior, e.g. how to resolve ambiguous cases
//! assert_eq!(version_compare4("1.0p1", "1.0pre1", VersionFlags::empty(), VersionFlags::empty()), Ordering::Equal);
//! assert_eq!(version_compare4("1.0p1", "1.0patch1", VersionFlags::empty(), VersionFlags::empty()), Ordering::Less);
//! assert_eq!(version_compare4("1.0p1", "1.0pre1", VersionFlags::P_IS_PATCH, VersionFlags::empty()), Ordering::Greater);
//! assert_eq!(version_compare4("1.0p1", "1.0patch1", VersionFlags::P_IS_PATCH, VersionFlags::empty()), Ordering::Equal);
//! ```
//!
//! and a Rust type storing version string along with flags, providing
//! convenient `Eq` and `Ord` implementations.
//!
//! ```
//! use libversion::{VersionFlags, Version};
//!
//! // Use with either owning or borrowed strings
//! assert!(Version::new("1.0") == Version::new("1.0".to_string()));
//!
//! // Compare and order
//! assert!(Version::new("0.99") < Version::new("1.01"));
//! assert!(Version::new("1.1") == Version::new("1.01.0"));
//!
//! let mut versions = vec!["1.0rc1", "1.0beta1", "1.0", "1.0alpha1", "0.99"];
//! versions.sort_by_key(|v| Version::new(*v));
//! assert_eq!(versions, vec!["0.99", "1.0alpha1", "1.0beta1", "1.0rc1", "1.0"]);
//!
//! // With flags
//! assert!(Version::with_flags("1.0p1", VersionFlags::P_IS_PATCH) == Version::new("1.0patch1"));
//! ```

#![forbid(unsafe_code)]

use std::cmp::Ordering;

use bitflags::bitflags;

use crate::iter::VersionComponentIterator;

mod component;
mod iter;
mod parse;
mod string;

bitflags! {
    /// Flags to tweak version comparison.
    #[derive(Clone, Copy, Debug)]
    pub struct VersionFlags: u32 {
        /// Treat `p` component as post-release.
        ///
        /// Instead of treating `p` component as pre-release (same as
        /// `pre`), treat is as post-release (same as `patch`).
        ///
        /// This is useful if it is known that `p` denotes a post-release
        /// in the given case, otherwise it's not known whether it's
        /// pre- or post- and comparison algorithm prefers to treat it
        /// as pre- to avoid ambiguous versions incorrectly outdating
        /// valid versions.
        ///
        /// ```
        /// # use libversion::{Version, VersionFlags};
        /// assert!(Version::new("1.0") > Version::new("1.0p1"));
        /// assert!(Version::new("1.0") < Version::with_flags("1.0p1", VersionFlags::P_IS_PATCH));
        /// ```
        const P_IS_PATCH   = 1;

        /// Treat any alphabetic component as post-release.
        ///
        /// Instead of treating unrecognized alphabetic components as
        /// pre-release (same as `pre`), treat these as post-release
        /// (same as `patch`).
        ///
        /// This is useful if it is known that alphabetic component denotes
        /// a post-release in the given case, otherwise it's not known whether
        /// it's pre- or post- and comparison algorithm prefers to treat it
        /// as pre- to avoid ambiguous versions incorrectly outdating
        /// valid versions.
        ///
        /// ```
        /// # use libversion::{Version, VersionFlags};
        /// assert!(Version::new("1.0") > Version::new("1.0esr1"));
        /// assert!(Version::new("1.0") < Version::with_flags("1.0esr1", VersionFlags::ANY_IS_PATCH));
        /// ```
        const ANY_IS_PATCH = 1 << 1;

        /// Derive lower bound for a given version prefix.
        ///
        /// Treat a given version as a prefix and derive lowest possible imaginary
        /// version with that prefix. That is, `1.0` with this flag would be a "version"
        /// that is lesser than `1.0`, `1.0alpha0`, `1.0alpha0alpha0` and any other
        /// possible version related to `1.0`, but still greater than `0.99999999999999`.
        ///
        /// Together with [`UPPER_BOUND`], this flag is useful to check if a version
        /// belongs to a major/minor/patch release branch, regardless of whether it's
        /// post or pre-release.
        ///
        /// ```
        /// # use libversion::{Version, VersionFlags};
        /// assert!(Version::with_flags("1.0", VersionFlags::LOWER_BOUND) > Version::new("0.99"));
        /// assert!(Version::with_flags("1.0", VersionFlags::LOWER_BOUND) < Version::new("1.0alpha0alpha0alpha0"));
        /// ```
        const LOWER_BOUND  = 1 << 2;

        /// Derive upper bound for a given version prefix.
        ///
        /// Treat a given version as a prefix and derive greatest possible imaginary
        /// version with that prefix. That is, `1.0` with this flag would be a "version"
        /// that is greater than `1.0`, `1.0patch99999`, `1.0post99999` and any other
        /// possible version related to `1.0`, but still lesser than `1.1`.
        ///
        /// Together with [`LOWER_BOUND`], this flag is useful to check if a version
        /// belongs to a major/minor/patch release branch, regardless of whether it's
        /// post or pre-release.
        ///
        /// ```
        /// # use libversion::{Version, VersionFlags};
        /// assert!(Version::with_flags("1.0", VersionFlags::UPPER_BOUND) < Version::new("1.1"));
        /// assert!(Version::with_flags("1.0", VersionFlags::UPPER_BOUND) > Version::new("1.0post999patch999"));
        /// ```
        const UPPER_BOUND  = 1 << 3;
    }
}

/// Compare two versions.
///
/// Compare `v1` with [`VersionFlags`] `v1_flags` against `v2` with [`VersionFlags`] `v2_flags`.
///
/// ```
/// # use std::cmp::Ordering;
/// # use libversion::{VersionFlags, version_compare4};
/// assert_eq!(
///     version_compare4(
///         "1.0",
///         "1.1",
///         VersionFlags::P_IS_PATCH,
///         VersionFlags::empty()
///     ),
///     Ordering::Less
/// );
/// ```
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

/// Compare two versions with default flags.
///
/// Same as `version_compare4(v1, v2, VersionFlags::empty(), VersionFlags::empty())`.
///
/// ```
/// # use std::cmp::Ordering;
/// # use libversion::version_compare2;
/// assert_eq!(
///     version_compare2("1.0", "1.1"),
///     Ordering::Less
/// );
/// ```
pub fn version_compare2(v1: &str, v2: &str) -> Ordering {
    version_compare4(v1, v2, VersionFlags::empty(), VersionFlags::empty())
}

/// Version string with flags.
///
/// A type which holds version string along with associated [`VersionFlags`],
/// allowing comparison and ordering.
///
/// ```
/// # use libversion::Version;
/// assert!(Version::new("1.0") < Version::new("1.1"));
/// assert!(Version::new("1.1") == Version::new("1.0001"));
///
/// assert!(Version::new("1.0") < Version::new("1.1".to_string()));
/// assert!(Version::new("1.1") == Version::new("1.0001".to_string()));
///
/// assert!(Version::new("1.0".to_string()) < Version::new("1.1".to_string()));
/// assert!(Version::new("1.1".to_string()) == Version::new("1.0001".to_string()));
/// ```
pub struct Version<T: AsRef<str> = String> {
    /// Version string.
    pub version: T,

    /// Associated flags to tweak version comparison.
    pub flags: VersionFlags,
}

impl<T: AsRef<str>> Version<T> {
    /// Create version from string.
    ///
    /// Uses default (empty) flags.
    pub fn new(version: T) -> Self {
        Self {
            version,
            flags: VersionFlags::empty(),
        }
    }

    /// Create version from string and flags.
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
