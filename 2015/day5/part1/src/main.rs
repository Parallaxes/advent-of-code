/*******************************************************
 * Advent of Code 2015 - Day 5: Doesn't He Have Intern-Elves For This? - Part 1
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
    let mut nice = 0;
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let forbidden_words = ["ab", "cd", "pq", "xy"];

    for line in contents.lines() {
        let mut vowel_count = 0;
        let mut double = false;
        let mut forbidden = false;

        for (i, c) in line.chars().enumerate() {
            if vowels.contains(&c) {
                vowel_count += 1;
            }

            if i > 0 {
                let prev = line.chars().nth(i - 1).unwrap();
                if prev == c {
                    double = true;
                }

                let pair = format!("{}{}", prev, c);
                if forbidden_words.contains(&pair.as_str()) {
                    forbidden = true;
                }
            }
        }

        if vowel_count >= 3 && double && !forbidden {
            nice += 1;
        }
    }

    nice
}