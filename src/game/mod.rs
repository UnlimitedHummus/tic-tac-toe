mod board;
mod player;
use board::*;
use player::Player;
pub fn play(){
    unimplemented!("whole gameplay loop in here");
}

struct Game {
    board: Board,
    turn: Player,

}