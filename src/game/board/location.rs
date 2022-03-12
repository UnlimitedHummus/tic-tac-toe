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
