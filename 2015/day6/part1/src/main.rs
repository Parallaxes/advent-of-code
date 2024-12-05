/*******************************************************
 * Advent of Code 2015 - Day 6: Probably a Fire Hazard - Part 1
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
    let mut grid = [[false; 1000]; 1000];
    let mut cnt = 0;

    for line in contents.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let (action, start, end) = if parts[0] == "toggle" {
            ("toggle", parts[1], parts[3])
        } else {
            (parts[1], parts[2], parts[4])
        };

        let start_coords: Vec<usize> = start.split(',').map(|x| x.parse().unwrap()).collect();
        let end_coords: Vec<usize> = end.split(',').map(|x| x.parse().unwrap()).collect();

        for x in start_coords[0]..=end_coords[0] {
            for y in start_coords[1]..=end_coords[1] {
            match action {
                "on" => {
                if !grid[x][y] {
                    grid[x][y] = true;
                    cnt += 1;
                }
                }
                "off" => {
                if grid[x][y] {
                    grid[x][y] = false;
                    cnt -= 1;
                }
                }
                "toggle" => {
                if grid[x][y] {
                    grid[x][y] = false;
                    cnt -= 1;
                } else {
                    grid[x][y] = true;
                    cnt += 1;
                }
                }
                _ => {}
            }
            }
        }
    }

    cnt
}