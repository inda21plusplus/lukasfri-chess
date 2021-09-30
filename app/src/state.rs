use ggez::graphics::*;
use ggez::*;
use glam::*;
use my_library::{game::Game, structs::Coordinate};

#[derive(Debug)]
pub struct State {
    pub game: Game,
    dt: std::time::Duration,
    piece_list: [Image; 12],
    white_turn: bool,
}

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const DESIRED_FPS: u32 = 30;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            self.dt = timer::delta(ctx);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        self.draw_board(ctx);
        self.draw_highlights(ctx);
        self.draw_pieces(ctx);

        present(ctx)?;
        Ok(())
    }
}

impl State {
    const WIDTH: f32 = 120.0;
    const BOARD_SIZE: u8 = 8;

    pub fn new(ctx: &mut Context) -> GameResult<State> {
        let s = Self {
            game: Game::new_default(),
            dt: std::time::Duration::new(0, 0),
            piece_list: [
                Image::new(ctx, "/pawn.png")?,
                Image::new(ctx, "/knight.png")?,
                Image::new(ctx, "/rook.png")?,
                Image::new(ctx, "/bishop.png")?,
                Image::new(ctx, "/queen.png")?,
                Image::new(ctx, "/king.png")?,
                Image::new(ctx, "/pawn_black.png")?,
                Image::new(ctx, "/knight_black.png")?,
                Image::new(ctx, "/rook_black.png")?,
                Image::new(ctx, "/bishop_black.png")?,
                Image::new(ctx, "/queen_black.png")?,
                Image::new(ctx, "/king_black.png")?,
            ],
            white_turn: true,
        };

        dbg!(&s);

        Ok(s)
    }

    pub(crate) fn draw_board(&mut self, ctx: &mut Context) -> GameResult {
        let COLORS: [Color; 2] = [Color::from_rgb(222, 49, 99), Color::from_rgb(104, 18, 43)];
        for rank in 0..Self::BOARD_SIZE {
            for file in 0..Self::BOARD_SIZE {
                let index = rank * 8 + file;
                let x = Self::WIDTH * file as f32;
                let y = Self::WIDTH * rank as f32;
                let color = COLORS[((rank + file) % 2) as usize];

                let square = Mesh::new_rectangle(
                    ctx,
                    DrawMode::fill(),
                    Rect {
                        x,
                        y,
                        w: Self::WIDTH,
                        h: Self::WIDTH,
                    },
                    color,
                )?;
                draw(ctx, &square, DrawParam::default());
            }
        }
        Ok(())
    }

    pub(crate) fn draw_statistics(&mut self, ctx: &mut Context) -> GameResult {
        let start_pos: f32 = Self::WIDTH * Self::BOARD_SIZE as f32;

        Ok(())
    }

    pub(crate) fn draw_pieces(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }

    pub(crate) fn draw_highlights(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }
}
