pub mod board {
    #[derive(Debug, PartialEq)]
    pub enum BoardError {
        InvalidLocation,
    }

    #[derive(Debug,PartialEq, Eq, Clone, Copy)]
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
    
        pub fn place(self, _symbol: Symbol, location: (u8,u8)) -> Result<Self, BoardError>{
            match location {
                (x,y) if x <3 && y <3 => (),
                (_,_) => return Err(BoardError::InvalidLocation)
            };
            todo!("implement the rest");
        }
    }
    
}

#[cfg(test)]
mod tests {
    use super::board::*;
    #[test]
    fn invalid_location() {
        let board = Board::new();
        let res = board.place(Symbol::O, (3,2));
        assert_eq!(res, Err(BoardError::InvalidLocation));
    }
}
