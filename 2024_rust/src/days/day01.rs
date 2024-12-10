use crate::utils::read_input;
use std::collections::HashMap;

use std::error::Error;

// Figure out how often each number from the left list appears in the right list.
pub fn solve_p2() -> Result<String, Box<dyn std::error::Error>> {
	let mut map: HashMap<i32, i32> = HashMap::new();
	let string_data = get_input_as_string()?;

	let(left_side, right_side) = split_sides_parse_and_sort(&string_data)?;
	for i in &left_side {
		for j in &right_side {
			if i == j {
				*map.entry(*i).or_insert(0) += 1;
			}
		}
	}

	let mut product_sum = 0;
	for (&key, &value) in map.iter() {
		product_sum += key * value;
	}

	Ok(format!("Result: {}", product_sum))
}

// Get difference between sorted coulmn elements
pub fn solve_p1() -> Result<String, Box<dyn std::error::Error>> {
	let input_data = get_input_as_string()?;
	let(left_side, right_side) = split_sides_parse_and_sort(&input_data)?;

	let mut running_difference = 0;
	for (l, r) in left_side.iter().zip(right_side.iter()) {
		running_difference += (l - r).abs();
	};

	Ok(format!("Result: {}", running_difference))
}

fn get_input_as_string() -> Result<String, Box<dyn std::error::Error>>{
	let input_path = "inputs/day01.txt";
	let input_data = match read_input(input_path) {
		Ok(data) => data,
		Err(e) => return Err(e),
	};
	Ok(input_data)
}

fn split_sides_parse_and_sort(data: &str) -> Result<(Vec<i32>, Vec<i32>), Box<dyn Error>> {
	let mut left_side: Vec<i32> = Vec::new(); 
	let mut right_side: Vec<i32> = Vec::new();
	let lines = data.lines(); 

	for line in lines {
		let parts: Vec<&str> = line.split_whitespace().collect();
		left_side.push(parts[0].parse()?);
		right_side.push(parts[1].parse()?);
	}
	left_side.sort();
	right_side.sort();

	Ok((left_side, right_side))
}
