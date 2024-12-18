use aoc_2024::days::day02;

#[test]
fn test_day02_p1_solution() {
    let result = day02::solve_p1();
    assert!(result.is_ok(), "Day 1 solution returned an error");
    assert_eq!(result.unwrap(), "Result: 334"); // Adjust expected result as necessary
}

#[test]
fn test_day02_p2_solution() {
    let result = day02::solve_p2();
    assert!(result.is_ok(), "Day 1 solution returned an error");
    assert_eq!(result.unwrap(), "Result: 400"); // Adjust expected result as necessary
}