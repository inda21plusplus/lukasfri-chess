use crate::{structs::{Board, Color, Coordinate}, pieces::Piece};

#[derive(Debug, Clone)]
pub struct Bishop {
    pub color: Color,
}
impl Bishop {
    pub fn new(color: Color) -> Bishop {
        return Bishop { color };
    }
}
impl Piece for Bishop {
    #[inline(never)]
    fn can_move(&self, board: &Board, from: &Coordinate, to: &Coordinate) -> bool {
        println!("Bishop can move!");
        return true;
    }

    fn get_color(&self) -> Color {
        self.color
    }

    fn get_char(&self) -> &str {
        "♝"
    }
}