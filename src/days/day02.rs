use crate::utils::read_input;
use std::error::Error;

pub fn solve_p1() -> Result<String, Box<dyn Error>> {
    let file_path = "inputs/day02.txt";
    let string_data = read_input(&file_path)?;

    let levels = string_data.lines();
    
    let mut safe_count = 0;
    for level in levels {
        if level_is_safe(&level) {
            safe_count += 1;
        }
    }
	Ok(format!("Result: {}", safe_count))
}

fn level_is_safe(level: &str) -> bool {
    let number_list = convert_line_to_numbers(level);
    let mut increasing: Option<bool> = None;
    let mut prev: Option<i32> = None;

    for num in number_list {
        if prev.is_none() {
            prev = Some(num);
            continue;
        }

        if increasing.is_none() {
            increasing = Some(num > prev.unwrap())
        }

        if(num - prev.unwrap()).abs() > 3 {
            return false;
        }

        let safe = match num {
            n if n > prev.unwrap() && increasing.unwrap() => true, 
            n if n < prev.unwrap() && !increasing.unwrap() => true,
            _ => false
        };

        if !safe {
            print!("Returning false\n");
            return false
        }
        prev = Some(num);
    }

    return true
}

fn convert_line_to_numbers(line: &str) -> Vec<i32> {
    line
        .split_whitespace()
        .filter_map(|s| s.parse::<i64>().ok())
        .filter_map(|n| n.try_into().ok())
        .collect()
}

#[test]
fn test_level_is_safe() {
    assert!(level_is_safe("1 2 3 4 5"));
    assert!(level_is_safe("5 4 3 2 1"));
    assert!(level_is_safe("7 6 4 2 1"));
    assert!(level_is_safe("1 3 6 7 9"));
}

#[test]
fn test_level_is_unsafe() {
    assert!(!level_is_safe("9 7 6 2 1"));
    assert!(!level_is_safe("1 3 2 4 5"));
    assert!(!level_is_safe("8 6 4 4 1"));
}
