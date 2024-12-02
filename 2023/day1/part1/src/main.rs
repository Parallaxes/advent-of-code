/*
 * Program     : trebuchet
 * Author      : Parallaxis (Parallaxes)
 * Date        : 10-05-2024
 * Version     : 1.0
 * Description : My solution the Part 1 Day 1 of Advent of Code 2023.
*/

// For std I/O and file handling
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// Main function to solve the problem
pub fn solve(input: &str) -> u32 {
    let ans: u32 = input
        // Loop through lines and filter out non-digits
        .lines()
        .map(|line| {
            let mut chars = line.chars().filter(|c| c.is_digit(10));

            // Get the first digit
            let first = chars
                .next()
                .expect("No first digit found");

            // If the last digit is found, use it, otherwise double the first digit
            let num = match chars.last() {
                Some(last) => format!("{}{}", first, last),
                None => format!("{}{}", first, first),
            };
            num.parse::<u32>().unwrap()
        })
        .sum();
    
    // Return the sum
    ans
}
 
fn main() {
    // Initialize the calibration value
    let mut calibration: u32 = 0;

    // Read file contents
    let path = Path::new("input.txt");
    let file = match File::open(&path) {
        Err(why) => {
            println!("Couldn't open {}: {}", path.display(), why);
            return;
        },
        Ok(file) => file,
    };

    // Iterate through each line and add the calculated value to the calibration variable
    let lines = io::BufReader::new(file).lines();
    for line in lines {
        if let Ok(ip) = line {
            calibration += solve(&ip);
            println!("Added value: {}", solve(&ip));
        }
    }

    // Yay! We have the calibration value
    println!("Calibration value: {}", calibration);
}

// :3