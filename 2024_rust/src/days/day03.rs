use crate::utils::{read_input, get_matches};
use regex::Regex;
use std::error::Error;


pub fn solve_p1() -> Result<String, Box<dyn Error>> {
    let input = read_input("inputs/day03.txt")?;
    let pattern = r"mul\(\d{0,3}\,\d{0,3}\)";

    let matches: Vec<&str> = get_matches(&input, pattern)?;
    for mat in &matches {
        println!("Match: {}", mat);
    }

    let mut count = 0;
    for mat in matches {
        if let Some((n, m)) = extract_values(mat) {
            count += n * m;
        };
    }
	Ok(format!("Result: {}", count))
}

pub fn solve_p2() -> Result<String, Box<dyn Error>> {
    let input = read_input("inputs/day03.txt")?;
    let pattern = r"(do\(\)|don\'t\(\))|(mul\(\d{0,3}\,\d{0,3}\))";
    let matches: Vec<&str> = get_matches(&input, pattern)?;

    let mut count = 0;
    let mut enabled = true;

    for mat in matches {
        if mat == "do()" {
            enabled = true;
        } else if mat == "don't()" {
            enabled = false;
        }

        if enabled {
            if let Some((n, m)) = extract_values(mat) {
                count += n * m;
            };
        }
    }

	Ok(format!("Result: {}", count))
}

fn extract_values(statement: &str) -> Option<(i32, i32)> {
    let reg = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    reg.captures(statement).and_then(|caps| {
        let n1 = caps.get(1)?.as_str().parse::<i32>().ok()?;
        let n2 = caps.get(2)?.as_str().parse::<i32>().ok()?;
        println!("n1: {}, n2: {}", n1, n2);
        Some((n1, n2))
    })
}

#[test]
fn test_get_matches_groups() {
    let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    let pattern = r"(do\(\)|don\'t\(\))|(mul\(\d{0,3}\,\d{0,3}\))";

    assert!(get_matches(input, pattern).unwrap() == vec!["mul(2,4)","don't()", "mul(5,5)", "mul(11,8)", "do()", "mul(8,5)"]);
}

#[test]
fn test_extract_values() {
    assert!(extract_values("mul(2,4)").unwrap() == (2, 4));
    assert!(extract_values("mul(22,33)").unwrap() == (22, 33));
    assert!(extract_values("mul(222,333)").unwrap() == (222, 333));
}
