use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Error");
    let (v1, v2) = parse(&contents);
    println!("{}", solve(&v1, &v2));
}

fn parse(contents: &str) -> (Vec<i32>, Vec<i32>) {
    let mut v1 = Vec::new();
    let mut v2 = Vec::new();
    for line in contents.lines() {
        if let Some((first, second)) = split(line) {
            v1.push(first);
            v2.push(second);
        }
    }

    v1.sort();
    v2.sort();
    (v1, v2)
}

fn split(line: &str) -> Option<(i32, i32)> {
    let mut nums = line.split_whitespace().map(|s| s.parse::<i32>());
    if let (Some(Ok(first)), Some(Ok(second))) = (nums.next(), nums.next()) {
        Some((first, second))
    } else {
        None
    }
}

fn solve(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
    let mut sum: i32 = 0;
    for i in 0..v1.len() {
        sum += (v1[i] - v2[i]).abs();
    }

    sum
}