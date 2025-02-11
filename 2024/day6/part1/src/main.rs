use std::fs;
use std::collections::HashSet;

// Driver
fn main() {
    // Read the contents of the input file
    let contents = fs::read_to_string("input.txt").expect("Error");
    println!("{}", solve(parse(&contents)));
}

// Parsing 
fn parse(contents: &str) -> Vec<Vec<char>> {
    let mut vec: Vec<Vec<char>> = Vec::new();
    for line in contents.lines() {
        let chars: Vec<char> = line.chars().collect();
        vec.push(chars);
    }
    vec
}

// Actual computation
fn solve(grid: Vec<Vec<char>>) -> i32 {
    // Initialize positions
    let mut position = (0, 0);

    // Who needs error handling anyway??
    if grid.is_empty() || grid[0].is_empty() {
        return 0;
    }

    // Just some debugging stuff
    let grid_size = (grid.len() as i32, grid[0].len() as i32);

    // Get initial position
    for (i, row) in grid.iter().enumerate() {
        if let Some(j) = row.iter().position(|&c| c == '^') {
            position = (i as i32, j as i32);
            break;
        }
    }

    // Directions!
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)]; // Right, down, left, up
    let mut direction_index = 3; // Starting direction is up
    let mut visited_positions = HashSet::new();
    visited_positions.insert(position); // Count the starting position!!

    // Until the guard goes out of bounds...
    while position.0 >= 0 && position.0 < grid_size.0 && position.1 >= 0 && position.1 < grid_size.1 {
        // See what the next position would be
        let next_position = (
            position.0 + directions[direction_index].0,
            position.1 + directions[direction_index].1,
        );

        // If the next position is within bounds, travel
        if next_position.0 >= 0 && next_position.0 < grid_size.0 && next_position.1 >= 0 && next_position.1 < grid_size.1 {
            // If there is an obstacle, turn right; else, move forward
            if grid[next_position.0 as usize][next_position.1 as usize] == '#' {
                direction_index = (direction_index + 1) % 4; // Turn right
            } else {
                position = next_position; // Move forward
                visited_positions.insert(position); // Mark visited
            }
        } else {
            break; // Move until off screen
        }
    }

    // Distinct positions visited
    visited_positions.len() as i32
}
