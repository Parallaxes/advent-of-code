use std::fs;
use std::collections::HashMap;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Error reading file");
    solve(&contents);
}

fn solve(contents: &str)