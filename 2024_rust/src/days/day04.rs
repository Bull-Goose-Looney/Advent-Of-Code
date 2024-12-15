use regex::Regex;

use crate::utils::{get_matches_overlap, read_input};
use std::error::Error;


pub fn solve_p1() -> Result<String, Box<dyn Error>> {
    let input: String = read_input("inputs/day04.txt")?;
    let lines: Vec<&str> = input.lines().collect();
    let search_strings: Vec<String> = get_rows_cols_diagonals(&lines);
    let pattern: &str = r"XMAS|SAMX";
    let count: i64 = count_occurences(pattern, &search_strings)?;

	Ok(format!("Result: {}", count))
}

pub fn solve_p2() -> Result<String, Box<dyn Error>> {
    let input = read_input("inputs/day04.txt")?;
    let lines: Vec<&str> = input.lines().collect();
    let count = get_crossed_masses(&lines)?;

	Ok(format!("Result: {}", count))
}

fn get_rows_cols_diagonals(input_lines: &Vec<&str>) -> Vec<String> {
    let max_row = input_lines.len(); // Number of rows
    let max_col = input_lines[0].len(); // Number of columns (assumes all lines are the same length)

    let mut cols = vec![String::new(); max_col];
    let mut rows = vec![String::new(); max_row];
    let mut fdiag = vec![String::new(); max_row + max_col - 1];
    let mut bdiag = vec![String::new(); fdiag.len()];
    let min_bdiag = -(max_row as isize) + 1;

    // Populate rows, columns, and diagonals
    for (y, line) in input_lines.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            cols[x].push(ch); // Append to the appropriate column
            rows[y].push(ch); // Append to the appropriate row
            fdiag[x + y].push(ch); // Append to the forward diagonal
            bdiag[(x as isize - y as isize - min_bdiag) as usize].push(ch); // Append to the backward diagonal
        }
    }
    // Combine all results into a single Vec<String>
    let mut result = Vec::new();
    result.extend(rows);
    result.extend(cols);
    result.extend(fdiag);
    result.extend(bdiag);
    result

}

fn get_crossed_masses(input_lines: &Vec<&str>) -> Result<i32, Box<dyn Error>> {
    let char_matrix = to_char_matrix(&input_lines);
    let mut count = 0;

    let reg = Regex::new(r"MAS|SAM")?;
    for row_index in 1..char_matrix.len()-1 {
        for col_index in 1..char_matrix[row_index].len()-1 {
            let current_char = char_matrix[row_index][col_index];
            if current_char == 'A' {

                let top_left = char_matrix[row_index-1][col_index-1];
                let top_right = char_matrix[row_index-1][col_index + 1];
                let bottom_left = char_matrix[row_index+1][col_index-1];
                let bottom_right = char_matrix[row_index+1][col_index+1];

                let criss: String = [top_left, current_char, bottom_right].into_iter().collect();
                let cross: String = [top_right, current_char, bottom_left].into_iter().collect();

                if reg.is_match(&criss) && reg.is_match(&cross) {
                    count += 1;
                }
            }
        }
    }
    Ok(count)
}

fn to_char_matrix(input: &Vec<&str>) -> Vec<Vec<char>> {
    input.iter()
         .map(|line| line.chars().collect()) // Convert each &str into Vec<char>
         .collect()
}

fn count_occurences(pattern: &str, input: &Vec<String>) -> Result<i64, Box<dyn Error>> {
    let mut count: i64 = 0;
    for search_string in input {
        let mats = get_matches_overlap(&search_string, &pattern)?;
        count += mats.len() as i64;
    }
    Ok(count)
}

#[test]
fn test_get_rows_cols_diagonals() {
    let input = vec![
        "xyz",
        "abc",
        "123"
    ];

    let result = get_rows_cols_diagonals(&input);
    println!("{}", result.len());
    for r in &result {
        println!("{}", r);
    }
    assert!(result.len() == 16);
}

#[test]
fn test_count_occurences() {
    let test_input_1 = vec!["XMASAMX".to_string()];
    let test_input_2 = vec!["SAMXMAS".to_string()];
    let pattern: &str = r"XMAS|SAMX";
    assert!(count_occurences(&pattern, &test_input_1).unwrap() == 2);
    assert!(count_occurences(&pattern, &test_input_2).unwrap() == 2);
}

#[test]
fn test_match_count_accuracy() {
    let input = vec![
        "MMMSXXMASM",
        "MSAMXMSMSA",
        "AMXSXMAAMM",
        "MSAMASMSMX",
        "XMASAMXAMM",
        "XXAMMXXAMA",
        "SMSMSASXSS",
        "SAXAMASAAA",
        "MAMMMXMMMM",
        "MXMXAXMASX"
    ];
    let search_lines = get_rows_cols_diagonals(&input);
    let pattern: &str = r"XMAS|SAMX";
    let number_of_matches = count_occurences(pattern, &search_lines); 
    assert!(number_of_matches.unwrap() == 18);
}


