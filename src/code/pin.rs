#[derive (PartialEq, Eq, Debug)]
#[allow(dead_code)]
enum PinColour {
    Blue,
    Green,
    Yellow,
    Orange,
    Red,
    Purple,
    Black,
    White,
    Brown,
    
    Empty
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_equality() {
        assert_eq!(PinColour::Blue, PinColour::Blue);
        assert_ne!(PinColour::Blue, PinColour::Green);
    }
}
