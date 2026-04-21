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
        version_compare4("1.1", "1.02", Flags::empty(), Flags::empty()),
        Ordering::Less
    );
    assert_eq!(
        version_compare4("1p1", "1p2", Flags::empty(), Flags::empty()),
        Ordering::Less
    );
    assert_eq!(
        version_compare4("1p1", "1p2", Flags::PIsPatch, Flags::empty()),
        Ordering::Greater
    );
}

#[test]
fn test_version_string_eq() {
    assert!(VersionString::new("1.1") == VersionString::new("1.01"));
    assert!(VersionString::new("1.1".to_string()) == VersionString::new("1.01"));
    assert!(VersionString::new("1.1") == VersionString::new("1.01".to_string()));
    assert!(VersionString::new("1.1".to_string()) == VersionString::new("1.01".to_string()));

    assert!(
        VersionString::with_flags("1.1", Flags::empty())
            == VersionString::with_flags("1.01", Flags::empty())
    );
    assert!(
        VersionString::with_flags("1.1".to_string(), Flags::empty())
            == VersionString::with_flags("1.01", Flags::empty())
    );
    assert!(
        VersionString::with_flags("1.1", Flags::empty())
            == VersionString::with_flags("1.01".to_string(), Flags::empty())
    );
    assert!(
        VersionString::with_flags("1.1".to_string(), Flags::empty())
            == VersionString::with_flags("1.01".to_string(), Flags::empty())
    );

    assert!(
        VersionString::with_flags("1.1", Flags::LowerBound)
            != VersionString::with_flags("1.01", Flags::UpperBound)
    );
    assert!(
        VersionString::with_flags("1.1".to_string(), Flags::LowerBound)
            != VersionString::with_flags("1.01", Flags::UpperBound)
    );
    assert!(
        VersionString::with_flags("1.1", Flags::LowerBound)
            != VersionString::with_flags("1.01".to_string(), Flags::UpperBound)
    );
    assert!(
        VersionString::with_flags("1.1".to_string(), Flags::LowerBound)
            != VersionString::with_flags("1.01".to_string(), Flags::UpperBound)
    );
}

#[test]
fn test_version_string_ord() {
    assert!(VersionString::new("1.1") < VersionString::new("1.02"));
    assert!(VersionString::new("1.1".to_string()) < VersionString::new("1.02"));
    assert!(VersionString::new("1.1") < VersionString::new("1.02".to_string()));
    assert!(VersionString::new("1.1".to_string()) < VersionString::new("1.02".to_string()));

    assert!(
        VersionString::with_flags("1p1", Flags::empty())
            < VersionString::with_flags("1p2", Flags::empty())
    );
    assert!(
        VersionString::with_flags("1p1".to_string(), Flags::empty())
            < VersionString::with_flags("1p2", Flags::empty())
    );
    assert!(
        VersionString::with_flags("1p1", Flags::empty())
            < VersionString::with_flags("1p2".to_string(), Flags::empty())
    );
    assert!(
        VersionString::with_flags("1p1".to_string(), Flags::empty())
            < VersionString::with_flags("1p2".to_string(), Flags::empty())
    );

    assert!(
        VersionString::with_flags("1p1", Flags::PIsPatch)
            > VersionString::with_flags("1p2", Flags::empty())
    );
    assert!(
        VersionString::with_flags("1p1".to_string(), Flags::PIsPatch)
            > VersionString::with_flags("1p2", Flags::empty())
    );
    assert!(
        VersionString::with_flags("1p1", Flags::PIsPatch)
            > VersionString::with_flags("1p2".to_string(), Flags::empty())
    );
    assert!(
        VersionString::with_flags("1p1".to_string(), Flags::PIsPatch)
            > VersionString::with_flags("1p2".to_string(), Flags::empty())
    );
}
