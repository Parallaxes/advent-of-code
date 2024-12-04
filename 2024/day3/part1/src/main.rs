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

fn solve(contents: &str) -> i32 {
    let mut sum = 0;
    let pattern = r"mul\(\s*(\d{1,3})\s*,\s*(\d{1,3})\s*\)";
    let re = Regex::new(pattern).unwrap();

    for caps in re.captures_iter(contents) {
        let a: i32 = caps[1].parse().unwrap();
        let b: i32 = caps[2].parse().unwrap();
        sum += a * b;
    }

    sum
}

