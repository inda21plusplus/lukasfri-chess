use crate::{structs::{Board, Color, Coordinate}, pieces::Piece};
use std::convert::TryFrom;

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
    fn can_move(&self, board: &Board, from: &Coordinate, to: &Coordinate) -> bool {
        let diff_x = i128::try_from(from.x).unwrap() - i128::try_from(to.x).unwrap();
        let diff_y = i128::try_from(from.y).unwrap() - i128::try_from(to.y).unwrap();
        if diff_x.abs() > 1 || diff_y.abs() > 1 { return false; };

        return true;
    }

    fn get_color(&self) -> Color {
        self.color
    }

    fn get_char(&self) -> &str {
        "♚"
    }
}