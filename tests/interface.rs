use libversion::*;
use std::cmp::Ordering;

#[test]
fn version_compare_strings() {
    assert_eq!(version_compare("1.1", "1.2"), Ordering::Less);
    assert_eq!(version_compare("1.1", &String::from("1.2")), Ordering::Less);
    assert_eq!(
        version_compare(&String::from("1.1"), &String::from("1.2")),
        Ordering::Less
    );
}

#[test]
fn version_compare_tuples() {
    assert_eq!(
        version_compare(("1.1", Flags::empty()), ("1.2", Flags::empty())),
        Ordering::Less
    );
    assert_eq!(
        version_compare(
            (&String::from("1.1"), Flags::empty()),
            (&String::from("1.2"), Flags::empty())
        ),
        Ordering::Less
    );
    assert_eq!(
        version_compare(&("1.1", Flags::empty()), &("1.2", Flags::empty())),
        Ordering::Less
    );
    assert_eq!(
        version_compare(
            &(&String::from("1.1"), Flags::empty()),
            &(&String::from("1.2"), Flags::empty())
        ),
        Ordering::Less
    );
}

#[test]
fn version_compare_version_str() {
    assert_eq!(
        version_compare(
            &VersionStr::new("1.1", Flags::empty()),
            &VersionStr::new("1.2", Flags::empty())
        ),
        Ordering::Less
    );
}

#[test]
fn version_compare_version_string() {
    assert_eq!(
        version_compare(
            &VersionString::new(String::from("1.1"), Flags::empty()),
            &VersionString::new(String::from("1.2"), Flags::empty())
        ),
        Ordering::Less
    );
}

#[test]
fn cmp_version_str() {
    assert!(VersionStr::new("1.1", Flags::empty()) < VersionStr::new("1.2", Flags::empty()));
    assert!(
        VersionStr::new("1.1", Flags::empty()) < VersionString::new("1.2".into(), Flags::empty())
    );
    assert!(VersionStr::new("1.1", Flags::empty()) < "1.2");
    assert!(VersionStr::new("1.1", Flags::empty()) < &String::from("1.2"));
    assert!(VersionStr::new("1.1", Flags::empty()) < ("1.2", Flags::empty()));
}

#[test]
fn cmp_version_string() {
    assert!(
        VersionString::new("1.1".into(), Flags::empty()) < VersionStr::new("1.2", Flags::empty())
    );
    assert!(
        VersionString::new("1.1".into(), Flags::empty())
            < VersionString::new("1.2".into(), Flags::empty())
    );
    assert!(VersionString::new("1.1".into(), Flags::empty()) < "1.2");
    assert!(VersionString::new("1.1".into(), Flags::empty()) < &String::from("1.2"));
    assert!(VersionString::new("1.1".into(), Flags::empty()) < ("1.2", Flags::empty()));
}
