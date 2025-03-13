use std::fs;
use std::collections::HashMap;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Error reading file");
    solve(&contents);
}

fn solve(contents: &str) {
    let mut map: HashMap<&str, u32> = HashMap::new();
    let mut valid = 0;
    for line in contents.lines() {
        map.clear();
        let vec: Vec<&str> = line.split_whitespace().collect();
        for s in &vec {
            if let Some(count) = map.get(s) {
                map.insert(s, count + 1);
            } else {
                map.insert(s, 1);
            };
        }
        if map.keys().len() == vec.len() {
            valid += 1;
        }
    }
    println!("{}", valid);
}