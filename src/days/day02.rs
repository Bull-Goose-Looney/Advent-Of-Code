use crate::utils::read_input;
use std::error::Error;

pub fn solve_p1() -> Result<String, Box<dyn Error>> {
    let file_path = "inputs/day02.txt";
    let string_data = read_input(&file_path)?;

    let levels = string_data.lines();
    
    let mut safe_count = 0;
    for level_string in levels {
        let level= convert_line_to_numbers(level_string);
        if level_is_safe(&level) {
            safe_count += 1;
        }
    }
	Ok(format!("Result: {}", safe_count))
}

pub fn solve_p2() -> Result<String, Box<dyn Error>> {
    let file_path = "inputs/day02.txt";
    let string_data = read_input(&file_path)?;

    let levels = string_data.lines();
    
    let mut safe_count = 0;
    for level_string in levels {
        let level = convert_line_to_numbers(level_string);
        if level_is_safe(&level) || level_is_savable(&level){
            safe_count += 1;
        }
    }
	Ok(format!("Result: {}", safe_count))
}

fn level_is_safe(level: &Vec<i32>) -> bool {
    let mut increasing: Option<bool> = None;
    let mut prev: Option<i32> = None;

    for num in level {
        if prev.is_none() {
            prev = Some(*num);
            continue;
        }

        if increasing.is_none() {
            increasing = Some(*num > prev.unwrap())
        }

        if(num - prev.unwrap()).abs() > 3 {
            return false;
        }

        let safe = match num {
            n if *n > prev.unwrap() && increasing.unwrap() => true, 
            n if *n < prev.unwrap() && !increasing.unwrap() => true,
            _ => false
        };

        if !safe {
            return false
        }
        prev = Some(*num);
    }

    return true
}

fn level_is_savable(level: &Vec<i32>) -> bool {
    let mut savable = false;
    // Loop over the string, removing one of the values each time, 
    let permutations: Vec<Vec<i32>> = (0..level.len())
        .map(|i| {
            let mut new_vec = level.clone();
            new_vec.remove(i); // Remove the element at index `i`
            new_vec
        })
        .collect();

    for p in &permutations {
        if level_is_safe(&p) {
            savable = true
        }
    }
    savable
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
    assert!(level_is_safe(&vec![1, 2, 3, 4, 5]));
    assert!(level_is_safe(&vec![5, 4, 3, 2, 1]));
    assert!(level_is_safe(&vec![7, 6, 4, 2, 1]));
    assert!(level_is_safe(&vec![1, 3, 6, 7, 9]));
}

#[test]
fn test_level_is_unsafe() {
    assert!(!level_is_safe(&vec![9, 7, 6, 2, 1]));
    assert!(!level_is_safe(&vec![1, 3, 2, 4, 1]));
    assert!(!level_is_safe(&vec![8, 6, 4, 4, 1]));
}

#[test]
fn test_level_is_savable() {
    assert!(level_is_savable(&vec![1, 3, 2, 4, 5]));
    assert!(level_is_savable(&vec![8, 6, 4, 4, 1]));
}

#[test]
fn test_level_is_unsavable() {
    assert!(!level_is_savable(&vec![1, 2, 7, 8, 9]));
    assert!(!level_is_savable(&vec![9, 7, 6, 2, 1]));
}
