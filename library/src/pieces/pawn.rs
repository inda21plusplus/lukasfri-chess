use crate::{structs::{Board, Color, Coordinate, Direction}, pieces::Piece};
use std::convert::TryFrom;

#[derive(Debug, Clone)]
pub struct Pawn {
    pub color: Color,
    pub just_moved_twice: bool,
    pub has_moved: bool,
    pub direction: Direction,
}
impl Pawn {
    pub fn new(color: Color) -> Pawn {
        return Pawn {
            color,
            just_moved_twice: false,
            has_moved: false,
            direction: match color {
                Color::White => Direction::North,
                Color::Black => Direction::South,
                Color::Red => Direction::North,
                Color::Yellow => Direction::East,
                Color::Green => Direction::South,
                Color::Blue => Direction::West,
            },
        };
    }
}
impl Piece for Pawn {
    fn move_piece(&mut self, board: &mut Board, from: &Coordinate, to: &Coordinate) -> bool {
        let diff_x = i128::try_from(from.x).unwrap() - i128::try_from(to.x).unwrap();
        let diff_y = i128::try_from(from.y).unwrap() - i128::try_from(to.y).unwrap();
        
        let mut x_allowed: i128 = 0;
        let mut y_allowed: i128 = 0;
        
        match self.direction {
            Direction::North => {
                x_allowed = 0;
                y_allowed = 1;
            },
            Direction::East => {
                x_allowed = 1;
                y_allowed = 0;
            },
            Direction::South => {
                x_allowed = 0;
                y_allowed = -1;
            },
            Direction::West => {
                x_allowed = -1;
                y_allowed = 0;
            },
        };

        if x_allowed.abs() == 1 && x_allowed.is_negative() == diff_x.is_negative() {
            if diff_x == 2 {
                if diff_y != 0 {return false;};
                if self.has_moved {return false;};
                let line_of_sight = board.trace_line_of_sight(from, to);
                if line_of_sight.is_none() { return false; };
                let line_of_sight_square = line_of_sight.unwrap();
                if !line_of_sight_square.is_empty() {return false;};

                let maybe_pawn_piece = line_of_sight_square.unwrap();
                let maybe_pawn = maybe_pawn_piece.downcast_ref::<Pawn>();
                if maybe_pawn.is_none() { return false; };
                let pawn = maybe_pawn.unwrap();
            }
            else if diff_y.abs() == 1 {
                let square = board.get_piece(to);
                return false;

            }
            else {return false;};
        };

        board.execute_move_piece(from, to);
        return true;
    }

    fn get_color(&self) -> Color {
        self.color
    }

    fn get_char(&self) -> &str {
        "♟︎"
    }
}