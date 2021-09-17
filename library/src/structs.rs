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
pub struct Square(pub Option<Option<Box<dyn Piece>>>);
impl Square {
    pub fn new_with_piece<T: Piece + 'static>(piece: T) -> Square {
        return Square(Some(Some(Box::new(piece))));
    }

    pub fn from_box(piece_box: Box<dyn Piece>) -> Square {
        return Square(Some(Some(piece_box)));
    }

    pub fn new_empty() -> Square {
        return Square(Some(None));
    }

    pub fn new_blocked() -> Square {
        return Square(None);
    }

    pub fn is_empty(&self) -> bool {
        if self.is_blocked() { return true; };
        return self.0.as_ref().unwrap().is_none();
    }

    pub fn is_blocked(&self) -> bool {
        self.0.is_none()
    }

    pub fn is_piece(&self) -> bool {
        return !self.is_empty();
    }

    pub fn unwrap(&self) -> &dyn Piece {
        self.0.as_ref().unwrap().as_ref().unwrap().as_ref()
    }
}

#[derive(Debug, Clone)]
pub struct Board(pub Vec<Square>, usize, usize);
impl Board {
    pub fn new_board(width: usize, height: usize) -> Board {
        return Board(vec![Square::new_empty(); width * height], width, height);
    }

    pub fn fill<T: Piece + 'static>(&mut self, from: Coordinate, to: Coordinate, piece: T) {
        for x in (from.0)..=(to.0) {
            for y in (from.1)..=(to.1) {
                self.set_piece_box(Coordinate(x, y), piece.clone_box());
            }
        }
    }

    pub fn set_piece_box(&mut self, at: Coordinate, piece_box: Box<dyn Piece>) -> bool {
        self.0[at.0 + at.1 * self.1] = Square::from_box(piece_box);
        return true;
    }

    pub fn set_piece<T: Piece + 'static>(&mut self, at: Coordinate, piece: T) -> bool {
        self.set_piece_box(at, Box::from(piece));
        return true;
    }
}

#[derive(Debug, Clone)]
pub struct Coordinate(pub usize, pub usize);
