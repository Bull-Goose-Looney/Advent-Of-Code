use crate::utils::read_input;
use regex::Regex;
use std::error::Error;


pub fn solve_p1() -> Result<String, Box<dyn Error>> {
    let input = read_input("inputs/day04.txt")?;
    let mut count = 0;

    let lines = input.lines().collect();

    for line in lines {

    }

    let line_matrix = get_line_matrix(lines);
    let diagonal_lines = get_diagonal_lines(&line_matrix);
    let vertical_lines = get_vertical_lines(&line_matrix);

    let pattern = r"XMAS|SAMX";
    // count = count_occurences(pattern, all_lines);

	Ok(format!("Result: {}", count))
}

pub fn solve_p2() -> Result<String, Box<dyn Error>> {
    let input = read_input("inputs/day04.txt")?;
    let mut count = 0;
	Ok(format!("Result: {}", count))
}

fn get_line_matrix(input: Vec<&str>) -> Vec<Vec<char>> {

    let mut matrix: Vec<Vec<char>> = Vec::new();

    for v in input {
        let &mut chars = v.chars().collect();
        matrix.append(chars);
    }

    matrix
}


fn get_diagonal_lines(input_lines: &Vec<Vec<&str>>) -> Vec<&str> {
    let max_col = input_lines.len();
    let max_row = input_lines[0].len();

    let mut cols = vec![Vec::new(); max_col];
    let mut rows = vec![Vec::new(); max_row];
    let mut fdiag = vec![Vec::new(); max_row + max_col - 1];
    let mut bdiag = vec![Vec::new(); fdiag.len()];
}

fn get_vertical_lines(input_lines: &Vec<Vec<&str>>) -> Vec<&str>{
    Vec::new()
}

fn  count_occurences(pattern: &str, input: Vec<&str>) -> i64 {

}

// #[test]
// fn test_get_matches() {
//     let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
//     let pattern = r"mul\(\d{0,3}\,\d{0,3}\)";

//     assert!(get_matches(input, pattern).unwrap() == vec!["mul(2,4)", "mul(5,5)", "mul(11,8)", "mul(8,5)"]);
// }

