#[derive(PartialEq, Debug)]
pub struct Location(u8);

#[derive(Debug, PartialEq)]
pub enum LocationError{
    InvalidInput
}

impl Location {
    pub fn new(x: u8, y: u8) -> Result<Self, LocationError> {
        Location::valid_location(x, y)?;
        Ok(Location(x + 3*y))
    }

    pub fn from(n: usize) -> Result<Self, LocationError> {
        match n {
            0 => Location::new(0, 0),
            1 => Location::new(1, 0),
            2 => Location::new(2, 0),
            3 => Location::new(0, 1),
            4 => Location::new(1, 1),
            5 => Location::new(2, 1),
            6 => Location::new(0, 2),
            7 => Location::new(1, 2),
            8 => Location::new(2, 2),
            _ => Err(LocationError::InvalidInput),
        }
    }

    fn valid_location(x: u8, y: u8) -> Result<(), LocationError> {
        match (x, y) {
            (x, y) if x < 3 && y < 3 => Ok(()),
            (_, _) => return Err(LocationError::InvalidInput),
        }
    }

    pub fn get_x(&self) -> u8 {
        self.0 %3
    }

    pub fn get_y(&self) -> u8 {
        self.0 /3
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn invalid_location() {
        assert_eq!(Location::new(3, 2), Err(LocationError::InvalidInput));
        assert_eq!(Location::new(1, 4), Err(LocationError::InvalidInput));
        assert_eq!(Location::from(9), Err(LocationError::InvalidInput));
    }
    #[test]
    fn valid_location() {
        let location = Location::new(1, 2).unwrap();
        assert_eq!(location.get_x(), 1_u8);
        assert_eq!(location.get_y(), 2_u8);
        assert_eq!(Location::from(1), Ok(Location(1)))
    }
    #[test]
    fn zero_location() {
        assert_eq!(Location::new(0, 0), Ok(Location(0)));
    }
}
