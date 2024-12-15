use std::fs;
use std::path::Path;

use regex::Regex;

pub fn read_input(file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    if !Path::new(file_path).exists() {
        return Err(format!("Input file not found: {}", file_path).into());
    }
    let content = fs::read_to_string(file_path)?;
    Ok(content)
}

pub fn get_matches<'a>(input: &'a str, pattern: &'a str) -> Result<Vec<&'a str>, regex::Error> { 
    let reg = Regex::new(pattern)?;
    Ok(reg.find_iter(input)
        .map(|mat| mat.as_str())
        .collect())
} 

// Loop one character at a time, regexing remaining string each time
pub fn get_matches_overlap<'a>(input: &'a str, pattern: &str) -> Result<Vec<&'a str>, regex::Error> {
    let reg = Regex::new(pattern)?;
    let mut matches = Vec::new();
    let mut start = 0;

    while start < input.len() {
        if let Some(mat) = reg.find(&input[start..]) {
            let match_start = start + mat.start();
            let match_end = start + mat.end();
            matches.push(&input[match_start..match_end]);
            start = match_start + 1; // Move forward by 1 character to allow overlap
        } else {
            break;
        }
    }

    Ok(matches)
}

#[test]
fn test_get_matches() {
    let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    let pattern = r"mul\(\d{0,3}\,\d{0,3}\)";

    assert!(get_matches(input, pattern).unwrap() == vec!["mul(2,4)", "mul(5,5)", "mul(11,8)", "mul(8,5)"]);
}

#[test]
fn test_get_matches_overlap() {
    let input = "XMASAMX";
    let pattern = r"XMAS|SAMX";
    assert!(get_matches_overlap(input, pattern).unwrap() == vec!["XMAS", "SAMX"]);
}

