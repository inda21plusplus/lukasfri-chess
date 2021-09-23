use crate::structs::{Board, Color, Coordinate, Direction};
mod pawn;
pub use pawn::Pawn;
mod king;
pub use king::King;
mod queen;
pub use queen::Queen;
mod knight;
pub use knight::Knight;
mod bishop;
pub use bishop::Bishop;
mod rook;
pub use rook::Rook;
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
    fn can_move(&mut self, board: &Board, from: &Coordinate, to: &Coordinate) -> bool;
    fn get_color(&self) -> Color;
    fn get_char(&self) -> &str;
}

impl dyn Piece {
    fn get_char(&self) -> &str {
        "?"
    }
}