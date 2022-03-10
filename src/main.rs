use tic_tac_toe::board::*;
fn main() {
    println!("Welcome to Tic-Tac-Toe!");
    let board = Board::new();
    // TODO:create a place method with simpler Syntax here
    let board = board.x_on(1,2).unwrap();
    let board = board.o_on(2,1).unwrap();
    println!("{}", board);
}
