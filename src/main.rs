mod game;
mod utils;

use crate::game::play_game;
use crate::utils::read_file;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    match parse_args(&args) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    }

    let lines: Vec<String> = match read_file(&args[1]) {
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

/// Parses the command-line arguments passed to the program and ensures that they are valid.
///
/// The input `args` should be a vector of strings containing the command-line arguments.
///
/// Checks if only one argument was passed and if it is a .txt file. If not, returns an error.
///
/// # Arguments
///
/// * `args` - A vector of strings containing the command-line arguments.
///
/// # Returns
///
/// * `Ok(())` - If the command-line arguments are valid.
///
/// * `Err(String)` - An error message describing why the command-line arguments are invalid.
///
fn parse_args(args: &Vec<String>) -> Result<(), String> {
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
