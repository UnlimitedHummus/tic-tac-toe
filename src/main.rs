use tic_tac_toe::board::*;
use tic_tac_toe::board::location::*;
fn main() {
    println!("Welcome to Tic-Tac-Toe!");
    let board = Board::new();
    let board = board.place(Symbol::X, &Location::new(1,2).unwrap()).unwrap();
    let board = board.place(Symbol::O, &Location::new(2,1).unwrap()).unwrap();
    println!("{}", board);
}
