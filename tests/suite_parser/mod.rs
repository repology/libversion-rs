use regex::Regex;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
pub struct TestCase {
    pub line_number: usize,
    pub text: String,
    pub left_version: String,
    pub left_flags: String,
    pub expected_result: String,
    pub right_flags: String,
    pub right_version: String,
}

pub fn parse_test_suite(path: &str) -> Vec<TestCase> {
    let f = File::open(path).unwrap();
    let reader = BufReader::new(f);

    let mut cases: Vec<TestCase> = Vec::new();

    let re = Regex::new("^\"(.*)\" ([a-z]*)([<>=])([a-z]*) \"(.*)\"$").unwrap();

    for (line_number, line) in reader.lines().map(|line| line.unwrap()).enumerate() {
        if let Some(captures) = re.captures(&line) {
            let (_, [left_version, left_flags, expected_result, right_flags, right_version]) =
                captures.extract();
            cases.push(TestCase {
                line_number: line_number + 1,
                text: line.to_string(),
                left_version: left_version.to_string(),
                left_flags: left_flags.to_string(),
                expected_result: expected_result.to_string(),
                right_flags: right_flags.to_string(),
                right_version: right_version.to_string(),
            });
        }
    }

    return cases;
}
