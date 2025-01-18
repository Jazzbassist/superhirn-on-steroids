pub mod terminal;

use crate::game::Score;

#[allow(dead_code)]
pub trait Ui {
    fn display_guesses(&self, guesses: &Vec<(String, Score)>);
}