/*******************************************************
 * Advent of Code 2015 - Day 2: I Was Told There Would Be No Math - Part 2
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
    let mut total = 0;
    for line in contents.lines() {
        let dimensions = line.split('x').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let mut dimensions = dimensions;
        dimensions.sort();
        let perimeter = 2 * (dimensions[0] + dimensions[1]);
        let volume = dimensions[0] * dimensions[1] * dimensions[2];

        total += perimeter + volume;
    }

    total
}