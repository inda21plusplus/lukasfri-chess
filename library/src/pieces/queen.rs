use crate::{structs::{Board, Color, Coordinate}, pieces::Piece};

#[derive(Debug, Clone)]
pub struct Queen {
    pub color: Color,
}
impl Queen {
    pub fn new(color: Color) -> Queen {
        return Queen { color };
    }
}
impl Piece for Queen {
    #[inline(never)]
    fn can_move(&self, board: &Board, from: &Coordinate, to: &Coordinate) -> bool {
        println!("Queen can move!");
        return true;
    }

    fn get_color(&self) -> Color {
        self.color
    }

    fn get_char(&self) -> &str {
        "♛"
    }
}