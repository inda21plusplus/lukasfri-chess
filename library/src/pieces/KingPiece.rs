use crate::{structs::{Board, Color, Coordinate}, pieces::Piece};

#[derive(Debug, Clone)]
pub struct King {
    pub color: Color,
    pub has_moved: bool,
}
impl King {
    pub fn new(color: Color) -> King {
        return King {
            color,
            has_moved: false,
        };
    }
}
impl Piece for King {
    #[inline(never)]
    fn can_move(&self, board: &Board, from: &Coordinate, to: &Coordinate) -> bool {
        println!("King can move!");
        return true;
    }

    fn get_color(&self) -> Color {
        self.color
    }

    fn get_char(&self) -> &str {
        "♚"
    }
}