use std::{convert::TryInto, ops::{Add, Sub}};

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
        if (at.x < self.width && at.y < self.height) == false {
            return &self.blocked;
        }

        return &self.pieces[at.x + at.y * self.width];
    }
}

#[derive(Debug, Clone)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize
}