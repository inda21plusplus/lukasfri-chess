use ggez::{graphics::Mesh, GameResult};

#[derive(Debug)]
pub struct SquareUI {
    pub square: Mesh,
    pub x: f32,
    pub y: f32,
}
