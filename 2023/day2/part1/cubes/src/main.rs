/*
 * Program     : cubes
 * Author      : Parallaxis (Parallaxes)
 * Date        : 10-06-2024
 * Version     : 1.0
 * Description : My solution the Day 2, Part 1 of Advent of Code 2023.
*/

use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

#[derive(Debug)]
struct Game {
    game_id: String,
    subsets: Vec<Vec<String>>,
}

// Get game data. We seperate this into fields game_id and subsets
fn parse_game_data(input: &str) -> Vec<Game> {
    let mut games = Vec::new();

    for line in input.lines() {
        if let Some((id, subsets_str)) = line.split_once(": ") {
            let subsets: Vec<Vec<String>> = subsets_str
                .split(';')
                .map(|subset| {
                    subset.trim()
                        .split(',')
                        .map(|color| color.trim().to_string())
                        .collect()
                })
                .collect();

            games.push(Game {
                game_id: id.parse().unwrap(),
                subsets,
            });
        }
    }

    games
}

// Set the color maxes struct
struct ColorMax {
    red: u32,
    green: u32,
    blue: u32,
}

// Check to see if the colors are lower than the maxes
fn check_color_counts(games: &[Game], color_max: &ColorMax) -> u32 {
    let mut sum = 0;

    for game in games {
        let game_num = game.game_id.split_whitespace().nth(1).unwrap_or("0").parse::<u32>().unwrap_or(0);

        let mut all_within_limits = true; // Flag to check if all counts are within limits

        for (i, subset) in game.subsets.iter().enumerate() {
            for color in subset {
                // Split the count and color
                if let Some((count_str, color_name)) = color.split_once(' ') {
                    if let Ok(count) = count_str.parse::<u32>() {
                        // Check against the maximum values for each color
                        match color_name.to_lowercase().as_str() {
                            "red" if count > color_max.red => {
                                println!("In {}, subset {}: '{}' exceeds RED_MAX of {}.", game.game_id, i + 1, color, color_max.red);
                                all_within_limits = false; // Set flag to false
                            }
                            "green" if count > color_max.green => {
                                println!("In {}, subset {}: '{}' exceeds GREEN_MAX of {}.", game.game_id, i + 1, color, color_max.green);
                                all_within_limits = false; // Set flag to false
                            }
                            "blue" if count > color_max.blue => {
                                println!("In {}, subset {}: '{}' exceeds BLUE_MAX of {}.", game.game_id, i + 1, color, color_max.blue);
                                all_within_limits = false; // Set flag to false
                            }
                            _ => {}
                        }
                    }
                }
            }  

            // Check if the total number of cubes in the subset exceeds the maximum sum
            let total_cubes: u32 = subset.iter().filter_map(|color| {
                color.split_once(' ').and_then(|(count_str, _)| count_str.parse::<u32>().ok())
            }).sum();
    
            let max_sum = color_max.red + color_max.green + color_max.blue;
            if total_cubes > max_sum {
                println!("In {}, subset {}: total cubes {} exceed MAX_SUM of {}.", game.game_id, i + 1, total_cubes, max_sum);
                all_within_limits = false; // Set flag to false
            }
        }

        // If all counts are within limits, add the game number to the sum
        if all_within_limits {
            println!("All counts are within limits for game {}.", game.game_id);
            sum += game_num;
        }
    }

    sum
}

// Driver
fn main() {
    let path = Path::new("input.txt");
    let file = match File::open(&path) {
        Err(why) => {
            println!("Couldn't open {}: {}", path.display(), why);
            return;
        },
        Ok(file) => file,
    };

    let mut contents = String::new();
    if let Err(why) = io::BufReader::new(file).read_to_string(&mut contents) {
        println!("Couldn't read {}: {}", path.display(), why);
        return;
    }

    let games = parse_game_data(&contents);

    let color_max = ColorMax {
        red: 12,
        green: 13,
        blue: 14,
    };

    let total_sum = check_color_counts(&games, &color_max);

    println!("The total sum of game numbers is: {}", total_sum);
}

// :3