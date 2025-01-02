use crate::utils::{read_input};
use std::{collections::HashMap, error::Error};


pub fn solve_p1() -> Result<String, Box<dyn Error>> {
	let input = read_input("inputs/day05.txt")?;
	let mut input_split = input.splitn(2, "\n\n");

	let rules_string = input_split.next().unwrap_or("");
	let pages_string = input_split.next().unwrap_or("");

	let updates = parse_pages(pages_string);
	let rules_map = parse_rules_into_map(rules_string);

	let count = compare_pages_with_rules_1(&rules_map, &updates);

	Ok(format!("Result: {}", count))
}

pub fn solve_p2() -> Result<String, Box<dyn Error>> {
	let input = read_input("inputs/day05.txt")?;
	Ok("".to_string())
}


fn compare_pages_with_rules_1(rules_map: &HashMap<i32, Vec<i32>>, updates: &Vec<Vec<i32>>) -> i32 {
	let mut count = 0;
	for update in updates {
		for page in update {
			if let Some(rules) = rules_map.get(page) {
				if follows_rules(rules, *page, update) {
					count+=1
				}
			}
		}
	}
	-1

}

fn follows_rules(rules: &Vec<i32>, page: i32, update: &Vec<i32>) -> bool {
	true
}

fn parse_rules_into_map(rules_string: &str) -> HashMap<i32, Vec<i32>> {
	let mut rules_map: HashMap<i32, Vec<i32>> = HashMap::new();
	let rules = rules_string.lines();

	for rule in rules {
		if let Some((key_str, value_str)) = rule.split_once("|") {
			let key: i32 =  match key_str.parse::<i32>() {
				Ok(number) => number,
				Err(_) => panic!("Error parsing key: {}", key_str),
			};
			let value: i32 =  match value_str.parse::<i32>() {
				Ok(number) => number,
				Err(_) => panic!("Error parsing value: {}", value_str),
			};
			rules_map.entry(key).or_insert(Vec::new()).push(value);
		} else {
			println!("Error parsing rule {}", rule);
		};
	}
	rules_map
}

fn parse_pages(pages_string: &str) -> Vec<Vec<i32>> {
	let mut page_set: Vec<Vec<i32>> = Vec::new();
	let pages_string = pages_string.lines();

	for line in pages_string {
		let page_list: Vec<i32> = line
			.split(',')
			.map(|s| s.trim())
			.filter_map(|s| s.parse::<i32>().ok())
			.collect();
		page_set.push(page_list);
	}
	page_set
}

#[test]
fn test_parse_rules() {
	// test
   let test_rules = "34|82\n33|99\n4|5\n4|6\n4|6"; 
   let map = parse_rules_into_map(test_rules);
   assert!(map.len() == 3);
   assert!(map.contains_key(&33));
   assert!(map.contains_key(&34));
   assert!(map.contains_key(&4));

   assert!(map.get(&4).unwrap() == &vec![5, 6, 6]);
}

#[test]
fn test_parse_pages() {
	let test_pages = "33,44,44,43,22,33\n22,11,433,4,45\n1,22,333,4444";
	let page_set = parse_pages(&test_pages);

	assert!(page_set.len() == 3);
	assert!(page_set[0] == vec![33, 44, 44, 43, 22, 33]);
}