use std::fmt::Debug;

use crate::code::pin::PinColour;
const MAX_LENGTH: usize = 5;

#[derive (Debug, PartialEq, Eq)]
pub(crate) struct BasicCode {
    pins: Vec<PinColour>,
}

impl BasicCode {
    fn set_pin ( &mut self, at_position: usize, colour: PinColour ){
        if at_position > MAX_LENGTH {
            panic!("Cannot set Pin at position greater than board");
        }
        else {
            self.pins[at_position] = colour;
        }
    }

    fn new () -> BasicCode {
        let mut pins = Vec::new();
        while pins.len() < MAX_LENGTH {
            pins.push(PinColour::Empty);
        }
        BasicCode{pins: pins}
    }

    fn compare (&self, other: &BasicCode) -> bool{
        self.eq(other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_equality() {
        let mut code1 = BasicCode::new();
        code1.set_pin(0, PinColour::Black);
        code1.set_pin(1, PinColour::Blue);

        let mut code2 = BasicCode::new();
        code2.set_pin(0, PinColour::Black);
        code2.set_pin(1, PinColour::Blue);

        assert!(code1.compare(&code2));

        let mut code1 = BasicCode::new();
        code1.set_pin(0, PinColour::Black);
        code1.set_pin(1, PinColour::Brown);

        let mut code2 = BasicCode::new();
        code2.set_pin(0, PinColour::Black);
        code2.set_pin(1, PinColour::Blue);

        assert!(!code1.compare(&code2));
    }
}