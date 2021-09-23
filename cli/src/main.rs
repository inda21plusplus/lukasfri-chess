use std::str::FromStr;

use colored::{ColoredString, Colorize};
use my_library::{game::Game, structs::{Color, Coordinate}};

fn print_board(game: &Game) {
    let mut output_string = String::new();
    for y_coord in (0..game.board.height).rev() {
        output_string.push_str(format!("{} ", y_coord + 1).as_str());
        for (x_coord, square) in game.board.pieces[y_coord*game.board.width..(y_coord+1)*game.board.width].iter().enumerate() {
            let mut piece_color: Option<Color> = None;
            let mut string = String::new();
            if square.is_empty() {
                string.push(' ');
            } else {
                let piece = square.unwrap();
                string.push_str(piece.get_char());
                piece_color = Some(piece.get_color());
            }
            string.push(' ');

            let mut colored_string: ColoredString = if (x_coord + (y_coord % 2)) % 2 == 0 {
                string.on_black()
            } else {
                string.on_bright_black()
            };

            if piece_color.is_some() {
                let color = piece_color.unwrap();
                colored_string = match color {
                    Color::White => colored_string.bright_white(),
                    Color::Black => colored_string.black(),
                    Color::Red => colored_string.bright_red(),
                    Color::Yellow => colored_string.bright_yellow(),
                    Color::Green => colored_string.bright_green(),
                    Color::Blue => colored_string.bright_blue(),
                };
            };

            output_string.push_str(format!("{}", colored_string).as_str());

            if x_coord + 1 == game.board.width { output_string.push_str("\n"); };
        }
    }
    output_string.push_str("  ");
    for i in 1..=game.board.width {
        output_string.push_str(format!("{} ", i).as_str());
    }

    print!("{}\n", output_string);
}

fn main() {
    let mut game: Game = Game::new();
    print_board(&game);
    game.move_piece(&Coordinate{x: 0, y: 0}, &Coordinate{x: 0, y: 5});
    print_board(&game);
    game.move_piece(&Coordinate{x: 0, y: 0}, &Coordinate{x: 0, y: 1});
    print_board(&game);
}
