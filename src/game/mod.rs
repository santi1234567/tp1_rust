mod table;

use crate::game::table::{check_moves, parse_table};

/// Plays a game of chess as described by the input `lines`, and returns the result of the game.
///
/// The input `lines` should be a vector of strings, where each string represents a row of the chess board.
///
/// # Arguments
///
/// * `lines` - A vector of strings representing the chess board.
///
/// # Returns
///
/// * `Ok(String)` - The result of the game, which can be one of the following values:
///   * "E" - Draw
///   * "B" - White wins
///   * "N" - Black wins
///   * "P" - Both lose
///
/// * `Err(String)` - An error message.
///
pub fn play_game(lines: Vec<String>) -> Result<String, String> {
    let table = match parse_table(&lines) {
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

#[cfg(test)]
mod tests {
    #[test]
    fn test_play_game() {
        use super::*;
        use crate::utils::read_file;
        assert_eq!(
            play_game(read_file(&"tables/game_E.txt").unwrap()).unwrap(),
            "E"
        );

        assert_eq!(
            play_game(read_file(&"tables/game_B.txt").unwrap()).unwrap(),
            "B"
        );

        assert_eq!(
            play_game(read_file(&"tables/game_N.txt").unwrap()).unwrap(),
            "N"
        );

        assert_eq!(
            play_game(read_file(&"tables/game_P.txt").unwrap()).unwrap(),
            "P"
        );
    }
}
