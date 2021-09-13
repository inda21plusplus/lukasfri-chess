use std::str::FromStr;

use colored::{ColoredString, Colorize};
use my_library::{game::Game, structs::Color};

fn print_board(game: &Game) {
    for (x_coord, x) in (&game.board.0).iter().enumerate() {
        let mut output_string = String::new();
        for (y_coord, y) in (x).iter().enumerate() {
            let mut piece_color: Option<Color> = None;
            let mut string = String::new();
            if !y.0 || y.1.is_none() {
                string.push(' ');
            } else {
                let piece = y.1.as_ref().unwrap();
                string.push_str(piece.get_char());
                piece_color = Some(piece.get_color());
            }
            string.push(' ');

            let mut colored_string: ColoredString = if ((y_coord + (x_coord % 2)) % 2) == 0 {
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
        }

        print!("{}\n", output_string);
    }
}

fn main() {
    let game: Game = Game::new();
    print_board(&game);
}
