// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: Apache-2.0 OR MIT

mod suite_parser;

use suite_parser::*;

use libversion::*;

fn parse_flags(flags: &str) -> VersionFlags {
    let mut res = VersionFlags::empty();

    for flag in flags.chars() {
        res |= match flag {
            'p' => VersionFlags::P_IS_PATCH,
            'a' => VersionFlags::ANY_IS_PATCH,
            'l' => VersionFlags::LOWER_BOUND,
            'u' => VersionFlags::UPPER_BOUND,
            _ => VersionFlags::empty(),
        }
    }

    res
}

fn parse_op(op: &str) -> std::cmp::Ordering {
    match op {
        "=" => std::cmp::Ordering::Equal,
        "<" => std::cmp::Ordering::Less,
        ">" => std::cmp::Ordering::Greater,
        _ => panic!("unexpected operator {op}"),
    }
}

fn display_op(op: std::cmp::Ordering) -> &'static str {
    match op {
        std::cmp::Ordering::Equal => "=",
        std::cmp::Ordering::Less => "<",
        std::cmp::Ordering::Greater => ">",
    }
}

#[test]
fn version_comparison_test_suite() {
    let data_path = "testdata/version-comparison-tests.txt";
    let cases = parse_test_suite(data_path);

    for case in cases {
        let left_flags = parse_flags(&case.left_flags);
        let right_flags = parse_flags(&case.right_flags);
        let expected = parse_op(&case.expected_result);

        {
            let result = version_compare4(
                &case.left_version,
                &case.right_version,
                left_flags,
                right_flags,
            );
            println!(
                "{} {}",
                if result == expected { "OK" } else { "FAILED" },
                case.text
            );
            assert!(
                result == expected,
                "Test suite case {}:{}: {} failed with unexpected result {}",
                data_path,
                case.line_number,
                case.text,
                display_op(result)
            );
        }
        {
            let result = version_compare4(
                &case.right_version,
                &case.left_version,
                right_flags,
                left_flags,
            );
            println!(
                "{} {}",
                if result == expected.reverse() {
                    "OK"
                } else {
                    "FAILED"
                },
                case.text
            );
            assert!(
                result == expected.reverse(),
                "Test suite case {}:{}: {} reverse check failed with unexpected result {}",
                data_path,
                case.line_number,
                case.text,
                display_op(result)
            );
        }
    }
}
