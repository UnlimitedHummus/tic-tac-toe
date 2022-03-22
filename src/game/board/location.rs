#[derive(PartialEq, Debug)]
pub struct Location(u8);

#[derive(Debug, PartialEq)]
pub enum LocationError{
    InvalidInput
}

impl Location {
    pub fn from(n: usize) -> Result<Self, LocationError> {
        match n {
            0..=8 => Ok(Location(n as u8)),
            _ => Err(LocationError::InvalidInput)
        }
    }

    pub fn get_x(&self) -> u8 {
        self.0 % 3
    }

    pub fn get_y(&self) -> u8 {
       self.0 / 3
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // 0|1|2
    // 3|4|5
    // 6|7|8
    //
    #[test]
    fn invalid_location() {
        assert_eq!(Location::from(9), Err(LocationError::InvalidInput));
        assert_eq!(Location::from(123), Err(LocationError::InvalidInput));
    }
    #[test]
    fn valid_location() {
        let location = Location::from(7).unwrap();
        assert_eq!(location.get_x(), 1_u8);
        assert_eq!(location.get_y(), 2_u8);
        assert_eq!(Location::from(1), Ok(Location(1)));
        assert_eq!(Location::from(0), Ok(Location(0)));
    }
    #[test]
    fn zero_location() {
        assert_eq!(Location::from(0), Ok(Location(0)));
    }
    #[test]
    fn get_x() {
        let location1 = Location::from(1);
        let location2 = Location::from(3);
        let location3 = Location::from(8);
        assert_eq!(location1.unwrap().get_x(), 1);
        assert_eq!(location2.unwrap().get_x(),0);
        assert_eq!(location3.unwrap().get_x(), 2);

    }
    #[test]
    fn get_y() {
        let location1 = Location::from(1);
        let location2 = Location::from(3);
        let location3 = Location::from(8);
        assert_eq!(location1.unwrap().get_y(), 0);
        assert_eq!(location2.unwrap().get_y(), 1);
        assert_eq!(location3.unwrap().get_y(), 2);
    }
}
