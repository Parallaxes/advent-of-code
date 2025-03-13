use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Error reading file");
    println!("{}", solve(&contents));
}

fn solve(contents: &str) -> i32 {
    let mut sum = 0;
    let chars: Vec<char> = contents.chars().collect();
    let len = chars.len();
    for i in 0..len {
        if chars[i] == chars[(i + len / 2) % len] {
            sum += chars[i].to_digit(10).unwrap_or(0);
        }
    }
    sum as i32
}
