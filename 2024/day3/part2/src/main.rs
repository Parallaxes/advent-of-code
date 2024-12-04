/*******************************************************
 * Advent of Code 2024 - Day 3: Mull It Over - Part 1
 * -----------------------------------------------------
 * Author      : Parallaxes
 * Date        : 12-03-2024
 * Description : :3
 * Input File  : input.txt
 * Language    : Rust
 * -----------------------------------------------------
 * Notes:
 * - N/A
 *******************************************************/

use std::fs;
use regex::Regex;

// Driver
fn main() {
    // Read the contents of the input file
    let contents = fs::read_to_string("input.txt").expect("Error");
    println!("{}", solve(&contents));
}


fn solve(contents: &str) -> u32 {
    // Create a regex pattern
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();

    // Iterate over the captures and fold the sum
    re.captures_iter(contents)
        // Fold the captures
        // The fold function takes a tuple of (sum, capturing) and the current capture
        .fold((0, true), |(sum, capturing), cap| match &cap[0] {
            // If the capture is "do()", set capturing to true
            "do()" => (sum, true),
            // If the capture is "don't()", set capturing to false
            "don't()" => (sum, false),

            // If the capture is "mul(a, b)", multiply a and b and add it to the sum
            _ if capturing => (
                sum + cap[1].parse::<u32>().unwrap() * cap[2].parse::<u32>().unwrap(),
                capturing,
            ),
            // Otherwise, return the sum and the capturing state
            _ => (sum, capturing),
        })

        // Return the sum
        .0
}

// Took this from online because I give up on regex :\