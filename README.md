# libversion

Advanced version string comparison library.

Need to compare software, package or whatever versions? Comparing
`1.0` and `1.1` could be easy, but are you ready for more
complex cases like `1.2-x.3~alpha4`? **libversion** is, which
is proven by using the library in [Repology](https://repology.org/)
project which relies on comparing software version strings, even
if they are written in different formats.

## Features

A short list of version features libversion handles for you:

* Simple versions, obviously: `0.9 < 1.0 < 1.1`
* Omitting insignificant components: `1.0 == 1.0.0`
* Leading zeroes: `1.001 == 1.1`
* Unusual separators: `1_2~3 == 1.2.3`
* Letter suffixes: `1.2 < 1.2a < 1.2b < 1.3`
* Alphanumeric prerelease components:
  * `1.0alpha1 == 1.0.alpha1 == 1.0a1 == 1.0.a1`
  * `1.0alpha1 < 1.0alpha2 < 1.0beta1 < 1.0rc1 < 1.0`
* Awareness of prerelease keywords: while `1.0 < 1.0a-1` (_a_ treated
  as version addendum), but `1.0alpha-1 < 1.0` (_alpha_ is treated
  as prerelease marker)
* Awareness of _patch_, _post_ and _pl_ keywords: while `1.0alpha1 < 1.0`
  (_alpha_ is pre-release), `1.0 < 1.0patch1 < 1.1` (_patch_ is post-release)
* Customizable handling of _p_ keyword (it may mean either _patch_ or _pre_,
  and since libversion cannot guess, this is controlled with an external flag)

Version comparison has `O(N)` complexity, implements a total order,
is case insensitive, does not allocate, does not fail. Versions strings
are treated as ASCII, Unicode characters will be treated as separators.

## Examples

The library provides both C-ish API resembling one of C `libversion`

```rust
use libversion::{VersionFlags, version_compare2, version_compare4};
use std::cmp::Ordering;

// Version comparison
assert_eq!(version_compare2("0.99", "1.01"), Ordering::Less);
assert_eq!(version_compare2("1.1", "1.01.0"), Ordering::Equal);
assert_eq!(version_compare2("1.0alpha1", "1.0.beta.1"), Ordering::Less);
assert_eq!(version_compare2("1.0.beta.2", "1.0.rc1"), Ordering::Less);
assert_eq!(version_compare2("1.0", "1.0-rc1"), Ordering::Greater);

// Ordering
let mut versions = vec!["1.0rc1", "1.0beta1", "1.0", "1.0alpha1", "0.99"];
versions.sort_by(|a, b| version_compare2(a, b));
assert_eq!(versions, vec!["0.99", "1.0alpha1", "1.0beta1", "1.0rc1", "1.0"]);

// With flags to tune behavior, e.g. how to resolve ambiguous cases
assert_eq!(version_compare4("1.0p1", "1.0pre1", VersionFlags::empty(), VersionFlags::empty()), Ordering::Equal);
assert_eq!(version_compare4("1.0p1", "1.0patch1", VersionFlags::empty(), VersionFlags::empty()), Ordering::Less);
assert_eq!(version_compare4("1.0p1", "1.0pre1", VersionFlags::P_IS_PATCH, VersionFlags::empty()), Ordering::Greater);
assert_eq!(version_compare4("1.0p1", "1.0patch1", VersionFlags::P_IS_PATCH, VersionFlags::empty()), Ordering::Equal);
```

and a Rust type storing version string along with flags, providing
convenient `Eq` and `Ord` implementations.

```rust
use libversion::{VersionFlags, Version};

// Use with either owning or borrowed strings
assert!(Version::new("1.0") == Version::new("1.0".to_string()));

// Compare and order
assert!(Version::new("0.99") < Version::new("1.01"));
assert!(Version::new("1.1") == Version::new("1.01.0"));

let mut versions = vec!["1.0rc1", "1.0beta1", "1.0", "1.0alpha1", "0.99"];
versions.sort_by_key(|v| Version::new(*v));
assert_eq!(versions, vec!["0.99", "1.0alpha1", "1.0beta1", "1.0rc1", "1.0"]);

// With flags
assert!(Version::with_flags("1.0p1", VersionFlags::P_IS_PATCH) == Version::new("1.0patch1"));
```

## Safety

This crate uses `#![forbid(unsafe_code)]` to ensure everything is implemented in 100% safe Rust.

## Supported Rust versions

`libversion` supports current stable Rust version and 2 most recent minor releases before it.
Increasing MSRV is not considered a semver breaking change as long as it follows this policy.
The current MSRV is 1.88.

## Documentation

See https://docs.rs/libversion/latest/libversion/ for complete documentation.

## Bindings and compatible implementations

* Go: [golibversion](https://github.com/saenai255/golibversion) by @saenai255
* Perl: [Version::libversion::XS](https://github.com/giterlizzi/perl-Version-libversion-XS) by @giterlizzi
* Python: [py-libversion](https://github.com/repology/py-libversion) by @AMDmi3
* Raku: [Version::Repology](https://raku.land/zef:lizmat/Version::Repology) by @lizmat (a pure Raku implementation)
* Ruby: [ruby-libversion](https://github.com/Zopolis4/ruby-libversion) by @Zopolis4
* C: [libversion](https://github.com/repology/libversion) by @AMDmi3 (original C implementation)

## Author

* [Dmitry Marakasov](https://github.com/AMDmi3) <amdmi3@amdmi3.ru>

## License

* [MIT](COPYING-MIT) OR [Apache-2.0](COPYING-APACHE)
