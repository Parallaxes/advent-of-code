use std::fs;
use std::collections::HashMap;
use std::str::FromStr;

fn main() {
    let contents = fs::read_to_string("sample.txt").expect("Error reading file");
    solve(&contents);
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

fn evaluate<'a>(expression: &'a str, kvs: &mut HashMap<&'a str, u16>) -> Option<u16> {
    let tokens: Vec<&str> = expression.split_whitespace().collect();
    println!("expression: {}", expression);
    println!("tokens: {}", tokens.len());

    if tokens.len() == 5 {
        let left = if let Some(&val) = kvs.get(tokens[0]) {
            val
        } else {
            tokens[0].parse::<u16>().ok()?
        };

        let command = tokens[1].parse::<Command>().ok()?;

        let right = if let Some(&val) = kvs.get(tokens[2]) {
            val
        } else {
            tokens[2].parse::<u16>().ok()?
        };
        let var = tokens[4];

        match command {
            Command::RShift => {
                let result = left >> right;
                kvs.insert(var, result);
                Some(result)
            },
            Command::LShift => {
                let result = left << right;
                kvs.insert(var, result);
                Some(result)
            },
            Command::And => {
                let result = left & right;
                kvs.insert(var, result);
                Some(result)
            },
            Command::Or => {
                let result = left | right;
                kvs.insert(var, result);
                Some(result)
            },
            Command::Xor => {
                let result = left ^ right;
                kvs.insert(var, result);
                Some(result)
            },
            _ => None,
        }
    } else if tokens.len() == 4 && tokens[0] == "NOT" {
        let num = if let Some(&val) = kvs.get(tokens[1]) {
            val
        } else {
            panic!("Could not access nonexistent element")
        };
        let var = tokens[3];
        kvs.insert(var, !num);
        println!("Taking NOT of {}", num);
        println!("Inserting {}, {}", var, !num);
        Some(!num)
    } else if tokens.len() == 3 && tokens[1] == "->" {
        let value = tokens[0].parse::<u16>().ok()?;
        let var = tokens[2];
        kvs.insert(var, value);
        Some(value)
    } else {
        None
    }
}

fn solve(contents: &str) {
    let mut kvs: HashMap<&str, u16> = HashMap::new();

    for line in contents.lines() {
        let input = line;
        let value = evaluate(input, &mut kvs);
        if let Some(result) = value {
            println!("Result: {}", result);
        } else {
            println!("Invalid");
        }
    }

    let mut sorted_keys: Vec<&&str> = kvs.keys().collect();
    sorted_keys.sort();
    for key in sorted_keys {
        if let Some(value) = kvs.get(key) {
            println!("{}: {}", key, value);
        }
    }
}