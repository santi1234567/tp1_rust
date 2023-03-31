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

mod tests {
    #[test]
    fn test_play_game() {
        use super::*;
        use crate::utils::read_file;
        assert_eq!(
            play_game(read_file("tables/game_E.txt").unwrap()).unwrap(),
            "E"
        );

        assert_eq!(
            play_game(read_file("tables/game_B.txt").unwrap()).unwrap(),
            "B"
        );

        assert_eq!(
            play_game(read_file("tables/game_N.txt").unwrap()).unwrap(),
            "N"
        );

        assert_eq!(
            play_game(read_file("tables/game_P.txt").unwrap()).unwrap(),
            "P"
        );
    }
}
