/*
 * Program     : trebuchet
 * Author      : Parallaxis (Parallaxes)
 * Date        : 10-06-2024
 * Version     : 2.0
 * Description : My solution the Day 1, Part 2 of Advent of Code 2023.
*/

use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

// What are structs anyway?
#[derive(Debug, PartialEq)]
struct NumberPosition {
    number: u32,
    position: usize,
}

// Get the position of digits in the input string
fn find(input: &str) -> Vec<NumberPosition> {
    // To be frank, I don't know what this does
    input
        .chars()
        .enumerate()
        .filter(|(_, c) | c.is_digit(10))
        .map(|(i, c) | NumberPosition {
            number: c.to_digit(10).unwrap(),
            position: i,
        })
        .collect()
}

// Get the position of spelled numbers in the input string
fn location(input: &str) -> Vec<NumberPosition> {
    let spelled_numbers = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut positions: Vec<NumberPosition> = Vec::new();

    // Yeahhhhhh
    for spelled_number in spelled_numbers.clone() {
        let mut start = 0;
        // Get the position of the spelled number in the input string
        while let Some(position) = input[start..].find(spelled_number) {
            // Push it into the positions vector
            positions.push(NumberPosition {
                number: spelled_numbers.iter().position(|&x| x == spelled_number).unwrap() as u32 + 1,
                position: start + position,
            });

            // Calculate start position
            start += position + spelled_number.len();
        }
    }

    // Sort the positions vector by position
    positions.sort_by_key(|x| x.position);
    positions
}

// Helper function to get the number from the input string
fn get_number(line: &str) -> u32 {
    let digits = find(line);
    let spelled_numbers = location(line);
    let mut numbers = Vec::new();

    // Extend the numbers vector with digits and spelled_numbers
    numbers.extend(digits);
    numbers.extend(spelled_numbers);
    numbers.sort_by_key(|x| x.position);

    let first = numbers.first().expect("No first digit found");
    let last = numbers.last().expect("No last digit found");
    let answer = first.number * 10 + last.number;

    answer
}

pub fn solve(input: &str) -> u32 {
    input.lines().map(|line| get_number(line)).sum()
}

fn main() {
    // Initialize the calibration value
    // Error here won't go away, too lazy to fix
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

    // Read the entire file into a string
    let mut contents = String::new();
    if let Err(why) = io::BufReader::new(file).read_to_string(&mut contents) {
        println!("Couldn't read {}: {}", path.display(), why);
        return;
    }

    // Solve the problem using the file contents
    calibration = solve(&contents);

    // Yay! We have the calibration value
    println!("Calibration value: {}", calibration);
}

// :3