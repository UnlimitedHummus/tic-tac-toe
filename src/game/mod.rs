mod board;
mod player;
use board::*;
use player::Player;
pub fn play(){
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
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn player_x_begins () {
        let game = Game::new();
        assert_eq!(game.active_player, Player::X)
    }
}