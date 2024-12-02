/*******************************************************
 * Advent of Code 2015 - Day 1: Not Quite Lisp - Part 2
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

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Error");
    println!("{}", solve(&contents));
}

fn solve(contents: &str) -> i32 {
    let mut floor: i32 = 0;
    let mut position: i32 = 0;
    for c in contents.chars() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => (),
        }

        position += 1;

        if floor == -1 { 
            break;
        }
    }

    position
}