/*******************************************************
 * Advent of Code 2024 - Day 1: Historian Hysteria - Part 1
 * -----------------------------------------------------
 * Author      : Parallaxes
 * Date        : 12-01-2024
 * Description : Very simple approach where we parse the
 *               input file and store the numbers in two
 *               separate vectors. We then sort the 
 *               vectors and calculate the sum of the 
 *               absolute differences.
 * Input File  : input.txt
 * Language    : Rust
 * -----------------------------------------------------
 * Notes:
 * - N/A
 *******************************************************/

use std::fs;

// Driver
fn main() {
    // Read the contents of the input file
    let contents = fs::read_to_string("input.txt").expect("Error");

    // Parse the contents and store the two vectors
    let (v1, v2) = parse(&contents);

    // Solve the problem and print the result
    println!("{}", solve(&v1, &v2));
}

// Parse function
fn parse(contents: &str) -> (Vec<i32>, Vec<i32>) {
    // Create two new vectors
    let mut v1 = Vec::new();
    let mut v2 = Vec::new();

    // For each line in the input file...
    for line in contents.lines() {
        // Split the line into a tuple of integers
        if let Some((first, second)) = split(line) {
            v1.push(first);
            v2.push(second);
        }
    }

    // Sort the vectors
    v1.sort();
    v2.sort();

    // Return the two vectors
    (v1, v2)
}

// Helper split function
fn split(line: &str) -> Option<(i32, i32)> {
    // Split the line into a vector of integers
    let mut nums = line.split_whitespace().map(|s| s.parse::<i32>());

    // If we have two integers, return them as a tuple. Data safety!
    if let (Some(Ok(first)), Some(Ok(second))) = (nums.next(), nums.next()) {
        Some((first, second))
    } else {
        None
    }
}

// Solve function
fn solve(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
    // Initialize the sum
    let mut sum: i32 = 0;

    // For each index in the vectors...
    for i in 0..v1.len() {
        // Add the absolute difference of the two numbers to the sum
        sum += (v1[i] - v2[i]).abs();
    }

    // Return the sum
    sum
}