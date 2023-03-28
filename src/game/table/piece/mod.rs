pub mod position;

use crate::game::table::piece::position::Position;
pub enum PieceType {
    R,
    D,
    A,
    C,
    T,
    P,
    Empty,
}
pub enum Color {
    White,
    Black,
    Empty,
}

pub struct Piece {
    pub piece_type: PieceType,
    pub position: Position,
    pub color: Color,
}

impl Piece {
    pub fn new() -> Piece {
        Piece {
            piece_type: PieceType::Empty,
            position: Position::default(),
            color: Color::Empty,
        }
    }
}
