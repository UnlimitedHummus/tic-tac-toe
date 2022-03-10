pub mod board {
    use location::*;
    use std::fmt;
    #[derive(Debug, PartialEq)]
    pub enum BoardError {
        InvalidLocation,
        LocationTaken,
    }
    pub mod location {
        use super::*;
        #[derive(PartialEq, Debug)]
        pub struct Location(u8, u8);

        impl Location {
            pub fn new(x: u8, y: u8) -> Result<Self, BoardError> {
                Location::valid_location(x, y)?;
                Ok(Location(x, y))
            }

            fn valid_location(x: u8, y: u8) -> Result<(), BoardError> {
                match (x, y) {
                    (x, y) if x < 3 && y < 3 => Ok(()),
                    (_, _) => return Err(BoardError::InvalidLocation),
                }
            }

            pub fn get_x(&self) -> u8 {
                self.0
            }

            pub fn get_y(&self) -> u8 {
                self.1
            }
        }

        #[cfg(test)]
        mod tests {
            use super::*;
            #[test]
            fn invalid_location() {
                assert_eq!(Location::new(3, 2), Err(BoardError::InvalidLocation));
                assert_eq!(Location::new(1, 4), Err(BoardError::InvalidLocation));
            }
            #[test]
            fn valid_location() {
                let location = Location::new(1, 2).unwrap();
                assert_eq!(location.get_x(), 1_u8);
                assert_eq!(location.get_y(), 2_u8);
            }
            #[test]
            fn zero_location() {
                assert_eq!(Location::new(0, 0), Ok(Location(0, 0)));
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum Symbol {
        X,
        O,
        None,
    }

    impl fmt::Display for Symbol {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match &self {
                Symbol::X => write!(f,"X"),
                Symbol::O => write!(f,"O"),
                Symbol::None => write!(f, " "),
            }
        }
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
        pub fn place(mut self, symbol: Symbol, location: &Location) -> Result<Self, BoardError> {
            // check if place on board is free
            if self.get_symbol(location) != Symbol::None {
                return Err(BoardError::LocationTaken);
            }
            *self.get_slot(location)? = symbol;
            Ok(self)
        }

        fn get_slot<'a>(
            &'a mut self,
            location: &Location,
        ) -> Result<&'a mut Symbol, BoardError> {
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
        fn place_x() {
            let board = Board::new();
            let location = Location::new(0, 0).unwrap();
            assert_eq!(
                board.place(Symbol::X, &location),
                Ok(Board {
                    board: [[Symbol::X, Symbol::None, Symbol::None], [Symbol::None; 3], [Symbol::None; 3]]
                })
            )
        }
        #[test]
        fn place_y() {
            let board = Board::new();
            let location = Location::new(1, 0).unwrap();
            assert_eq!(
                board.place(Symbol::X, &location),
                Ok(Board {
                    board: [[Symbol::None; 3], [Symbol::X, Symbol::None, Symbol::None], [Symbol::None; 3]]
                })
            );
        }
        #[test]
        fn invalid_place() {
            let board = Board {
                board: [[Symbol::None; 3], [Symbol::X, Symbol::None, Symbol::None], [Symbol::None; 3]],
            };
            assert_eq!(
                board.place(Symbol::O, &Location::new(1, 0).unwrap()),
                Err(BoardError::LocationTaken)
            )
        }
        #[test]
        fn formatting() {
            let board = Board{board:[[Symbol::X, Symbol::O, Symbol::X],[Symbol::None, Symbol::O, Symbol::None],[Symbol::None, Symbol::None,Symbol::X]]};
            assert_eq!(board.to_string(),"X|O|X\n-+-+-\n |O| \n-+-+-\n | |X\n");
        }
    }
}
