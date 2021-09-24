use std::{borrow::BorrowMut, convert::TryInto, ops::{Add, Sub}};
use crate::pieces::Piece;
use std::convert::TryFrom;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, Copy, PartialEq)]
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

    pub fn unwrap_mut(&mut self) -> &mut dyn Piece {
        self.0.as_mut().unwrap().as_mut().unwrap().as_mut()
    }
}

#[derive(Debug, Clone)]
pub struct Board {
    pub pieces: Vec<Square>,
    pub width: usize,
    pub height: usize,
    blocked: Square,
}
impl Board {
    pub fn new_board(width: usize, height: usize) -> Board {
        return Board{
            pieces: vec![Square::new_empty(); width * height],
            width,
            height,
            blocked: Square::new_blocked()
        };
    }

    pub fn fill<T: Piece + 'static>(&mut self, from: &Coordinate, to: &Coordinate, piece: T) {
        for x in (from.x)..=(to.x) {
            for y in (from.y)..=(to.y) {
                self.set_piece_box(&Coordinate{x, y}, piece.clone_box());
            }
        }
    }

    pub fn set_piece_square(&mut self, at: &Coordinate, piece_box: Square) -> bool {
        if at.x < 0 || at.y < 0 { false;}
        self.pieces[at.x + at.y * self.width] = piece_box;
        return true;
    }

    pub fn set_piece_box(&mut self, at: &Coordinate, piece_box: Box<dyn Piece>) -> bool {
        if at.x < 0 || at.y < 0 { false;}
        self.pieces[at.x + at.y * self.width] = Square::from_box(piece_box);
        return true;
    }

    pub fn set_piece<T: Piece + 'static>(&mut self, at: &Coordinate, piece: T) -> bool {
        self.set_piece_box(at, Box::from(piece));
        return true;
    }

    pub fn get_piece(&self, at: &Coordinate) -> &Square {
        if !(at.x < self.width && at.y < self.height) {
            return &self.blocked;
        }

        return &self.pieces[at.x + at.y * self.width];
    }

    pub fn get_piece_mut(&mut self, at: &Coordinate) -> &mut Square {
        if !(at.x < self.width && at.y < self.height) {
            return self.blocked.borrow_mut();
        }

        return self.pieces[at.x + at.y * self.width].borrow_mut();
    }

    pub fn trace_line_of_sight(&self, from: &Coordinate, to: &Coordinate) -> &Square {
        let diff_x = i128::try_from(from.x).unwrap() - i128::try_from(to.x).unwrap();
        let diff_y = i128::try_from(from.y).unwrap() - i128::try_from(to.y).unwrap();
        if diff_x != 0 && diff_y != 0 && diff_x != diff_y { panic!("HUH"); };

        let mut checked_square: &Square = &self.blocked;

        for i in 1..=(diff_x.abs().max(diff_y.abs())) {
            let x = i128::try_from(from.x).unwrap() + if diff_x == 0 {0} else  {diff_x/diff_x.abs()*i};
            let y = i128::try_from(from.y).unwrap() + if diff_y == 0 {0} else  {diff_y/diff_y.abs()*i};
            if x < 0 || y < 0 { return &self.blocked };

            checked_square = self.get_piece(&Coordinate{x: x.try_into().unwrap(), y: y.try_into().unwrap()});

            if checked_square.is_blocked() || checked_square.is_piece() { break; };
        };

        return checked_square;
    }

    pub fn execute_move_piece(&mut self, from: &Coordinate, to: &Coordinate) {
        let square = self.get_piece(from).clone();

        self.set_piece_square(to, square);
        self.set_piece_square(from, Square::new_empty());
    }
}

#[derive(Debug, Clone)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize
}