use crate::code;

pub (crate) trait Feedback {
    fn correct_colours(&self) -> usize;
    fn correct_positions(&self) -> usize;
    fn is_correct(&self) -> bool;
    fn display(&self);
}

pub (crate) struct BasicFeedback {
    correct_positions: usize,
    correct_colours: usize
}

impl Feedback for BasicFeedback {
    fn is_correct(&self) -> bool {
        self.correct_positions == code::code::MAX_LENGTH
    }

    fn display(&self) {
        println!("correct positions: {}, correct colours: {}", self.correct_positions, self.correct_colours)
    }

    fn correct_positions(&self) -> usize {
        self.correct_positions
    }

    fn correct_colours(&self) -> usize {
        self.correct_colours
    }
}

impl BasicFeedback {
    pub fn new(correct_positions: usize, correct_colours: usize) -> BasicFeedback {
        BasicFeedback{correct_positions: correct_positions, correct_colours: correct_colours}
    }
}