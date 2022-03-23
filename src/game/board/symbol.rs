use super::super::player::Player;
use std::fmt;
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Symbol {
    X,
    O,
    None,
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Symbol::X => write!(f, "X"),
            Symbol::O => write!(f, "O"),
            Symbol::None => write!(f, " "),
        }
    }
}

impl From<Player> for Symbol {
    fn from(player: Player) -> Self {
        match player {
            Player::X => Symbol::X,
            Player::O => Symbol::O,
        }
    }
}
