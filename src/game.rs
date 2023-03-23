const BOARD_SIZE: usize = 8;

enum PieceType {
    R,
    D,
    A,
    C,
    T,
    P,
    Empty
}
  
struct Piece {
    piece_type: PieceType,
    position: Position
}

#[derive(Default)]

struct Position {
    x: usize,
    y: usize
}



struct Table {
    white_piece: Piece,
    black_piece: Piece
}

pub fn play_game(lines: Vec<String>) -> Result<(), String> {
    let table;
    match parse_table(lines) {
        Ok(t) => table = t,
        Err(e) => return Err(e)
    }


 
    Ok(())
}

fn parse_table(lines: Vec<String>) -> Result<Table, String> {
    let mut table = Table {
        white_piece: Piece {
            piece_type: PieceType::Empty,
            position: Position::default()
        },
        black_piece: Piece {
            piece_type: PieceType::Empty,
            position: Position::default()
        }
    };
    if lines.len() != BOARD_SIZE {
        return Err(format!("ERROR: Table formated incorrectly. Expected {} rows, got {}", BOARD_SIZE, lines.len()));
    }
    for (line_number, line) in lines.iter().enumerate() {
        let result = parse_line(line.to_string(), &mut table, line_number);
        if let Err(e) = result {
            return Err(e);
        }
    }

    match table.white_piece.piece_type {
        PieceType::Empty => return Err("ERROR: No white piece inserted".to_string()),
        _ => {}
    }
        
    match table.black_piece.piece_type {
        PieceType::Empty => return Err("ERROR: No black piece inserted".to_string()),
        _ => {}
    }

    Ok(table)
}

fn parse_line(line: String, table: &mut Table, line_number: usize) -> Result<(), String> {
    if line.len() != BOARD_SIZE * 2 - 1{
        return Err(format!("ERROR: Table formated incorrectly. Row number {} has {} characters, expected {}", line_number, line.len(), BOARD_SIZE * 2 - 1));
    }
    for (char_number,word) in line.split_whitespace().enumerate() {
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
                        _ => return Err(format!("ERROR: Invalid piece: {}", c))
                    };
                    table.white_piece.position.y = line_number;
                    table.white_piece.position.x = char_number;
                }
                _ => return Err("ERROR: More than 1 white pieces inserted".to_string())
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
                        _ => return Err(format!("ERROR: Invalid piece: {}", c))
                    };
                    table.black_piece.position.y = line_number;
                    table.black_piece.position.x = char_number;
                }
                _ => return Err("ERROR: More than 1 black pieces inserted".to_string())
            }
        } else if c != '_' {
            return Err(format!("ERROR: Invalid piece: {}", c));
        }
    }
    Ok(())
}

    