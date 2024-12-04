/*******************************************************
 * Advent of Code 2024 - Day 2: Red-Nosed Reports - Part 2
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

// Driver
fn main() {
    // Read the contents of the input file
    let contents = fs::read_to_string("input.txt").expect("Error");
    println!("{}", solve(&contents));
}

fn solve(contents: &str) -> i32 {
    // Counter for safe sequences
    let mut safe = 0;

    // For line in contents...
    for line in contents.lines() {
        // Parse integers from the line
        let mut v: Vec<i32> = line
            .split_whitespace()
            .map(|int| int.parse::<i32>().unwrap())
            .collect();

        // Skip if the sequence is too short
        if v.len() < 2 {
            continue;
        }

        // Check if the sequence is already safe
        if is_safe(&v) {
            safe += 1;
            continue;
        }

        // Try removing each element to see if it becomes safe
        for i in 0..v.len() {
            let mut v_copy = v.clone();
            v_copy.remove(i);

            // If the sequence is safe, increment the counter and break
            if is_safe(&v_copy) {
                safe += 1;
                break;
            }
        }
    }

    safe
}

// Helper function to check if a sequence is safe
fn is_safe(v: &Vec<i32>) -> bool {
    // If the sequence is too short, return false
    if v.len() < 2 {
        return false;
    }

    // Check if the sequence is strictly increasing or decreasing
    let flag = if v[0] > v[1] {
        "dec"
    } else if v[0] < v[1] {
        "inc"
    } else {
        return false; // Not strictly increasing or decreasing
    };

    // For each element in the sequence...
    for i in 1..v.len() {
        // Calculate the difference between the current and previous element
        let diff = (v[i] - v[i - 1]).abs();

        // If the difference is not between 1 and 3, or the sequence is not strictly increasing or decreasing, return false
        if diff < 1 || diff > 3 || (flag == "inc" && v[i] <= v[i - 1])
            || (flag == "dec" && v[i] >= v[i - 1])
        {
            return false;
        }
    }

    // If all checks pass, return true
    true
}
