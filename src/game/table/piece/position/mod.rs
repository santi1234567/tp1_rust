/// A struct representing the position of a chess piece on the board.
#[derive(Default, PartialEq, Debug)]
pub struct Position {
    /// The x-coordinate of the position.
    pub x: usize,
    /// The y-coordinate of the position.
    pub y: usize,
}
