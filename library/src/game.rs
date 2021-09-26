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

    pub fn new_default() -> Self {
        const WIDTH: usize = 8usize;
        const HEIGHT: usize = 8usize;

        let mut board = Board::new_board(8usize, 8usize);

        board.fill(&Coordinate{x: 0, y: 1}, &Coordinate{x: 7, y: 1}, Pawn::new(Color::White));
        board.set_piece(&Coordinate{x: 4, y: 0}, King::new(Color::White));
        board.set_piece(&Coordinate{x: 3, y: 0}, Queen::new(Color::White));
        board.set_piece(&Coordinate{x: 0, y: 0}, Rook::new(Color::White));
        board.set_piece(&Coordinate{x: 7, y: 0}, Rook::new(Color::White));
        board.set_piece(&Coordinate{x: 1, y: 0}, Knight::new(Color::White));
        board.set_piece(&Coordinate{x: 6, y: 0}, Knight::new(Color::White));
        board.set_piece(&Coordinate{x: 2, y: 0}, Bishop::new(Color::White));
        board.set_piece(&Coordinate{x: 5, y: 0}, Bishop::new(Color::White));

        board.fill(&Coordinate{x: 0, y: 6}, &Coordinate{x: 7, y: 6}, Pawn::new(Color::Black));
        board.set_piece(&Coordinate{x: 4, y: 7}, King::new(Color::Black));
        board.set_piece(&Coordinate{x: 3, y: 7}, Queen::new(Color::Black));
        board.set_piece(&Coordinate{x: 0, y: 7}, Rook::new(Color::Black));
        board.set_piece(&Coordinate{x: 7, y: 7}, Rook::new(Color::Black));
        board.set_piece(&Coordinate{x: 1, y: 7}, Knight::new(Color::Black));
        board.set_piece(&Coordinate{x: 6, y: 7}, Knight::new(Color::Black));
        board.set_piece(&Coordinate{x: 2, y: 7}, Bishop::new(Color::Black));
        board.set_piece(&Coordinate{x: 5, y: 7}, Bishop::new(Color::Black));

        return Game {
            turn: Color::White,
            board,
        };
    }

    pub fn move_piece(&mut self, from: &Coordinate, to: &Coordinate) -> bool {
        let mut square = self.board.get_piece(from).clone();
        if from.x == to.x && from.y == to.y { return false; };
        if !square.is_piece() { return false; };
        let piece = square.unwrap_mut();
        if piece.get_color() != self.turn { return false; };
        let success = piece.move_piece(&mut self.board, from, to);

        if success {
            match self.turn {
                Color::White => {
                    self.turn = Color::Black;
                },
                Color::Black => {
                    self.turn = Color::White;
                },
                Color::Red => {
                    self.turn = Color::Yellow;
                },
                Color::Yellow => {
                    self.turn = Color::Green;
                },
                Color::Green => {
                    self.turn = Color::Blue;
                },
                Color::Blue => {
                    self.turn = Color::Red;
                },
            };
        };

        return success;
    }

    pub fn get_moves(&mut self, at: &Coordinate) -> Option<Vec<Coordinate>> {
        let mut square = self.board.get_piece(at).clone();
        if !square.is_piece() { return None; };
        let piece = square.unwrap_mut();

        return Some(vec![Coordinate{x: 0, y: 0}]);
    }
}
