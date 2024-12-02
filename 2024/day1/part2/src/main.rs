/*******************************************************
 * Advent of Code 2024 - Day 1: Historian Hysteria - Part 2
 * -----------------------------------------------------
 * Author      : Parallaxes
 * Date        : 12-01-2024
 * Description : Basically a simple approach where we
 *               parse the input file and store the
 *               frequencies of the first and second
 *               numbers in two separate hashmaps.
 * Input File  : input.txt
 * Language    : Rust
 * -----------------------------------------------------
 * Notes:
 * - N/A
 *******************************************************/

use std::fs;
use std::collections::HashMap;

// Driver
fn main() {
    // Read the contents of the input file
    let contents = fs::read_to_string("input.txt").expect("Error");

    // Parse the contents and store the two hash maps
    let (v1, v2) = parse(&contents);

    // Solve the problem and print the result
    println!("{}", solve(&v1, &v2));
}

// Parse function
fn parse(contents: &str) -> (HashMap<i32, i32>, HashMap<i32, i32>) {
    // Create two new hash maps
    let mut v1 = HashMap::new();
    let mut v2 = HashMap::new();
    
    // For each line in the input file...
    for line in contents.lines() {
        // Split the line into a vector of integers
        let nums: Vec<i32> = line
            .split_whitespace()
            .filter_map(|num| num.parse().ok())
            .collect();
    
        // Increment the frequency of the first and second numbers
        // Just assume that the data is clean and there are always two numbers
        *v1.entry(nums[0]).or_insert(0) += 1;
        *v2.entry(nums[1]).or_insert(0) += 1;
    }

    // Return the two hash maps
    (v1, v2)
}

// Solve function
fn solve(v1: &HashMap<i32, i32>, v2: &HashMap<i32, i32>) -> i32 {
    // Initialize the sum
    let mut sum: i32 = 0;

    // For each key-value pair in the first hash map...
    for (key, value) in v1 {
        // If the key exists in the second hash map...
        if let Some(second_freq) = v2.get(key) {
            // Add the product of the key and the frequency of the second number to the sum
            sum += key * second_freq;
        }
    }

    // Return the sum
    sum
}
