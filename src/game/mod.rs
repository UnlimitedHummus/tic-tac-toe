mod board;
mod player;
mod tui;
use board::*;
use player::Player;
use board::symbol::*;
pub fn play(){
    tui::greet();
    let game = Game::new();
    while !game.board.is_winning(){ 
        let location = tui::get_user_input(game.active_player);
        //game.board = game.board.place(Symbol::from(game.active_player), &location);
    }
    unimplemented!("whole gameplay loop in here");
}

struct Game {
    board: Board,
    pub active_player: Player,
}

impl Game {
    fn new() -> Self {
        Game{board: Board::new(), active_player: Player::X}
    }

    fn swap_active_player(&mut self) {
        self.active_player = ! self.active_player;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn player_x_begins () {
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