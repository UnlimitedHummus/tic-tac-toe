mod board;
mod player;
mod tui;
use board::symbol::*;
use board::*;
use player::Player;
pub fn play() {
    tui::greet();
    let mut game = Game::new();
    for _round in 1..10 {
        tui::display_board(&game.board);
        let location = tui::get_user_input(game.active_player);
        match game
            .board
            .place(Symbol::from(game.active_player), &location)
        {
            Ok(board) => {
                game.board = board;
                game.swap_active_player();
            }
            Err(BoardError::LocationTaken(board)) => {
                game.board = board;
                tui::location_taken();
                continue;
            }
        }
        if game.board.is_winning() {
            game.swap_active_player();
            tui::display_board(&game.board);
            tui::you_won(game.active_player);
            return;
        }
    }
    tui::display_board(&game.board);
    tui::draw();
}

struct Game {
    board: Board,
    pub active_player: Player,
}

impl Game {
    fn new() -> Self {
        Game {
            board: Board::new(),
            active_player: Player::X,
        }
    }

    fn swap_active_player(&mut self) {
        self.active_player = !self.active_player;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn player_x_begins() {
        let game = Game::new();
        assert_eq!(game.active_player, Player::X)
    }
    #[test]
    fn swap_players() {
        let mut game = Game::new();
        assert_eq!(game.active_player, Player::X);
        game.swap_active_player();
        assert_eq!(game.active_player, Player::O);
        game.swap_active_player();
        assert_eq!(game.active_player, Player::X);
    }
}
