mod location;
mod symbol;
use location::*;
use symbol::*;
use std::fmt;
#[derive(Debug, PartialEq)]
pub enum BoardError {
    InvalidLocation,
    LocationTaken,
}

// TODO:add turn to board struct so incorrect sequences of assignment can be caught
#[derive(PartialEq, Debug)]
pub struct Board {
    board: [[Symbol; 3]; 3],
}

impl Board {
    pub fn new() -> Board {
        Board {
            board: [[Symbol::None; 3]; 3],
        }
    }

    // TODO:refactor to make placing easier and not expose the location struct
    pub fn x_on(self, x: u8, y: u8) -> Result<Self, BoardError> {
        self.place(Symbol::X, x, y)
    }
    pub fn o_on(self, x: u8, y: u8) -> Result<Self, BoardError> {
        self.place(Symbol::O, x, y)
    }
    fn place(mut self, symbol: Symbol, x: u8, y: u8) -> Result<Self, BoardError> {
        let location = Location::new(x, y)?;
        if self.get_symbol(&location) != Symbol::None {
            return Err(BoardError::LocationTaken);
        }
        *self.get_slot(&location)? = symbol;
        Ok(self)
    }

    fn get_slot<'a>(&'a mut self, location: &Location) -> Result<&'a mut Symbol, BoardError> {
        if self.get_symbol(location) != Symbol::None {
            Err(BoardError::LocationTaken)
        } else {
            Ok(&mut self.board[location.get_x() as usize][location.get_y() as usize])
        }
    }

    fn get_symbol(&self, location: &Location) -> Symbol {
        *self
            .board
            .get(location.get_x() as usize)
            .unwrap()
            .clone()
            .get(location.get_y() as usize)
            .unwrap()
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}|{}|{}\n-+-+-\n{}|{}|{}\n-+-+-\n{}|{}|{}\n",
            self.get_symbol(&Location::new(0, 0).unwrap()),
            self.get_symbol(&Location::new(0, 1).unwrap()),
            self.get_symbol(&Location::new(0, 2).unwrap()),
            self.get_symbol(&Location::new(1, 0).unwrap()),
            self.get_symbol(&Location::new(1, 1).unwrap()),
            self.get_symbol(&Location::new(1, 2).unwrap()),
            self.get_symbol(&Location::new(2, 0).unwrap()),
            self.get_symbol(&Location::new(2, 1).unwrap()),
            self.get_symbol(&Location::new(2, 2).unwrap()),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn place_x_valid() {
        let board = Board::new();
        assert_eq!(
            board.x_on(0, 0),
            Ok(Board {
                board: [
                    [Symbol::X, Symbol::None, Symbol::None],
                    [Symbol::None; 3],
                    [Symbol::None; 3]
                ]
            })
        )
    }
    #[test]
    fn place_x_invalid() {
        assert_eq!(Board::new().x_on(3, 2), Err(BoardError::InvalidLocation))
    }
    #[test]
    fn place_o_valid() {
        assert_eq!(
            Board::new().o_on(1, 0),
            Ok(Board {
                board: [
                    [Symbol::None; 3],
                    [Symbol::O, Symbol::None, Symbol::None],
                    [Symbol::None; 3]
                ]
            })
        );
    }
    #[test]
    fn place_o_invalid() {
        let board = Board {
            board: [
                [Symbol::None; 3],
                [Symbol::X, Symbol::None, Symbol::None],
                [Symbol::None; 3],
            ],
        };
        assert_eq!(board.o_on(1, 0), Err(BoardError::LocationTaken))
    }
    #[test]
    fn formatting() {
        let board = Board {
            board: [
                [Symbol::X, Symbol::O, Symbol::X],
                [Symbol::None, Symbol::O, Symbol::None],
                [Symbol::None, Symbol::None, Symbol::X],
            ],
        };
        assert_eq!(board.to_string(), "X|O|X\n-+-+-\n |O| \n-+-+-\n | |X\n");
    }
}
