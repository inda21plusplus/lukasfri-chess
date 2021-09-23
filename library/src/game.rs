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

        board.fill(&Coordinate{x: 0, y: 0}, &Coordinate{x: 7, y: 0}, Pawn::new(Color::White));

        board.set_piece(&Coordinate{x: 0, y: 0}, King::new(Color::White));

        return Game {
            turn: Color::White,
            board,
        };
    }

    pub fn move_piece(&mut self, from: &Coordinate, to: &Coordinate) -> bool {
        let square = self.board.get_piece(from);
        if from.x == to.x && from.y == to.y { return false; };
        if !square.is_piece() || !square.unwrap().can_move(&self.board, from, to) { return false; };
        self.execute_move(from, to);
        return true;
    }

    fn execute_move(&mut self, from: &Coordinate, to: &Coordinate) {
        let square = self.board.get_piece(from).clone();

        self.board.set_piece_square(to, square);
        self.board.set_piece_square(from, Square::new_empty());
    }
}
