/*******************************************************
 * Advent of Code 2024 - Day 3: Print Queue - Part 1
 * -----------------------------------------------------
 * Author      : Parallaxes
 * Date        : 02-05-2025
 * Description : :3
 * Input File  : input.txt
 * Language    : Rust
 * -----------------------------------------------------
 * Notes:
 * - N/A
 *******************************************************/

use std::fs;
use std::collections::HashMap;

// Driver
fn main() {
    // Read the contents of the input file
    let contents = fs::read_to_string("input.txt").expect("Error");
    println!("{}", solve(&contents));
    //parse_orders(&contents);
}

fn solve(contents: &str) -> i32 {
    let (orders, start) = parse_orders(contents);

    let mut sum = 0;
    for i in start..contents.lines().count() {
        let query = parse_query(contents.lines().nth(i).unwrap());
        if check_order(&orders, &query) {
            let middle_index = query.len() / 2;
            sum += query[middle_index];
        }
    }

    sum
}

fn parse_orders(contents: &str) -> (HashMap<i32, i32>, usize) {
    let mut map = HashMap::new();
    let mut line_number = 0;
    for (i, line) in contents.lines().enumerate() {
        if line.trim().is_empty() {
            line_number = i + 1;
            break;
        }
        let parts: Vec<&str> = line.split("|").collect();
        let key = parts[0].parse().unwrap();
        let value = parts[1].parse().unwrap();
        map.insert(key, value);
    }

    (map, line_number)
}

fn parse_query(query: &str) -> Vec<i32> {
    query.split(',')
        .map(|s| s.trim().parse().unwrap())
        .collect()
}

fn check_order(orders: &HashMap<i32, i32>, query: &Vec<i32>) -> bool {
    for (key, value) in orders {
        let key_index = query.iter().position(|&x| x == *key);
        let value_index = query.iter().position(|&x| x == *value);

        println!("Checking order: key = {}, value = {}", key, value);
        println!("Key index: {:?}, Value index: {:?}", key_index, value_index);

        if let (Some(k_idx), Some(v_idx)) = (key_index, value_index) {
            if k_idx > v_idx {
                println!("Order check failed: key index {} is greater than value index {}", k_idx, v_idx);
                return false;
            }
        }
    }
    println!("Order check passed");
    true
}
