pub mod location;
pub mod symbol;
use location::*;
use std::fmt;
use symbol::*;

#[derive(Debug, PartialEq)]
pub enum BoardError {
    LocationTaken(Board),
}

#[derive(PartialEq, Debug, Clone)]
pub struct Board {
    board: [Symbol; 9],
}

impl Board {
    pub fn new() -> Board {
        Board {
            board: [Symbol::None; 9],
        }
    }

    pub fn place(mut self, symbol: Symbol, location: &Location) -> Result<Self, BoardError> {
        if self.get_symbol(location) != Symbol::None {
            return Err(BoardError::LocationTaken(self));
        }
        *self.get_slot(location) = symbol;
        Ok(self)
    }
    //TODO: make location not a borrow location now implements the copy trait
    fn get_slot<'a>(&'a mut self, location: &Location) -> &'a mut Symbol {
        &mut self.board[usize::from(*location)]
    }

    fn get_symbol(&self, location: &Location) -> Symbol {
        *self.board.get(usize::from(*location)).unwrap()
    }

    pub fn is_winning(&self) -> bool {
        self.winning_row() || self.winning_diagonal() || self.winning_col()
    }
    fn winning_col(&self) -> bool {
        [Symbol::X, Symbol::O].iter().any(|&comp| {
            (0..3).any(|n| {
                self.board
                    .iter()
                    .skip(n)
                    .step_by(3)
                    .all(|&symbol| symbol == comp)
            })
        })
    }
    fn winning_row(&self) -> bool {
        [Symbol::X, Symbol::O].iter().any(|&comp| {
            self.board
                .windows(3)
                .step_by(3)
                .any(|row| row.iter().all(|&symbol| symbol == comp))
        })
    }
    fn winning_diagonal(&self) -> bool {
        let board = self.board;
        let main_diagonal = [Symbol::X, Symbol::O]
            .iter()
            .any(|&comp| board[0] == comp && board[4] == comp && board[8] == comp);
        let anti_diagonal = [Symbol::X, Symbol::O]
            .iter()
            .any(|&comp| board[2] == comp && board[4] == comp && board[6] == comp);
        main_diagonal || anti_diagonal
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}|{}|{}\n-+-+-\n{}|{}|{}\n-+-+-\n{}|{}|{}\n",
            self.get_symbol(&Location::try_from(0).unwrap()),
            self.get_symbol(&Location::try_from(1).unwrap()),
            self.get_symbol(&Location::try_from(2).unwrap()),
            self.get_symbol(&Location::try_from(3).unwrap()),
            self.get_symbol(&Location::try_from(4).unwrap()),
            self.get_symbol(&Location::try_from(5).unwrap()),
            self.get_symbol(&Location::try_from(6).unwrap()),
            self.get_symbol(&Location::try_from(7).unwrap()),
            self.get_symbol(&Location::try_from(8).unwrap()),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    macro_rules! new_board {
        ($a:tt,$b:tt,$c:tt;$d:tt,$e:tt,$f:tt;$g:tt,$h:tt,$i:tt) => {
            Board {
                board: [
                    Symbol::$a,
                    Symbol::$b,
                    Symbol::$c,
                    Symbol::$d,
                    Symbol::$e,
                    Symbol::$f,
                    Symbol::$g,
                    Symbol::$h,
                    Symbol::$i,
                ],
            }
        };
    }
    #[test]
    fn place_x_valid() {
        let board = Board::new();
        assert_eq!(
            board.place(Symbol::X, &Location::try_from(0).unwrap()),
            Ok(new_board!(
                X, None, None;
                None,None,None;
                None,None,None))
        );
    }
    #[test]
    fn place_o_valid() {
        assert_eq!(
            Board::new().place(Symbol::O, &Location::try_from(1).unwrap()),
            Ok(new_board!(
                None,O,None;
                None,None,None;
                None,None,None
            ))
        );
    }
    #[test]
    fn place_o_invalid() {
        let board = new_board!(
            None,X,None;
            None,None,None;
            None,None,None
        );
        let copy = board.clone();
        assert_eq!(
            board.place(Symbol::O, &Location::try_from(1).unwrap()),
            Err(BoardError::LocationTaken(copy))
        )
    }
    #[test]
    fn formatting() {
        let board = new_board!(
            X,O,X;
            None,O,None;
            None,None,X);
        assert_eq!(board.to_string(), "X|O|X\n-+-+-\n |O| \n-+-+-\n | |X\n");
    }
    #[test]
    fn x_winning_on_first_row() {
        let board = new_board!(
                X,X,X;
                None,O,None;
                O,None,X);
        assert_eq!(board.winning_row(), true);
    }
    #[test]
    fn o_winning_on_first_row() {
        let board = new_board!(
            O,O,O;
            X,O,None;
            O,None,X
        );
        assert_eq!(board.winning_row(), true);
    }
    #[test]
    fn x_winning_on_third_row() {
        let board = new_board!(
            O,X,O;
            X,O,None;
            X,X,X
        );
        assert_eq!(board.winning_row(), true);
    }
    #[test]
    fn no_one_winning() {
        let board = new_board!(
            O,X,O;
            X,O,None;
            X,O,X);
        assert_eq!(board.winning_row(), false);
    }
    #[test]
    fn empty_board_no_winners() {
        let board = Board::new();
        assert_eq!(board.winning_row(), false);
    }
    #[test]
    fn winning_col() {
        let board = new_board!(
            X,X,O;
            X,O,O;
            X,None,X
        );
        assert_eq!(board.winning_col(), true);
        let board = new_board!(
                X,O,X;
                None,O,O;
                O,O,X);
        assert_eq!(board.winning_col(), true);
        let board = new_board!(
                X,X,O;
                None,O,O;
                X,O,O);
        assert_eq!(board.winning_col(), true);
    }
    #[test]
    fn no_winning_col() {
        let board = Board::new();
        assert_eq!(board.winning_col(), false);
        let board = board
            .place(Symbol::X, &Location::try_from(2).unwrap())
            .unwrap();
        assert_eq!(board.winning_col(), false);
    }
    #[test]
    fn winning_x_main_diagonal() {
        let board = new_board!(
            X,X,O;
            None,X,O;
            X,O,X);
        assert_eq!(board.winning_diagonal(), true);
    }
    #[test]
    fn winning_o_main_diagonal() {
        let board = new_board!(
            O, None, None;
            X, O, None;
            O, X, O);
        assert_eq!(board.winning_diagonal(), true);
    }
    #[test]
    fn winning_o_antidiagonal() {
        let board = new_board!(
            None, X, O;
            None, O, X;
            O, None, X
        );
        assert_eq!(board.winning_diagonal(), true);
    }
    #[test]
    fn no_winning_diagonal() {
        let board = new_board!(
            X,X,O;
            None,None,O;
            X,O,X);
        assert_eq!(board.winning_diagonal(), false);
        let board = new_board!(
            X,    O, X;
            None, X, None;
            None, O, None
        );
        assert_eq!(board.winning_diagonal(), false);
    }
    #[test]
    fn board_creation_macro() {
        let board1 = Board {
            board: [
                Symbol::X,
                Symbol::X,
                Symbol::O,
                Symbol::None,
                Symbol::None,
                Symbol::O,
                Symbol::X,
                Symbol::O,
                Symbol::X,
            ],
        };
        let board2 = new_board!(X, X ,O;
                                None,None,O;
                                X,O,X);
        assert_eq!(board1, board2);
        let board3 = new_board!(X, X, X;
                                O, O, O;
                                None, None, None);
        let board4 = Board {
            board: [
                Symbol::X,
                Symbol::X,
                Symbol::X,
                Symbol::O,
                Symbol::O,
                Symbol::O,
                Symbol::None,
                Symbol::None,
                Symbol::None,
            ],
        };
        assert_eq!(board3, board4);
    }
    #[test]
    fn winning_board() {
        let board = new_board!(
            X, X, X;
            None, None, None;
            None, O, None
        );
        assert_eq!(board.is_winning(), true);
        let board = new_board!(
            O, X, O;
            None, O, None;
            None, O, O
        );
        assert_eq!(board.is_winning(), true);
        let board = new_board!(
            O, X, O;
            None, X, None;
            None, X, O
        );
        assert_eq!(board.is_winning(), true);
    }
    #[test]
    fn not_winning_board() {
        let board = new_board!(
            X,    O, X;
            None, X, None;
            None, O, None
        );
        assert_eq!(board.is_winning(), false);
    }
}
