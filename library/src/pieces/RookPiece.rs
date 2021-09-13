use crate::{structs::{Board, Color, Coordinate}, pieces::Piece};

#[derive(Debug, Clone)]
pub struct Rook {
    pub color: Color,
    pub has_moved: bool,
}
impl Rook {
    pub fn new(color: Color) -> Rook {
        return Rook {
            color,
            has_moved: false,
        };
    }
}
impl Piece for Rook {
    #[inline(never)]
    fn can_move(&self, board: &Board, from: &Coordinate, to: &Coordinate) -> bool {
        println!("Rook can move!");
        return true;
    }

    fn get_color(&self) -> Color {
        self.color
    }

    fn get_char(&self) -> &str {
        "♜"
    }
}