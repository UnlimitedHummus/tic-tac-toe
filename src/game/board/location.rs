#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Location(u8);

#[derive(Debug, PartialEq)]
pub enum LocationError {
    InvalidInput,
}

impl Location {
    pub fn get_x(&self) -> u8 {
        self.0 % 3
    }

    pub fn get_y(&self) -> u8 {
        self.0 / 3
    }
}

impl TryFrom<usize> for Location {
    type Error = LocationError;
    fn try_from(num: usize) -> Result<Self, LocationError> {
        match num {
            0..=8 => Ok(Location(num as u8)),
            _ => Err(LocationError::InvalidInput),
        }
    }
}

impl From<Location> for usize {
    fn from(location: Location) -> Self {
        return location.0 as usize;
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
    fn from() {
        let location = Location::try_from(1).unwrap();
        assert_eq!(usize::from(location), 1);
    }
    #[test]
    fn invalid_location() {
        assert_eq!(Location::try_from(9), Err(LocationError::InvalidInput));
        assert_eq!(Location::try_from(123), Err(LocationError::InvalidInput));
    }
    #[test]
    fn valid_location() {
        let location = Location::try_from(7).unwrap();
        assert_eq!(location.get_x(), 1_u8);
        assert_eq!(location.get_y(), 2_u8);
        assert_eq!(Location::try_from(1), Ok(Location(1)));
        assert_eq!(Location::try_from(0), Ok(Location(0)));
    }
    #[test]
    fn zero_location() {
        assert_eq!(Location::try_from(0), Ok(Location(0)));
    }
    #[test]
    fn get_x() {
        let location1 = Location::try_from(1).unwrap();
        let location2 = Location::try_from(3).unwrap();
        let location3 = Location::try_from(8).unwrap();
        assert_eq!(location1.get_x(), 1);
        assert_eq!(location2.get_x(), 0);
        assert_eq!(location3.get_x(), 2);
    }
    #[test]
    fn get_y() {
        let location1 = Location::try_from(1).unwrap();
        let location2 = Location::try_from(3).unwrap();
        let location3 = Location::try_from(8).unwrap();
        assert_eq!(location1.get_y(), 0);
        assert_eq!(location2.get_y(), 1);
        assert_eq!(location3.get_y(), 2);
    }
}
