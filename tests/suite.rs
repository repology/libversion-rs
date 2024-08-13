use libversion::*;
use suite_parser::*;

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

    return res;
}

fn parse_op(op: &str) -> std::cmp::Ordering {
    match op {
        "=" => return std::cmp::Ordering::Equal,
        "<" => return std::cmp::Ordering::Less,
        ">" => return std::cmp::Ordering::Greater,
        _ => panic!("unexpected operator {}", op),
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
            let result = version_compare(
                (&case.left_version, left_flags),
                (&case.right_version, right_flags),
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
            let result = version_compare(
                (&case.right_version, right_flags),
                (&case.left_version, left_flags),
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
