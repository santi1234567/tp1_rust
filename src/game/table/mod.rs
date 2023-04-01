pub mod piece;

use crate::game::table::piece::position::Position;
use crate::game::table::piece::{Color, Piece, PieceType};
use std::cmp::{max, min};

#[derive(PartialEq, Debug)]
pub struct Table {
    white_piece: Piece,
    black_piece: Piece,
}

// impl PartialEq for Table {
//     fn eq(&self, other: &Self) -> bool {
//         self.white_piece == other.white_piece && self.black_piece == other.black_piece
//     }
// }

const BOARD_SIZE: usize = 8;

pub fn parse_table(lines: &Vec<String>) -> Result<Table, String> {
    let mut table = Table {
        white_piece: Piece::new(),
        black_piece: Piece::new(),
    };
    if lines.len() != BOARD_SIZE {
        return Err(format!(
            "ERROR: Table formated incorrectly. Table has {} rows, expected {}",
            lines.len(),
            BOARD_SIZE
        ));
    }
    for (line_number, line) in lines.iter().enumerate() {
        parse_line(line.to_string(), &mut table, line_number)?;
    }

    if let PieceType::Empty = table.white_piece.piece_type {
        return Err("ERROR: No white piece inserted".to_string());
    }
    if let PieceType::Empty = table.black_piece.piece_type {
        return Err("ERROR: No black piece inserted".to_string());
    }

    Ok(table)
}

fn parse_line(line: String, table: &mut Table, line_number: usize) -> Result<(), String> {
    if line.len() != BOARD_SIZE * 2 - 1 {
        return Err(format!(
            "ERROR: Table formated incorrectly. Row number {} has {} characters, expected {}",
            line_number,
            line.len(),
            BOARD_SIZE * 2 - 1
        ));
    }
    for (char_number, word) in line.split_whitespace().enumerate() {
        if word.len() != 1 {
            return Err(format!("ERROR: Invalid piece: {}", word));
        }
        let c = word.chars().next().unwrap();
        if c.is_ascii_lowercase() {
            match table.white_piece.piece_type {
                PieceType::Empty => {
                    table.white_piece.piece_type = match c {
                        'r' => PieceType::R,
                        'd' => PieceType::D,
                        'a' => PieceType::A,
                        'c' => PieceType::C,
                        't' => PieceType::T,
                        'p' => PieceType::P,
                        _ => return Err(format!("ERROR: Invalid piece: {}", c)),
                    };
                    table.white_piece.position.y = line_number;
                    table.white_piece.position.x = char_number;
                    table.white_piece.color = Color::White;
                }
                _ => return Err("ERROR: More than one white piece inserted".to_string()),
            }
        } else if c.is_ascii_uppercase() {
            match table.black_piece.piece_type {
                PieceType::Empty => {
                    table.black_piece.piece_type = match c {
                        'R' => PieceType::R,
                        'D' => PieceType::D,
                        'A' => PieceType::A,
                        'C' => PieceType::C,
                        'T' => PieceType::T,
                        'P' => PieceType::P,
                        _ => return Err(format!("ERROR: Invalid piece: {}", c)),
                    };
                    table.black_piece.position.y = line_number;
                    table.black_piece.position.x = char_number;
                    table.black_piece.color = Color::Black;
                }
                _ => return Err("ERROR: More than one black piece inserted".to_string()),
            }
        } else if c != '_' {
            return Err(format!("ERROR: Invalid piece: {}", c));
        }
    }
    Ok(())
}

pub fn check_moves(table: &Table) -> (bool, bool) {
    let white_piece = &table.white_piece;
    let black_piece = &table.black_piece;

    (
        check_move_piece(white_piece, black_piece),
        check_move_piece(black_piece, white_piece),
    )
}

fn check_move_piece(attacker: &Piece, other: &Piece) -> bool {
    match attacker.piece_type {
        PieceType::R => check_move_r(&attacker.position, &other.position),
        PieceType::D => check_move_d(&attacker.position, &other.position),
        PieceType::A => check_move_a(&attacker.position, &other.position),
        PieceType::C => check_move_c(&attacker.position, &other.position),
        PieceType::T => check_move_t(&attacker.position, &other.position),
        PieceType::P => check_move_p(attacker, other),
        PieceType::Empty => false,
    }
}

fn check_move_r(attacker_position: &Position, other_position: &Position) -> bool {
    for i in attacker_position.x - 1..=attacker_position.x + 1 {
        for j in attacker_position.y - 1..=attacker_position.y + 1 {
            if i == other_position.x && j == other_position.y {
                return true;
            }
        }
    }
    false
}

fn check_move_d(attacker_position: &Position, other_position: &Position) -> bool {
    check_move_a(attacker_position, other_position)
        || check_move_t(attacker_position, other_position)
}

fn check_move_a(attacker_position: &Position, other_position: &Position) -> bool {
    // Diagonal moves

    // Diagonal moves from top left to bottom right
    for i in 1..=BOARD_SIZE - max(attacker_position.x, attacker_position.y) - 1 {
        if attacker_position.x + i == other_position.x
            && attacker_position.y + i == other_position.y
        {
            return true;
        }
    }

    // Diagonal moves from top right to bottom left
    for i in 1..=min(attacker_position.x, BOARD_SIZE - attacker_position.y - 1) {
        if attacker_position.x - i == other_position.x
            && attacker_position.y + i == other_position.y
        {
            return true;
        }
    }

    // Diagonal moves from bottom left to top right
    for i in 1..=min(attacker_position.y, BOARD_SIZE - attacker_position.x - 1) {
        if attacker_position.x + i == other_position.x
            && attacker_position.y - i == other_position.y
        {
            return true;
        }
    }

    // Diagonal moves from bottom right to top left
    for i in 1..=min(attacker_position.x, attacker_position.y) {
        if attacker_position.x - i == other_position.x
            && attacker_position.y - i == other_position.y
        {
            return true;
        }
    }
    false
}

fn check_move_t(attacker_position: &Position, other_position: &Position) -> bool {
    // Horizontal and vertical moves
    if attacker_position.x == other_position.x || attacker_position.y == other_position.y {
        return true;
    }
    false
}

fn check_move_c(attacker_position: &Position, other_position: &Position) -> bool {
    // Top left

    if (attacker_position.x > 0
        && attacker_position.y > 1
        && attacker_position.x - 1 == other_position.x
        && attacker_position.y - 2 == other_position.y)
        || (attacker_position.x > 1
            && attacker_position.y > 0
            && attacker_position.x - 2 == other_position.x
            && attacker_position.y - 1 == other_position.y)
    {
        return true;
    }

    // Top right

    if (attacker_position.x < BOARD_SIZE - 1
        && attacker_position.y > 1
        && attacker_position.x + 1 == other_position.x
        && attacker_position.y - 2 == other_position.y)
        || (attacker_position.x < BOARD_SIZE - 2
            && attacker_position.y > 0
            && attacker_position.x + 2 == other_position.x
            && attacker_position.y - 1 == other_position.y)
    {
        return true;
    }

    // Bottom left

    if (attacker_position.x > 0
        && attacker_position.y < BOARD_SIZE - 1
        && attacker_position.x - 1 == other_position.x
        && attacker_position.y + 2 == other_position.y)
        || (attacker_position.x > 1
            && attacker_position.y < BOARD_SIZE - 2
            && attacker_position.x - 2 == other_position.x
            && attacker_position.y + 1 == other_position.y)
    {
        return true;
    }

    // Bottom right

    if (attacker_position.x < BOARD_SIZE - 1
        && attacker_position.y < BOARD_SIZE - 1
        && attacker_position.x + 1 == other_position.x
        && attacker_position.y + 2 == other_position.y)
        || (attacker_position.x < BOARD_SIZE - 2
            && attacker_position.y < BOARD_SIZE - 2
            && attacker_position.x + 2 == other_position.x
            && attacker_position.y + 1 == other_position.y)
    {
        return true;
    }

    false
}

fn check_move_p(attacker: &Piece, other: &Piece) -> bool {
    // Pawn moves
    match attacker.color {
        Color::White => check_move_p_white(&attacker.position, &other.position),
        Color::Black => check_move_p_black(&attacker.position, &other.position),
        Color::Empty => false, // Should not happen
    }
}

fn check_move_p_white(attacker_position: &Position, other_position: &Position) -> bool {
    // White pawn moves

    if (attacker_position.x > 0
        && attacker_position.x < BOARD_SIZE - 1
        && attacker_position.y > 0
        && attacker_position.y - 1 == other_position.y
        && ((attacker_position.x - 1 == other_position.x)
            || (attacker_position.x + 1 == other_position.x)))
        || (attacker_position.x == 0 && (attacker_position.x + 1 == other_position.x))
        || (attacker_position.x == BOARD_SIZE - 1 && (attacker_position.x - 1 == other_position.x))
    {
        return true;
    }

    false
}

fn check_move_p_black(attacker_position: &Position, other_position: &Position) -> bool {
    // Black pawn moves

    if (attacker_position.x > 0
        && attacker_position.x < BOARD_SIZE - 1
        && attacker_position.y < BOARD_SIZE - 1
        && attacker_position.y + 1 == other_position.y
        && ((attacker_position.x - 1 == other_position.x)
            || (attacker_position.x + 1 == other_position.x)))
        || ((attacker_position.x == 0 && (attacker_position.x + 1 == other_position.x))
            || (attacker_position.x == BOARD_SIZE - 1
                && (attacker_position.x - 1 == other_position.x)))
    {
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_table() {
        use super::*;
        use crate::utils::read_file;
        fn setup_parse_table_test(file_path: &str) -> Vec<String> {
            match read_file(&file_path) {
                Ok(file_contents) => file_contents,
                Err(e) => {
                    println!("{}", e);
                    panic!();
                }
            }
        }
        let mut lines = setup_parse_table_test("tables/no_white.txt");

        assert!(parse_table(&lines).is_err());
        assert_eq!(
            parse_table(&lines).err(),
            Some("ERROR: No white piece inserted".to_string())
        );

        lines = setup_parse_table_test("tables/no_black.txt");

        assert!(parse_table(&lines).is_err());
        assert_eq!(
            parse_table(&lines).err(),
            Some("ERROR: No black piece inserted".to_string())
        );

        let lines = setup_parse_table_test("tables/invalid_piece.txt");

        assert!(parse_table(&lines).is_err());
        assert_eq!(
            parse_table(&lines).err(),
            Some("ERROR: Invalid piece: x".to_string())
        );

        let lines = setup_parse_table_test("tables/invalid_table_format_chars.txt");

        assert!(parse_table(&lines).is_err());
        assert_eq!(
            parse_table(&lines).err(),
            Some(
                "ERROR: Table formated incorrectly. Row number 4 has 17 characters, expected 15"
                    .to_string()
            )
        );

        let lines = setup_parse_table_test("tables/invalid_table_format_rows.txt");

        assert!(parse_table(&lines).is_err());
        assert_eq!(
            parse_table(&lines).err(),
            Some("ERROR: Table formated incorrectly. Table has 9 rows, expected 8".to_string())
        );

        let lines = setup_parse_table_test("tables/2_black.txt");

        assert!(parse_table(&lines).is_err());
        assert_eq!(
            parse_table(&lines).err(),
            Some("ERROR: More than one black piece inserted".to_string())
        );

        let lines = setup_parse_table_test("tables/2_white.txt");

        assert!(parse_table(&lines).is_err());
        assert_eq!(
            parse_table(&lines).err(),
            Some("ERROR: More than one white piece inserted".to_string())
        );

        let lines = setup_parse_table_test("tables/d.txt");

        assert!(parse_table(&lines).is_ok());
        assert_eq!(
            parse_table(&lines).unwrap(),
            Table {
                white_piece: Piece {
                    color: Color::White,
                    position: Position { x: 5, y: 7 },
                    piece_type: PieceType::D
                },
                black_piece: Piece {
                    color: Color::Black,
                    position: Position { x: 5, y: 1 },
                    piece_type: PieceType::T
                }
            }
        );
    }

    #[test]
    fn test_moves() {
        use super::*;
        use crate::utils::read_file;
        fn setup_parse_table_test(file_path: &str) -> Vec<String> {
            match read_file(&file_path) {
                Ok(file_contents) => file_contents,
                Err(e) => {
                    println!("{}", e);
                    panic!();
                }
            }
        }
        fn setup_move_test(file_path: &str) -> Table {
            let lines = setup_parse_table_test(file_path);
            parse_table(&lines).unwrap()
        }
        let mut table = setup_move_test("tables/d.txt");
        assert_eq!(check_moves(&table), (true, true));

        table = setup_move_test("tables/r.txt");
        assert_eq!(check_moves(&table), (true, true));

        table = setup_move_test("tables/a.txt");
        assert_eq!(check_moves(&table), (true, false));

        table = setup_move_test("tables/c.txt");
        assert_eq!(check_moves(&table), (true, false));

        table = setup_move_test("tables/t.txt");
        assert_eq!(check_moves(&table), (true, true));

        table = setup_move_test("tables/p.txt");
        assert_eq!(check_moves(&table), (true, true));

        table = setup_move_test("tables/p_border.txt");
        assert_eq!(check_moves(&table), (false, false));
    }
}
