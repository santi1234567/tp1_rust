pub mod position;

use crate::game::table::piece::position::Position;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum PieceType {
    R,
    D,
    A,
    C,
    T,
    P,
    Empty,
}
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Color {
    White,
    Black,
    Empty,
}

#[derive(Debug, PartialEq, Clone, Copy)]
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
    #[allow(dead_code)]
    pub fn copy(&self) -> Piece {
        Piece {
            piece_type: self.piece_type,
            position: self.position,
            color: self.color,
        }
    }
}
