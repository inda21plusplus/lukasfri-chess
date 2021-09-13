use crate::pieces::Piece;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, Copy)]
pub enum Color {
    White,
    Black,
    Red,
    Yellow,
    Green,
    Blue,
}

#[derive(Debug, Clone)]
pub struct Square(pub bool, pub Option<Box<dyn Piece>>);
impl Square {
    pub fn new_with_piece<T: Piece + 'static>(piece: T) -> Square {
        return Square(true, Some(Box::new(piece)));
    }

    pub fn from_box(piece_box: Box<dyn Piece>) -> Square {
        return Square(true, Some(piece_box));
    }

    pub fn new_empty() -> Square {
        return Square(true, None);
    }

    pub fn new_blocked() -> Square {
        return Square(false, None);
    }
}

#[derive(Debug, Clone)]
pub struct Board(pub Vec<Vec<Square>>);
impl Board {
    pub fn new_board(width: usize, height: usize) -> Board {
        return Board(vec![vec![Square::new_empty(); height]; width]);
    }

    pub fn fill<T: Piece + 'static>(&mut self, from: Coordinate, to: Coordinate, piece: T) {
        for x in (from.0)..=(to.0) {
            for y in (from.1)..=(to.1) {
                self.set_piece_box(Coordinate(x, y), piece.clone_box());
            }
        }
    }

    pub fn set_piece_box(&mut self, at: Coordinate, piece_box: Box<dyn Piece>) -> bool {
        self.0[at.0][at.1] = Square::from_box(piece_box);
        return true;
    }

    pub fn set_piece<T: Piece + 'static>(&mut self, at: Coordinate, piece: T) -> bool {
        self.0[at.0][at.1] = Square::new_with_piece(piece);
        return true;
    }
}

#[derive(Debug, Clone)]
pub struct Coordinate(pub usize, pub usize);
