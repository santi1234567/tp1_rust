mod game;
mod utils;

use crate::game::play_game;
use crate::utils::read_file;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    match parse_args(args.clone()) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    }

    let lines: Vec<String> = match read_file(args[1].clone()) {
        Ok(file_contents) => file_contents,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };
    match play_game(lines) {
        Ok(res) => println!("{}", res),
        Err(e) => eprintln!("{}", e),
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
