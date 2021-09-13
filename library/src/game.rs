use crate::pieces::{Bishop, King, Knight, Pawn, Queen, Rook};
use crate::structs::{Board, Color, Coordinate, Direction, Square};

#[derive(Debug)]
pub struct Game {
    pub turn: Color,
    pub board: Board,
}

impl Game {
    pub fn new() -> Self {
        const WIDTH: usize = 8usize;
        const HEIGHT: usize = 8usize;

        let mut board = Board::new_board(8usize, 8usize);

        board.fill(Coordinate(0, 0), Coordinate(7, 0), Pawn::new(Color::White));

        board.set_piece(Coordinate(0, 0), King::new(Color::White));

        return Game {
            turn: Color::White,
            board,
        };
    }

    pub fn move_piece(&self, from: &Coordinate, to: &Coordinate) {}

    fn execute_move(&self, from: &Coordinate, to: &Coordinate) {}
}
