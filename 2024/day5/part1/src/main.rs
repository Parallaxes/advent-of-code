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
        let mut key_indices = vec![];
        let mut value_indices = vec![];

        for (i, &num) in query.iter().enumerate() {
            if num == *key {
                key_indices.push(i);
            } else if num == *value {
                value_indices.push(i);
            }
        }

        println!("Checking order for key: {}, value: {}", key, value);
        println!("Key indices: {:?}", key_indices);
        println!("Value indices: {:?}", value_indices);

        for &key_idx in &key_indices {
            for &value_idx in &value_indices {
                if key_idx > value_idx {
                    println!("Order check failed: key index {} is greater than value index {}", key_idx, value_idx);
                    return false;
                }
            }
        }

        for &value_idx in &value_indices {
            for &key_idx in &key_indices {
                if value_idx > key_idx {
                    println!("Order check failed: value index {} is greater than key index {}", value_idx, key_idx);
                    return false;
                }
            }
        }
    }
    println!("Order check passed");
    true
}
