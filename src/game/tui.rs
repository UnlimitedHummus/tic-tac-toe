use super::player::Player;
use super::board::symbol::*;
use super::board::location::Location;
use std::io;
use super::board::Board;
// TODO: figure out how to unit test this function
pub fn get_user_input(player: Player) -> Location 
{
    let mut input = String::new();
    loop{
        println!("Where would you like to place your {} :", Symbol::from(player));
        io::stdin().read_line(&mut input).expect("Reading input went wrong");
        if let Ok(index) = input.split_whitespace().next().unwrap().parse::<usize>() {
            if let Ok(location) = Location::try_from(index - 1) { // work 0 indexed only internally
                return location
            } else {
                println!("Please enter a number between 1 and 9");
            }
        } else {
            println!("There was an error parsing your input");
        }
    }

}
pub fn location_taken() {
    println!("The last input was not valid. Please try again!");
}
pub fn display_board(board: &Board){
    println!("{}", board)
}
pub fn greet(){
    println!("Welcome to Tic-Tac-Toe!");
}

pub fn you_won(player: Player) {
    println!("Congratulations Player {} you won!", Symbol::from(player));
}

#[cfg(test)]
mod test {
    
}