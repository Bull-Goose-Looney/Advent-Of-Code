use crate::utils::{read_input};
use std::{collections::HashMap, error::Error};


pub fn solve_p1() -> Result<String, Box<dyn Error>> {
    let input = read_input("inputs/day03.txt")?;
    let mut input_split = input.splitn(2, "\n\n");

    let rules_string = input_split.next().unwrap_or("");
    let pages_string = input_split.next().unwrap_or("");

    let rules_map = parse_rules_into_map(rules_string);
    let pages: Vec<i32> = pages_string.split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let count = compare_pages_with_rules(&rules_map, &pages);

    Ok(format!("Result: {}", count))
}

pub fn solve_p2() -> Result<String, Box<dyn Error>> {
    let input = read_input("inputs/day05.txt")?;
    Ok("".to_string())
}


fn compare_pages_with_rules(rules_map: &HashMap<i32, Vec<i32>>, pages: &Vec<i32>) -> i32 {
    0

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