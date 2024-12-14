use aoc_2024::days::day03;

#[test]
fn test_day03_p1_solution() {
    let result = day03::solve_p1();
    assert!(result.is_ok(), "Day 3 solution returned an error");
    assert_eq!(result.unwrap(), "Result: 171183089"); // Adjust expected result as necessary
}

#[test]
fn test_day03_p2_solution() {
    let result = day03::solve_p2();
    assert!(result.is_ok(), "Day 3 solution returned an error");
    assert_eq!(result.unwrap(), "Result: 63866497"); // Adjust expected result as necessary
}