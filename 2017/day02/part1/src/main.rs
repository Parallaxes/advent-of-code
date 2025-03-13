use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Error reading file");
    solve(&contents);
}

fn solve(contents: &str) {
    let mut checksum = 0;
    for line in contents.lines() {
        let mut vec: Vec<u32> = line.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect();
        vec.sort_unstable();
        checksum += vec[vec.len() - 1] - vec[0];
    }
    println!("{}", checksum);
}