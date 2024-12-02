/*******************************************************
 * Advent of Code 2015 - Day 4: The Ideal Stocking Stuffer - Part 1
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
use md5::{Md5, Digest};

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Error reading file");
    println!("{}", solve(&contents));
}

fn solve(contents: &str) -> i32 {
    let mut i = 0;
    let key = contents.trim();

    loop {
        let input = format!("{}{}", key, i);
        let mut hasher = Md5::new();

        hasher.update(input);

        let result = hasher.finalize();
        if result[0] == 0 && result[1] == 0 && result[2] < 16 {
            return i;
        }

        i += 1;
    }

    i // SHOULD reach here
}
 