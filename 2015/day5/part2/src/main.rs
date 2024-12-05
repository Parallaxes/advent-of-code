/*******************************************************
 * Advent of Code 2015 - Day 5: Doesn't He Have Intern-Elves For This? - Part 2
 * -----------------------------------------------------
 * Author      : Parallaxes
 * Date        : 12-04-2024
 * Description : :3
 * Input File  : input.txt
 * Language    : Rust
 * -----------------------------------------------------
 * Notes:
 * - N/A
 *******************************************************/

use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Error reading file");
    println!("{}", solve(&contents));
}

fn solve(contents: &str) -> i32 {
    let mut cnt = 0;

    for line in contents.lines() {
        let repeated_seperated = line
            .trim()
            .chars()
            .zip(line.chars().skip(2))
            .any(|(a, b)| a == b);

        if two_pairs(line) && repeated_seperated {
            cnt += 1;
        }
    }

    cnt
}

fn two_pairs(s: &str) -> bool {
    if s.len() < 4 {
        return false;
    }

    let pair = &s[0..2];
    let remain = &s[2..];

    remain.contains(pair) || two_pairs(&s[1..])
}