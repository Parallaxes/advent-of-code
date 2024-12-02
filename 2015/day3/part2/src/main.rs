/*******************************************************
 * Advent of Code 2015 - Day 3: Perfectly Spherical Houses in a Vacuum - Part 1
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
use std::collections::HashSet;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Error");
    println!("{}", solve(&contents));
}

fn solve(contents: &str) -> usize {
    let mut visited = HashSet::new();
    let (mut x, mut y) = (0, 0);
    let (mut rx, mut ry) = (0, 0);
    
    visited.insert((x, y)); // Include the starting point
    visited.insert((rx, ry)); // Include the starting point

    // Alternate the movements between the two
    for (i, c) in contents.chars().enumerate() {
        if i % 2 == 0 {
            match c {
                '^' => y += 1,
                'v' => y -= 1,
                '>' => x += 1,
                '<' => x -= 1,
                _ => (),
            }
            visited.insert((x, y));
        } else {
            match c {
                '^' => ry += 1,
                'v' => ry -= 1,
                '>' => rx += 1,
                '<' => rx -= 1,
                _ => (),
            }
            visited.insert((rx, ry));
        }
    }

    visited.len()
}