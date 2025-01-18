pub mod terminaluser;

use crate::score::Score;

#[allow(dead_code)]
pub trait Ui {
    fn display_guesses(&self, guesses: &Vec<(String, Score)>);
    fn display_guesses_with_info(&self, guesses: &Vec<(String, Score)>, secret: &str);
    fn display_message(&self, message: &str);
    fn read_input(&self) -> String;
}