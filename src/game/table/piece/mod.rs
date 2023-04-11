pub mod position;

use crate::game::table::piece::position::Position;

/// An enum representing the types of pieces in a table game.
#[derive(PartialEq, Debug)]
pub enum PieceType {
    /// King.
    R,
    /// Queen.
    D,
    /// Bishop.
    A,
    /// Knight.
    C,
    /// Rook.
    T,
    /// Pawn.
    P,
    /// Empty.
    Empty,
}

/// An enum representing the colors of chess piece.
#[derive(PartialEq, Debug)]
pub enum Color {
    /// The white color.
    White,
    /// The black color.
    Black,
    /// An empty color.
    Empty,
}

/// A struct representing a chess piece.
#[derive(Debug, PartialEq)]
pub struct Piece {
    /// The type of the piece.
    pub piece_type: PieceType,
    /// The position of the piece on the chessboard.
    pub position: Position,
    /// The color of the piece.
    pub color: Color,
}

impl Piece {
    /// Creates a new `Piece` instance with default values.
    pub fn new() -> Piece {
        Piece {
            piece_type: PieceType::Empty,
            position: Position::default(),
            color: Color::Empty,
        }
    }
}
