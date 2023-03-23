mod game;

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use crate::game::play_game;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut result = parse_args(args.clone());
    if let Err(e) = result {
        eprintln!("{}", e);
        return;
    }

    let lines: Vec<String>;

    match read_file(args[1].clone()) {
        Ok(file_contents) => lines = file_contents,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    }
    result = play_game(lines);
    if let Err(e) = result {
        eprintln!("{}", e);
        return;
    }
}

fn parse_args(args: Vec<String>) -> Result<(), String> {
    if args.len() < 2 {
        return Err("ERROR: Not enough arguments".to_string());
    }
    if args.len() >= 3 {
        return Err("ERROR: Too many arguments passed".to_string());
    }
    if !args[1].ends_with(".txt") {
        return Err("ERROR: Arguments should be entered in format: -- <file.txt>".to_string());
    }

    Ok(())
}

fn read_file(file_name: impl AsRef<Path>) -> Result<Vec<String>, String> {
    let file = File::open(file_name);
    if let Err(e) = file {
        return Err(format!("ERROR: {}", e));
    }
    let file = file.unwrap();
    let reader = BufReader::new(file);
    let lines = reader
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();
    Ok(lines)
}