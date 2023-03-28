mod table;

use crate::game::table::{check_moves, parse_table};

pub fn play_game(lines: Vec<String>) -> Result<String, String> {
    let table = match parse_table(lines) {
        Ok(t) => t,
        Err(e) => return Err(e),
    };

    let (w, b) = check_moves(&table);
    if w && b {
        Ok("E".to_string()) // Draw
    } else if w {
        Ok("B".to_string()) // White wins
    } else if b {
        Ok("N".to_string()) // Black wins
    } else {
        Ok("P".to_string()) // Both lose
    }
}
