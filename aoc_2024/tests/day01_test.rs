use aoc_2024::days::day01;

#[test]
fn test_day01_p1_solution() {
    let result = day01::solve_p1();
    assert!(result.is_ok(), "Day 1 solution returned an error");
    assert_eq!(result.unwrap(), "Result: 2192892"); // Adjust expected result as necessary
}

#[test]
fn test_day01_p2_solution() {
    let result = day01::solve_p2();
    assert!(result.is_ok(), "Day 1 solution returned an error");
    assert_eq!(result.unwrap(), "Result: 22962826"); // Adjust expected result as necessary
}