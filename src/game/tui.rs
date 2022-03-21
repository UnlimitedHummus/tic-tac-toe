use super::player::Player;
use super::board::symbol::*;
use super::board::location::Location;
use std::io;
// TODO: figure out how to unit test this function
pub fn get_user_input(player: Player) -> Location 
{
    let mut input = String::new();
    loop{
        println!("Where would you like to place your {} :", Symbol::from(player));
        io::stdin().read_line(&mut input).expect("Reading input went wrong");
        if let Ok(index) = input.split_whitespace().next().unwrap().parse::<usize>() {
            if let Ok(location) = Location::from(index) {
                return location
            } else {
                println!("Please enter a number between 1 and 9");
            }
        } else {
            println!("There was an error parsing your input");
        }
    }

}
pub fn invalid_location() {
    println!("The last input was not valid. Please try again!");
}
pub fn display_board(){
    todo!("Display the board");
}
pub fn greet(){
    println!("Welcome to Tic-Tac-Toe!");
}

#[cfg(test)]
mod test {
    
}