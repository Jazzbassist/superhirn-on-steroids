use std::collections::HashMap;
use std::fmt::Debug;

use crate::code::feedback::*;
use crate::code::pin::PinColour;

pub (crate) const MAX_LENGTH: usize = 5;

pub trait Code {
    fn set_pin ( &mut self, at_position: usize, colour: PinColour );
    fn new () -> Self;
    fn compare (&self, guessed: &Self) -> Box<dyn Feedback>;
    fn display(&self);
    fn size(&self) -> usize;
}

#[derive (Debug, PartialEq, Eq)]
pub struct BasicCode {
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

        
        let mut correct_positions: usize = 0;
        for pos in 0..MAX_LENGTH {
            //println!("comparing {} and {}. Equal: {}", these[pos], those[pos], these[pos] == those[pos]);
            if these[pos] == those[pos] {
                correct_positions+=1;
            }
        }
        let mut secret_code_counts = HashMap::new();
        let mut guess_counts = HashMap::new();
        for colour in these {
            *secret_code_counts.entry(colour).or_insert(0) += 1;
        }
        for colour in those {
            *guess_counts.entry(colour).or_insert(0) += 1;
        }
        let mut correct_colours: usize = 0;
        for (color, count) in secret_code_counts.iter() {
            correct_colours += std::cmp::min(count, guess_counts.get(color).unwrap_or(&0));
        }
        correct_colours -= correct_positions;

        Box::new(BasicFeedback::new(correct_positions, correct_colours))
    }

    fn size(&self) -> usize {
        MAX_LENGTH
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
        assert_eq!(5, feedback.correct_positions());
        assert_eq!(0, feedback.correct_colours());

        let mut code1 = BasicCode::new();
        code1.set_pin(0, PinColour::Black);
        code1.set_pin(1, PinColour::Brown);
        code1.set_pin(2, PinColour::Blue);
        code1.set_pin(3, PinColour::Empty);
        code1.set_pin(4, PinColour::Empty);

        let mut code2 = BasicCode::new();
        code2.set_pin(0, PinColour::Black); //right
        code2.set_pin(1, PinColour::Blue);  //right_colour
        code2.set_pin(2, PinColour::Empty); //false
        code2.set_pin(4, PinColour::Empty); //right
        code2.set_pin(3, PinColour::Empty); //right


        let feedback = code1.compare(&code2);
        assert!(!feedback.is_correct());
        assert_eq!(3, feedback.correct_positions());
        assert_eq!(1, feedback.correct_colours());
    }
}