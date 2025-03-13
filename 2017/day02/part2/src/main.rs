use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Error reading file");
    solve(&contents);
}

fn solve(contents: &str) {
    let mut checksum = 0;
    for line in contents.lines() {
        let vec: Vec<u32> = line.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect();
        for i in 0..vec.len() {
            for j in 0..vec.len() {
                if vec[j] != 0 && vec[i] % vec[j] == 0 && vec[i] != vec[j] {
                    println!("Adding: {}, {}", vec[i], vec[j]);
                    checksum += vec[i] / vec[j];
                }
            }
        }
    }
    println!("{}", checksum);
}