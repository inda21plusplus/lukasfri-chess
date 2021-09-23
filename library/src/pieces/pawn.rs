use crate::{structs::{Board, Color, Coordinate, Direction}, pieces::Piece};

#[derive(Debug, Clone)]
pub struct Pawn {
    pub color: Color,
    pub just_moved_twice: bool,
    pub direction: Direction,
}
impl Pawn {
    pub fn new(color: Color) -> Pawn {
        return Pawn {
            color,
            just_moved_twice: false,
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
    fn can_move(&mut self, board: &Board, from: &Coordinate, to: &Coordinate) -> bool {
        return true;
    }

    fn get_color(&self) -> Color {
        self.color
    }

    fn get_char(&self) -> &str {
        "♟︎"
    }
}