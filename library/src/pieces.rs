use crate::structs::{Board, Color, Coordinate, Direction};
mod PawnPiece;
pub use PawnPiece::Pawn;
mod KingPiece;
pub use KingPiece::King;
mod QueenPiece;
pub use QueenPiece::Queen;
mod KnightPiece;
pub use KnightPiece::Knight;
mod BishopPiece;
pub use BishopPiece::Bishop;
mod RookPiece;
pub use RookPiece::Rook;
// LÄRT MIG DETTA/LÅNAT FRÅN https://stackoverflow.com/questions/30353462/how-to-clone-a-struct-storing-a-boxed-trait-object
pub trait PieceClone {
    fn clone_box(&self) -> Box<dyn Piece>;
}

impl<T> PieceClone for T
where
    T: 'static + Piece + Clone,
{
    fn clone_box(&self) -> Box<dyn Piece> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Piece> {
    fn clone(&self) -> Box<dyn Piece> {
        self.clone_box()
    }
}

pub trait Piece: std::fmt::Debug + PieceClone {
    fn can_move(&self, board: &Board, from: &Coordinate, to: &Coordinate) -> bool;
    fn get_color(&self) -> Color;
    fn get_char(&self) -> &str;
}

impl dyn Piece {
    fn get_char(&self) -> &str {
        "?"
    }
}