/*******************************************************
 * Advent of Code 2024 - Day 2: Red-Nosed Reports - Part 1
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
    let mut safe = 0;

    for line in contents.lines() {
        let mut v = Vec::new();

        for int in line.split_whitespace() {
            v.push(int.parse::<i32>().unwrap());
        }

        if v.len() < 2 {
            continue;
        }

        let mut flag = if v[0] > v[1] {
            "dec"
        } else if v[0] < v[1] {
            "inc"
        } else {
            continue;
        };

        let mut is_safe = true;
        for i in 1..v.len() {
            let diff = (v[i] - v[i - 1]).abs();
            if diff < 1 || diff > 3 {
                is_safe = false;
                break;
            }
            if (flag == "inc" && v[i] <= v[i - 1]) || (flag == "dec" && v[i] >= v[i - 1]) {
                is_safe = false;
                break;
            }
        }

        if is_safe {
            safe += 1;
        }
    }

    safe
}