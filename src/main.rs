// Entry point, use to call modules (e.g days)
mod days;
mod utils;

use std::env;

fn main() {
    println!("Advent of Code 2024 - Starting the Challenge!");
    println!("Current directory: {:?}", env::current_dir().unwrap());
    
    // Example: Solve for Day 1 part 1
    match days::day01::solve_p1() {
        Ok(result) => println!("Day 1, Part 1 Result: {}", result),
        Err(e) => println!("Error solving Day 1, Part 1: {}", e),
    }

    // Example: Solve for Day 1 part 2
    match days::day01::solve_p2() {
        Ok(result) => println!("Day 1, Part 2 Result: {}", result),
        Err(e) => println!("Error solving Day l, Part 2: {}", e),
    }

    // Example: Solve for Day 1 part 2
    match days::day02::solve_p1() {
        Ok(result) => println!("Day 2, Part 1 Result: {}", result),
        Err(e) => println!("Error solving Day 2, Part 1: {}", e),
    }



    // Example: Solve for Day 2 (if implemented)
    // match days::day02::solve() {
    //     Ok(result) => println!("Day 2 Result: {}", result),
    //     Err(e) => println!("Error solving Day 2: {}", e),
    // }
}