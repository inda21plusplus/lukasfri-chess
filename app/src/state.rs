use std::{ptr::NonNull, sync::mpsc::channel};

use arr_macro::arr;
use ggez::{filesystem::File, graphics::*, *};
use glam::Vec2;
use my_library::{self, game::Game, pieces::*, structs::Coordinate};

use crate::{properties::Properties, square_ui::SquareUI, WINDOW_HEIGHT, WINDOW_WIDTH};

#[derive(Debug)]
pub struct State {
    pub game: Game,
    properties: Properties,
    dt: std::time::Duration,
    piece_list: [Image; 12],
    squares: [Option<SquareUI>; 64],
    white_turn: bool,
    temporary_coordinate: Option<Coordinate>,
}

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        //Limit FPS to stop application from using loads of CPU Power
        const DESIRED_FPS: u32 = 60;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            self.dt = timer::delta(ctx);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        self.draw_board(ctx);
        self.draw_highlights(ctx);
        self.draw_text(ctx);
        self.draw_pieces(ctx);

        present(ctx)?;

        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: event::MouseButton,
        _x: f32,
        _y: f32,
    ) {
        self.temporary_coordinate = Some(Self::screen_to_coordinate(_x, _y));
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        _button: event::MouseButton,
        _x: f32,
        _y: f32,
    ) {
        let x: usize = (_x / (WINDOW_WIDTH / 8.0)).floor() as usize;
        let y: usize = (_y / (WINDOW_HEIGHT / 8.0)).floor() as usize;

        let coord: Coordinate = Coordinate { x, y };

        if self.temporary_coordinate.as_ref().unwrap().x == coord.x
            && self.temporary_coordinate.as_ref().unwrap().y == coord.y
        {
            self.temporary_coordinate = None;
            return;
        }

        self.game.move_piece(
            &self.temporary_coordinate.as_ref().unwrap(),
            &Coordinate { x, y },
        );

        self.temporary_coordinate = None;
    }
}

impl State {
    const WIDTH: f32 = 120.0;
    const BOARD_SIZE: u8 = 8;

    pub fn new(ctx: &mut Context) -> GameResult<State> {
        let s = Self {
            game: Game::new_default(),
            properties: Properties {
                font: Font::new(ctx, "/fonts/StickNoBills-Bold.ttf")?,
                debug: true,
            },
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
            squares: arr![None; 64],
            white_turn: true,
            temporary_coordinate: None,
        };

        Ok(s)
    }

    pub(crate) fn draw_board(&mut self, ctx: &mut Context) -> GameResult {
        let colors: [Color; 2] = [Color::from_rgb(222, 49, 99), Color::from_rgb(104, 18, 43)];
        for rank in 0..Self::BOARD_SIZE {
            for file in 0..Self::BOARD_SIZE {
                let index = rank * 8 + file;
                let x = Self::WIDTH * file as f32;
                let y = Self::WIDTH * rank as f32;
                let color = colors[((rank + file) % 2) as usize];

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

                if self.properties.debug {
                    let pos: Vec2 =
                        Vec2::new(x + Self::WIDTH / 2.0 - 16.0, y + Self::WIDTH / 2.0 - 16.0);
                    let dbg_text: Text = Text::new((index.to_string(), self.properties.font, 32.0));
                    draw(ctx, &dbg_text, (pos,))?;
                }

                draw(ctx, &square, DrawParam::default())?;
                self.squares[index as usize] = Some(SquareUI { square, x, y });
            }
        }
        Ok(())
    }

    pub(crate) fn draw_text(&mut self, ctx: &mut Context) -> GameResult {
        let colors: [Color; 2] = [Color::from_rgb(222, 49, 99), Color::from_rgb(104, 18, 43)];
        let list: [&str; 8] = ["A", "B", "C", "D", "E", "F", "G", "H"];
        for rank in 0..Self::BOARD_SIZE {
            for file in 0..Self::BOARD_SIZE {
                const OFFSET: f32 = 8.0;
                let x = Self::WIDTH * file as f32;
                let y = Self::WIDTH * rank as f32;
                let color = colors[((rank + file + 1) % 2) as usize];

                if file == Self::BOARD_SIZE - 1 {
                    let pos: Vec2 = Vec2::new(x + Self::WIDTH - 3.0 * OFFSET, y + OFFSET);
                    let text: Text =
                        Text::new(((rank + 1).to_string(), self.properties.font, 36.0));

                    draw(ctx, &text, DrawParam::new().dest(pos).color(color))?;
                }

                if rank == Self::BOARD_SIZE - 1 {
                    let pos: Vec2 = Vec2::new(
                        x + Self::WIDTH - 3.0 * OFFSET,
                        y + Self::WIDTH - 4.0 * OFFSET,
                    );
                    let text: Text =
                        Text::new((list[file as usize].to_string(), self.properties.font, 36.0));

                    draw(ctx, &text, DrawParam::new().dest(pos).color(color))?;
                }
            }
        }
        Ok(())
    }

    pub(crate) fn draw_highlights(&mut self, ctx: &mut Context) -> GameResult {
        let coord = self.temporary_coordinate.as_ref();
        if coord.clone().is_none() {
            return Ok(());
        }

        let p_square = self.squares[Self::coordinate_to_index(coord.as_ref().unwrap())].as_ref();
        let moves = self.game.get_moves(coord.as_ref().unwrap());
        if !moves.is_none() {
            let moves = moves.unwrap();
            for sq in moves {
                draw(
                    ctx,
                    &self.squares[Self::coordinate_to_index(&sq)]
                        .as_ref()
                        .unwrap()
                        .square,
                    DrawParam::new().color(Color::from_rgb(3, 25, 39)),
                )?;
            }

            draw(
                ctx,
                &p_square.unwrap().square,
                DrawParam::new().color(Color::from_rgb(241, 167, 188)),
            )?;
        }

        Ok(())
    }

    pub(crate) fn draw_statistics(&mut self, ctx: &mut Context) -> GameResult {
        let start_pos: f32 = Self::WIDTH * Self::BOARD_SIZE as f32;

        Ok(())
    }

    pub(crate) fn draw_pieces(&mut self, ctx: &mut Context) -> GameResult {
        const POS_OFFSET: f32 = 16.0;
        let mut counter: usize = 0;
        for piece in self.game.board.pieces.clone() {
            if piece.clone().0.unwrap().is_none() {
                counter += 1;
                continue;
            }
            let p: Box<dyn Piece> = piece.0.unwrap().unwrap();

            let color_offset: usize = if p.get_color() == my_library::structs::Color::Black {
                6
            } else {
                0
            };

            let image: &Image = match p.get_char() {
                "♟︎" => &self.piece_list[0 + color_offset],
                "♞" => &self.piece_list[1 + color_offset],
                "♜" => &self.piece_list[2 + color_offset],
                "♝" => &self.piece_list[3 + color_offset],
                "♛" => &self.piece_list[4 + color_offset],
                "♚" => &self.piece_list[5 + color_offset],
                _ => &self.piece_list[0],
            };
            let pos: Vec2 = Vec2::new(
                self.squares[counter].as_ref().unwrap().x + POS_OFFSET,
                self.squares[counter].as_ref().unwrap().y + POS_OFFSET,
            );
            let scale: Vec2 = Vec2::new(1.4, 1.4);

            draw(ctx, image, DrawParam::new().dest(pos).scale(scale));
            counter += 1;
        }

        Ok(())
    }

    fn coordinate_to_screen(coord: Coordinate) -> (f32, f32) {
        let x = (coord.x as f32) * (WINDOW_HEIGHT / 8.0);
        let y = (coord.y as f32) * (WINDOW_HEIGHT / 8.0);

        (x, y)
    }

    fn screen_to_coordinate(x: f32, y: f32) -> Coordinate {
        let x: usize = (x / (WINDOW_HEIGHT / 8.0)).floor() as usize;
        let y: usize = (y / (WINDOW_HEIGHT / 8.0)).floor() as usize;
        Coordinate { x, y }
    }

    fn coordinate_to_index(coord: &Coordinate) -> usize {
        coord.x + coord.y * 8
    }
}
