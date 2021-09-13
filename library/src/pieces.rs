use crate::structs::{Board, Color, Coordinate, Direction};
// LÄRT MIG DETTA/LÅNAT FRÅN https://stackoverflow.com/questions/30353462/how-to-clone-a-struct-storing-a-boxed-trait-object
pub trait PieceClone {
    fn clone_box(&self) -> Box<dyn Piece>;
}

impl<T> PieceClone for T
where
    T: 'static + Piece + Clone,
{
    fn clone_box(&self) -> Box<dyn Piece> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Piece> {
    fn clone(&self) -> Box<dyn Piece> {
        self.clone_box()
    }
}

pub trait Piece: std::fmt::Debug + PieceClone {
    fn can_move(&self, board: &Board, from: &Coordinate, to: &Coordinate) -> bool;
    fn get_color(&self) -> Color;
    fn get_char(&self) -> &str;
}

impl Piece {
    fn get_char(&self) -> &str {
        "?"
    }
}

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
    #[inline(never)]
    fn can_move(&self, board: &Board, from: &Coordinate, to: &Coordinate) -> bool {
        println!("King can move!");
        return true;
    }

    fn get_color(&self) -> Color {
        self.color
    }

    fn get_char(&self) -> &str {
        "♚"
    }
}

#[derive(Debug, Clone)]
pub struct Queen {
    pub color: Color,
}
impl Queen {
    pub fn new(color: Color) -> Queen {
        return Queen { color };
    }
}
impl Piece for Queen {
    #[inline(never)]
    fn can_move(&self, board: &Board, from: &Coordinate, to: &Coordinate) -> bool {
        println!("Queen can move!");
        return true;
    }

    fn get_color(&self) -> Color {
        self.color
    }

    fn get_char(&self) -> &str {
        "♛"
    }
}

#[derive(Debug, Clone)]
pub struct Bishop {
    pub color: Color,
}
impl Bishop {
    pub fn new(color: Color) -> Bishop {
        return Bishop { color };
    }
}
impl Piece for Bishop {
    #[inline(never)]
    fn can_move(&self, board: &Board, from: &Coordinate, to: &Coordinate) -> bool {
        println!("Bishop can move!");
        return true;
    }

    fn get_color(&self) -> Color {
        self.color
    }

    fn get_char(&self) -> &str {
        "♝"
    }
}

#[derive(Debug, Clone)]
pub struct Knight {
    pub color: Color,
}
impl Knight {
    pub fn new(color: Color) -> Knight {
        return Knight { color };
    }
}
impl Piece for Knight {
    #[inline(never)]
    fn can_move(&self, board: &Board, from: &Coordinate, to: &Coordinate) -> bool {
        println!("Knight can move!");
        return true;
    }

    fn get_color(&self) -> Color {
        self.color
    }

    fn get_char(&self) -> &str {
        "♞"
    }
}

#[derive(Debug, Clone)]
pub struct Rook {
    pub color: Color,
    pub has_moved: bool,
}
impl Rook {
    pub fn new(color: Color) -> Rook {
        return Rook {
            color,
            has_moved: false,
        };
    }
}
impl Piece for Rook {
    #[inline(never)]
    fn can_move(&self, board: &Board, from: &Coordinate, to: &Coordinate) -> bool {
        println!("Rook can move!");
        return true;
    }

    fn get_color(&self) -> Color {
        self.color
    }

    fn get_char(&self) -> &str {
        "♜"
    }
}

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
    #[inline(never)]
    fn can_move(&self, board: &Board, from: &Coordinate, to: &Coordinate) -> bool {
        println!("Pawn can move!");
        return true;
    }

    fn get_color(&self) -> Color {
        self.color
    }

    fn get_char(&self) -> &str {
        "♟︎"
    }
}
