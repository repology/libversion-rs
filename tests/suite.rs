use libversion::*;
use suite_parser::*;
use std::env;

mod suite_parser;

fn parse_flags(flags: &str) -> Flags {
    let mut res = Flags::empty();

    for flag in flags.chars() {
        res |= match flag {
            'p' => Flags::PIsPatch, 
            'a' => Flags::AnyIsPatch, 
            'l' => Flags::LowerBound, 
            'u' => Flags::UpperBound, 
            _ => Flags::empty(),
        }
    }

    return res
}

fn parse_op(op: &str) -> i8 {
    match op {
        "=" => return 0,
        "<" => return -1,
        ">" => return 1,
        _ => panic!("unexpected operator {}", op),
    }
}

fn display_op(op: i8) -> &'static str {
    match op {
        0 => return "=",
        -1 => return "<",
        1 => return ">",
        _ => panic!("unexpected comparison result {}", op),
    }
}

#[test]
fn version_comparison_test_suite() {
    let data_path = "testdata/version-comparison-tests.txt";
    let cases = parse_test_suite(data_path);

    let mut error_count = 0;

    for case in cases {
        let left_flags = parse_flags(&case.left_flags);
        let right_flags = parse_flags(&case.right_flags);
        let expected = parse_op(&case.expected_result);
        let result = version_compare4(
            &case.left_version,
            &case.right_version,
            left_flags,
            right_flags,
        );
        println!("{} {}", if result == expected { "OK" } else { "FAILED" }, case.text);
        assert!(
            result == expected,
            "Test suite case {}:{}: {} failed with unexpected result {}",
            data_path,
            case.line_number,
            case.text,
            display_op(result)
        );
    }
}