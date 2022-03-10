pub mod board {
    #[derive(Debug, PartialEq)]
    pub enum BoardError {
        InvalidLocation,
        LocationTaken,
    }

    pub struct Location(pub u8, pub u8);

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum Symbol {
        X,
        O,
    }
    #[derive(PartialEq, Debug)]
    pub struct Board {
        board: [[Option<Symbol>; 3]; 3],
    }

    impl Board {
        pub fn new() -> Board {
            Board {
                board: [[None; 3]; 3],
            }
        }

        pub fn place(self, _symbol: Symbol, location: Location) -> Result<Self, BoardError> {
            Board::valid_location(&location)?;

            // check if place on board is free
            if self.get_symbol(location) != None {
                return Err(BoardError::LocationTaken);
            }
            todo!();
        }

        fn valid_location(location: &Location) -> Result<(), BoardError> {
            match *location {
                Location(x, y) if x < 3 && y < 3 => Ok(()),
                Location(_, _) => return Err(BoardError::InvalidLocation),
            }
        }

        fn get_symbol(&self, location: Location) -> Option<Symbol> {
            *self
                .board
                .get(location.0 as usize)
                .unwrap()
                .clone()
                .get(location.1 as usize)
                .unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::board::*;
    #[test]
    fn invalid_location() {
        let board = Board::new();
        let res = board.place(Symbol::O, Location(3, 2));
        assert_eq!(res, Err(BoardError::InvalidLocation));
    }
}
