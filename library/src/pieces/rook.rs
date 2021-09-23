use crate::{structs::{Board, Color, Coordinate}, pieces::Piece};
use std::convert::TryFrom;

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
    fn can_move(&self, board: &Board, from: &Coordinate, to: &Coordinate) -> bool {
        let diff_x = i128::try_from(from.x).unwrap() - i128::try_from(to.x).unwrap();
        let diff_y = i128::try_from(from.y).unwrap() - i128::try_from(to.y).unwrap();
        if diff_x.abs() == 0 || diff_y.abs() == 0 { return false; };

        return true;
    }

    fn get_color(&self) -> Color {
        self.color
    }

    fn get_char(&self) -> &str {
        "♜"
    }
}