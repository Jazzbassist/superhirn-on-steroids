use std::fmt::Debug;

use crate::code::feedback::*;
use crate::code::pin::PinColour;

pub (crate) const MAX_LENGTH: usize = 5;

pub (crate) trait Code {
    fn set_pin ( &mut self, at_position: usize, colour: PinColour );
    fn new () -> Self;
    fn compare (&self, guessed: &Self) -> Box<dyn Feedback>;
    fn display(&self);
}

#[derive (Debug, PartialEq, Eq)]
struct BasicCode {
    pins: Vec<PinColour>,
}

impl Code for BasicCode {
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

    fn display(&self) {
        for pin in &self.pins {
            println!("{}", pin);
        }
    }

    fn compare (&self, guessed: &BasicCode) -> Box<dyn Feedback>{

        let these = self.pins.clone();
        let those = guessed.pins.clone();

        let mut these_remaining: Vec<PinColour> = Vec::new();
        let mut those_remaining: Vec<PinColour> = Vec::new();
        
        let mut correct_positions: usize = 0;
        for pos in 0..MAX_LENGTH {
            //println!("comparing {} and {}. Equal: {}", these[pos], those[pos], these[pos] == those[pos]);
            if these[pos] == those[pos] {
                correct_positions+=1;
            }
            else {
                these_remaining.push(these[pos]);
                those_remaining.push(those[pos]);
            }
        }

        let mut correct_colours: usize = 0;
        for (_that_pos, that_colour) in those_remaining.iter_mut().enumerate() {
            for (this_pos, this_colour) in these_remaining.iter_mut().enumerate() {
                //println!("comparing {} and {}. Equal: {}", that_colour, this_colour, that_colour==this_colour);
                if that_colour == this_colour {
                    correct_colours+=1;
                    these_remaining.remove(this_pos);
                    break;
                }
            }

        }

        Box::new(BasicFeedback::new(correct_positions, correct_colours))
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

        assert_eq!(code1, code2);

        let mut code1 = BasicCode::new();
        code1.set_pin(0, PinColour::Black);
        code1.set_pin(1, PinColour::Brown);

        let mut code2 = BasicCode::new();
        code2.set_pin(0, PinColour::Black);
        code2.set_pin(1, PinColour::Blue);

        assert_ne!(code1, code2);
    }

    #[test]
    fn test_feedback() {

        let mut code1 = BasicCode::new();
        code1.set_pin(0, PinColour::Black);
        code1.set_pin(1, PinColour::Blue);

        let mut code2 = BasicCode::new();
        code2.set_pin(0, PinColour::Black);
        code2.set_pin(1, PinColour::Blue);

        let feedback = code1.compare(&code2);
        assert!(feedback.is_correct());
        feedback.display();

        let mut code1 = BasicCode::new();
        code1.set_pin(0, PinColour::Black);
        code1.set_pin(1, PinColour::Brown);
        code1.set_pin(2, PinColour::Blue);
        code1.set_pin(3, PinColour::Empty);
        code1.set_pin(4, PinColour::Empty);

        let mut code2 = BasicCode::new();
        code2.set_pin(0, PinColour::Black); //right
        code2.set_pin(1, PinColour::Blue);  //false
        code2.set_pin(2, PinColour::Empty); //right_color
        code2.set_pin(4, PinColour::Empty); //right
        code2.set_pin(3, PinColour::Empty); //right


        let feedback = code1.compare(&code2);
        assert!(!feedback.is_correct());
        feedback.display();
    }
}