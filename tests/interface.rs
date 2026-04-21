// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::cmp::Ordering;

use libversion::*;

#[test]
fn test_version_compare2() {
    assert_eq!(version_compare2("1.1", "1.02"), Ordering::Less);
    assert_eq!(version_compare2("1p1", "1p2"), Ordering::Less);
}

#[test]
fn test_version_compare4() {
    assert_eq!(
        version_compare4("1.1", "1.02", VersionFlags::empty(), VersionFlags::empty()),
        Ordering::Less
    );
    assert_eq!(
        version_compare4("1p1", "1p2", VersionFlags::empty(), VersionFlags::empty()),
        Ordering::Less
    );
    assert_eq!(
        version_compare4(
            "1p1",
            "1p2",
            VersionFlags::P_IS_PATCH,
            VersionFlags::empty()
        ),
        Ordering::Greater
    );
}

#[test]
fn test_version_string_eq() {
    assert!(Version::new("1.1") == Version::new("1.01"));
    assert!(Version::new("1.1".to_string()) == Version::new("1.01"));
    assert!(Version::new("1.1") == Version::new("1.01".to_string()));
    assert!(Version::new("1.1".to_string()) == Version::new("1.01".to_string()));

    assert!(
        Version::with_flags("1.1", VersionFlags::empty())
            == Version::with_flags("1.01", VersionFlags::empty())
    );
    assert!(
        Version::with_flags("1.1".to_string(), VersionFlags::empty())
            == Version::with_flags("1.01", VersionFlags::empty())
    );
    assert!(
        Version::with_flags("1.1", VersionFlags::empty())
            == Version::with_flags("1.01".to_string(), VersionFlags::empty())
    );
    assert!(
        Version::with_flags("1.1".to_string(), VersionFlags::empty())
            == Version::with_flags("1.01".to_string(), VersionFlags::empty())
    );

    assert!(
        Version::with_flags("1.1", VersionFlags::LOWER_BOUND)
            != Version::with_flags("1.01", VersionFlags::UPPER_BOUND)
    );
    assert!(
        Version::with_flags("1.1".to_string(), VersionFlags::LOWER_BOUND)
            != Version::with_flags("1.01", VersionFlags::UPPER_BOUND)
    );
    assert!(
        Version::with_flags("1.1", VersionFlags::LOWER_BOUND)
            != Version::with_flags("1.01".to_string(), VersionFlags::UPPER_BOUND)
    );
    assert!(
        Version::with_flags("1.1".to_string(), VersionFlags::LOWER_BOUND)
            != Version::with_flags("1.01".to_string(), VersionFlags::UPPER_BOUND)
    );
}

#[test]
fn test_version_string_ord() {
    assert!(Version::new("1.1") < Version::new("1.02"));
    assert!(Version::new("1.1".to_string()) < Version::new("1.02"));
    assert!(Version::new("1.1") < Version::new("1.02".to_string()));
    assert!(Version::new("1.1".to_string()) < Version::new("1.02".to_string()));

    assert!(
        Version::with_flags("1p1", VersionFlags::empty())
            < Version::with_flags("1p2", VersionFlags::empty())
    );
    assert!(
        Version::with_flags("1p1".to_string(), VersionFlags::empty())
            < Version::with_flags("1p2", VersionFlags::empty())
    );
    assert!(
        Version::with_flags("1p1", VersionFlags::empty())
            < Version::with_flags("1p2".to_string(), VersionFlags::empty())
    );
    assert!(
        Version::with_flags("1p1".to_string(), VersionFlags::empty())
            < Version::with_flags("1p2".to_string(), VersionFlags::empty())
    );

    assert!(
        Version::with_flags("1p1", VersionFlags::P_IS_PATCH)
            > Version::with_flags("1p2", VersionFlags::empty())
    );
    assert!(
        Version::with_flags("1p1".to_string(), VersionFlags::P_IS_PATCH)
            > Version::with_flags("1p2", VersionFlags::empty())
    );
    assert!(
        Version::with_flags("1p1", VersionFlags::P_IS_PATCH)
            > Version::with_flags("1p2".to_string(), VersionFlags::empty())
    );
    assert!(
        Version::with_flags("1p1".to_string(), VersionFlags::P_IS_PATCH)
            > Version::with_flags("1p2".to_string(), VersionFlags::empty())
    );
}
