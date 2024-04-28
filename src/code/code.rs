use crate::code::pin::PinColour;

#[derive (Debug, PartialEq, Eq)]
struct BasicCode {
    pins: Vec<PinColour>
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_equality() {
        let mut expected = BasicCode{pins: Vec::new()};
        expected.pins.push(PinColour::Black);
        expected.pins.push(PinColour::Blue);
    }
}