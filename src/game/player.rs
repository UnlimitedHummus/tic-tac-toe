use std::ops::Not;
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Player {
    X,
    O
}

impl Not for Player {
    type Output=Self;

    fn not(self) -> Self::Output {
        match self{
            Self::O => Self::X,
            Self::X => Self::O,
        }
    }
}