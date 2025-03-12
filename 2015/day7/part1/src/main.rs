use std::fs;
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

fn main() {
    let contents = fs::read_to_string("sample.txt").expect("Error reading file");
    println!("{:?}", solve(&contents));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Command {
    RShift,
    LShift,
    And,
    Or,
    Xor,
    Not,
    Assign,
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "RSHIFT" => Ok(Command::RShift),
            "LSHIFT" => Ok(Command::LShift),
            "AND" => Ok(Command::And),
            "OR" => Ok(Command::Or),
            "XOR" => Ok(Command::Xor),
            "NOT" => Ok(Command::Not),
            "->" => Ok(Command::Assign),
            _ => Err(()),
        }
    }
}

fn evaluate(expression: &str) -> Option<u16> {
    let tokens: Vec<&str> = expression.split_whitespace().collect();
    println!("expression: {}", expression);
    println!("tokens: {}", tokens.len());

    if tokens.len() == 5 {
        let left = tokens[0].parse::<u16>().ok()?;
        let command = tokens[1].parse::<Command>().ok()?;
        let right = tokens[2].parse::<u16>().ok()?;
        let var = tokens[4];

        match command {
            Command::RShift => {
                let result = left >> right;
                Some(result)
            },
            Command::LShift => {
                let result = left << right;
                Some(result)
            },
            Command::And => {
                let result = left & right;
                Some(result)
            },
            Command::Or => {
                let result = left | right;
                Some(result)
            },
            Command::Xor => {
                let result = left ^ right;
                Some(result)
            },
            Command::Assign => {
                let result = right;
                Some(result)
            },
            _ => None,
        }
    } else if tokens.len() == 2 && tokens[0] == "NOT" {
        let num = tokens[1].parse::<u16>().ok()?;
        Some(!num)
    } else {
        None
    }
}

fn solve(contents: &str) {
    let mut kvs: HashMap<String, i32> = HashMap::new();

    for line in contents.lines() {
        let input = line;
        if let Some(result) = evaluate(input) {
            println!("Result: {}", result);
        } else {
            println!("Invalid");
        }
    }
}