use crate::{structs::{Board, Color, Coordinate}, pieces::Piece};

#[derive(Debug, Clone)]
pub struct Knight {
    pub color: Color,
}
impl Knight {
    pub fn new(color: Color) -> Knight {
        return Knight { color };
    }
}
impl Piece for Knight {
    #[inline(never)]
    fn can_move(&self, board: &Board, from: &Coordinate, to: &Coordinate) -> bool {
        println!("Knight can move!");
        return true;
    }

    fn get_color(&self) -> Color {
        self.color
    }

    fn get_char(&self) -> &str {
        "♞"
    }
}