/*******************************************************
 * Advent of Code 2023 - Day 4: Scratchcards - Part 1
 * -----------------------------------------------------
 * Author      : Parallaxes
 * Date        : 12-01-2024
 * Description : :3
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

    // Solve the problem and print the result
    println!("{}", solve(&contents));
}

// Solve function
fn solve(contents: &str) -> i32 {
    // Initialize the sum
    let mut sum = 0;

    // For each line in the input file...
    for line in contents.lines() {
        // Create two new hash maps
        let mut v1 = HashMap::new();
        let mut v2 = HashMap::new();

        if line.starts_with("Card ") {
            // Split the line into a vector of integers
            let parts: Vec<&str> = line.split(':').collect();

            // If we have two integers, return them as a tuple.
            if parts.len() == 2 {
                // Split the line into a vector of integers
                let numbers: Vec<&str> = parts[1].split('|').collect();

                // If we have two integers, return them as a tuple.
                if numbers.len() == 2 {

                    // Increment the frequency of the first and second numbers
                    for num in numbers[0].split_whitespace() {
                        let count = v1.entry(num.to_string()).or_insert(0);
                        *count += 1;
                    }
                    for num in numbers[1].split_whitespace() {
                        let count = v2.entry(num.to_string()).or_insert(0);
                        *count += 1;
                    }
                }
            }
        }

        // Initialize the number of matches
        let mut matches = 0;

        // For each key in the first hash map...
        for key in v1.keys() {

            // If the key exists in the second hash map...
            if v2.contains_key(key) {
                matches += 1;
            }
        }

        // Add the number of matches to the sum
        if matches > 0 {
            // The value of the card is computed as 2^(matches - 1)
            sum += 2_i32.pow((matches - 1) as u32);
        }
    }

    // Return the sum
    sum
}