pub mod game;
pub mod pieces;
pub mod structs;

#[cfg(test)]
mod tests {
    use crate::{game::Game, structs::Coordinate};

    #[test]
    fn move_pawn() {
        
        let mut game: Game = Game::new_default();
        assert_eq!(game.move_piece(&Coordinate{x: 4, y: 1}, &Coordinate{x: 4, y: 3}), true);
    }

    #[test]
    fn move_pawn_not_on_your_turn() {
        
        let mut game: Game = Game::new_default();
        assert_eq!(game.move_piece(&Coordinate{x: 4, y: 1}, &Coordinate{x: 4, y: 3}), true);
        assert_eq!(game.move_piece(&Coordinate{x: 4, y: 3}, &Coordinate{x: 4, y: 4}), false);
    }

    #[test]
    fn move_pawns() {
        
        let mut game: Game = Game::new_default();
        assert_eq!(game.move_piece(&Coordinate{x: 4, y: 1}, &Coordinate{x: 4, y: 3}), true);
        assert_eq!(game.move_piece(&Coordinate{x: 4, y: 6}, &Coordinate{x: 4, y: 4}), true);
        assert_eq!(game.move_piece(&Coordinate{x: 3, y: 1}, &Coordinate{x: 3, y: 3}), true);
    }

    #[test]
    fn move_pawns_into_others_illegally() {
        let mut game: Game = Game::new_default();
        assert_eq!(game.move_piece(&Coordinate{x: 4, y: 1}, &Coordinate{x: 4, y: 3}), true);
        assert_eq!(game.move_piece(&Coordinate{x: 4, y: 6}, &Coordinate{x: 4, y: 4}), true);
        assert_eq!(game.move_piece(&Coordinate{x: 4, y: 3}, &Coordinate{x: 4, y: 4}), false);
    }

    #[test]
    fn move_knight() {
        let mut game: Game = Game::new_default();
        assert_eq!(game.move_piece(&Coordinate{x: 1, y: 0}, &Coordinate{x: 2, y: 2}), true);
    }

    #[test]
    fn move_rook_illegally_to_friendly() {
        let mut game: Game = Game::new_default();
        assert_eq!(game.move_piece(&Coordinate{x: 0, y: 0}, &Coordinate{x: 0, y: 1}), false);
    }

    #[test]
    fn move_rook_illegally_diagonally() {
        let mut game: Game = Game::new_default();
        assert_eq!(game.move_piece(&Coordinate{x: 1, y: 1}, &Coordinate{x: 1, y: 2}), true);
        assert_eq!(game.move_piece(&Coordinate{x: 1, y: 6}, &Coordinate{x: 1, y: 5}), true);
        assert_eq!(game.move_piece(&Coordinate{x: 0, y: 0}, &Coordinate{x: 1, y: 1}), false);
    }

    #[test]
    fn move_bishop_illegally_to_friendly() {
        let mut game: Game = Game::new_default();
        assert_eq!(game.move_piece(&Coordinate{x: 2, y: 0}, &Coordinate{x: 0, y: 1}), false);
    }

    #[test]
    fn move_bishop_illegally_straight() {
        let mut game: Game = Game::new_default();
        assert_eq!(game.move_piece(&Coordinate{x: 1, y: 0}, &Coordinate{x: 2, y: 2}), true);
        assert_eq!(game.move_piece(&Coordinate{x: 1, y: 6}, &Coordinate{x: 1, y: 5}), true);
        assert_eq!(game.move_piece(&Coordinate{x: 2, y: 0}, &Coordinate{x: 1, y: 0}), false);
    }

    #[test]
    fn move_castle_short() {
        let mut game: Game = Game::new_default();
        assert_eq!(game.move_piece(&Coordinate{x: 4, y: 1}, &Coordinate{x: 4, y: 2}), true);
        assert_eq!(game.move_piece(&Coordinate{x: 0, y: 6}, &Coordinate{x: 0, y: 5}), true);
        assert_eq!(game.move_piece(&Coordinate{x: 5, y: 0}, &Coordinate{x: 4, y: 1}), true);
        assert_eq!(game.move_piece(&Coordinate{x: 1, y: 6}, &Coordinate{x: 1, y: 5}), true);
        assert_eq!(game.move_piece(&Coordinate{x: 2, y: 6}, &Coordinate{x: 2, y: 5}), true);
        assert_eq!(game.move_piece(&Coordinate{x: 6, y: 0}, &Coordinate{x: 7, y: 2}), true);
        assert_eq!(game.move_piece(&Coordinate{x: 3, y: 6}, &Coordinate{x: 3, y: 5}), true);
        assert_eq!(game.move_piece(&Coordinate{x: 4, y: 0}, &Coordinate{x: 6, y: 0}), true);
    }

    #[test]
    fn move_castle_long() {
        let mut game: Game = Game::new_default();
        assert_eq!(game.move_piece(&Coordinate{x: 1, y: 0}, &Coordinate{x: 2, y: 2}), true);
        assert_eq!(game.move_piece(&Coordinate{x: 1, y: 6}, &Coordinate{x: 1, y: 5}), true);
        assert_eq!(game.move_piece(&Coordinate{x: 2, y: 0}, &Coordinate{x: 1, y: 0}), false);
    }
}
