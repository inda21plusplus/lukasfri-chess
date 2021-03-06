use crate::{structs::{Board, Color, Coordinate}, pieces::Piece};
use crate::pieces::Rook;
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
    fn move_piece(&mut self, board: &mut Board, from: &Coordinate, to: &Coordinate) -> bool {
        let diff_x = i128::try_from(from.x).unwrap() - i128::try_from(to.x).unwrap();
        let diff_y = i128::try_from(from.y).unwrap() - i128::try_from(to.y).unwrap();
        if (diff_x.abs() == 2 && diff_y.abs() == 0) || (diff_x.abs() == 2 && diff_y.abs() == 0) {
            let mut x = to.x;
            if x < 0 {x = usize::MIN }
            else if x > 0 {x = usize::MAX };

            let mut y = to.y;
            if y < 0 {y = usize::MIN }
            else if y > 0 {y = usize::MAX };


            let line_of_sight = board.trace_line_of_sight_mut(from, &Coordinate{x, y});
            if line_of_sight.is_none() { return false; };
            let line_of_sight_square = line_of_sight.unwrap();
            if !line_of_sight_square.is_piece() { return false; };

            let maybe_rook_piece = line_of_sight_square.unwrap_mut();
            let maybe_rook = maybe_rook_piece.downcast_mut::<Rook>();
            if maybe_rook.is_none() { return false; };
            let rook = maybe_rook.unwrap();

        };
        if diff_x.abs() > 1 || diff_y.abs() > 1 { return false; };

        board.execute_move_piece(from, to);
        return true;
    }

    fn get_color(&self) -> Color {
        self.color
    }

    fn get_char(&self) -> &str {
        "♚"
    }
}