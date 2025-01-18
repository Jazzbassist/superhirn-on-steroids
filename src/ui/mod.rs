pub mod terminal;

use crate::score::Score;

#[allow(dead_code)]
pub trait Ui {
    fn display_guesses(&self, guesses: &Vec<(String, Score)>);
}