// Entry point, use to call modules (e.g days)
mod days;
mod utils;

use std::env;

fn main() {
    println!("Advent of Code 2024 - Starting the Challenge!");
    println!("Current directory: {:?}", env::current_dir().unwrap());
    
    match days::day01::solve_p1() {
        Ok(result) => println!("Day 1, Part 1 {}", result),
        Err(e) => println!("Error solving Day 1, Part 1: {}", e),
    }

    match days::day01::solve_p2() {
        Ok(result) => println!("Day 1, Part 2 {}", result),
        Err(e) => println!("Error solving Day l, Part 2: {}", e),
    }

    match days::day02::solve_p1() {
        Ok(result) => println!("Day 2, Part 1 {}", result),
        Err(e) => println!("Error solving Day 2, Part 1: {}", e),
    }

    match days::day02::solve_p2() {
        Ok(result) => println!("Day 2, Part 2 {}", result),
        Err(e) => println!("Error solving Day 2, Part 2: {}", e),
    }

    match days::day03::solve_p1() {
        Ok(result) => println!("Day 3, Part 1 {}", result),
        Err(e) => println!("Error solving Day 3, Part 1: {}", e),
    }

    match days::day03::solve_p2() {
        Ok(result) => println!("Day 3, Part 2 {}", result),
        Err(e) => println!("Error solving Day 3, Part 2: {}", e),
    }

    match days::day04::solve_p1() {
        Ok(result) => println!("Day 4, Part 1 {}", result),
        Err(e) => println!("Error solving Day 4, Part 1: {}", e),
    }

    match days::day04::solve_p2() {
        Ok(result) => println!("Day 4, Part 2 {}", result),
        Err(e) => println!("Error solving Day 4, Part 2: {}", e),
    }

    match days::day05::solve_p1() {
        Ok(result) => println!("Day 5, Part 1 {}", result),
        Err(e) => println!("Error solving Day 5, Part 1: {}", e),
    }


}