// ui.rs
use crate::game::*;
use colored::Colorize;

use std::io;
pub enum PlayerType {
    Keeper,
    Seeker,
}

impl PlayerType {
    pub fn colored_name(&self) -> String {
        match self {
            PlayerType::Keeper => "Keeper".red(),
            PlayerType::Seeker => "Seeker".green(),
        }
        .to_string()
    }
}
pub struct IO {}

impl IO {
    pub fn output(&self, message: &str) {
        println!("{}", message);
    }
}

pub struct Player {
    player_type: PlayerType,
    io: IO,
}

impl Player {
    pub fn as_str(&self) -> &str {
        match self.player_type {
            PlayerType::Keeper => "Keeper",
            PlayerType::Seeker => "Seeker",
        }
    }

    pub fn display_message(&self, message: &str) {
        self.io
            .output(format!("{}: {}", self.player_type.colored_name(), message).as_str());
    }

    pub fn read_input(&self) -> String {
        self.display_message(&match self.player_type {
            PlayerType::Keeper => "Enter the new secret code (digits only):",
            PlayerType::Seeker => "Enter your guess:",
        });
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read new secret");
        input.trim().to_string()
    }

    pub fn display_guesses(&self, guesses: &Vec<(String, Score)>) {
        let formatted = [
            vec!["Previous Guesses:".to_string()],
            format_guesses(&guesses),
        ]
        .concat();
        self.display_message(&formatted.join("\n\t"));
    }

    pub fn display_guesses_colorified(&self, guesses: &Vec<(String, Score)>, secret: &str) {
        let colorified = colorify_guesses(guesses, secret);
        self.display_guesses(&colorified);
    }
}
//nonplayer?
fn colorify_guess(guess: &str, secret: &str) -> String {
    let mut display = String::new();
    for (s_char, g_char) in secret.chars().zip(guess.chars()) {
        if s_char == g_char {
            display.push_str(&g_char.to_string().green().to_string());
        } else if secret.contains(g_char) {
            display.push_str(&g_char.to_string().yellow().to_string());
        } else {
            display.push(g_char);
        }
    }
    display
}

fn colorify_guesses(guesses: &Vec<(String, Score)>, secret: &str) -> Vec<(String, Score)> {
    guesses
        .iter()
        .map(|(guess, score)| (colorify_guess(guess, secret), score.clone()))
        .collect()
}

fn format_guesses(guesses: &Vec<(String, Score)>) -> Vec<String> {
    guesses
        .iter()
        .map(|(guess, score)| format!("{}, {}", guess, &score.display()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_format_guess_for_display_with_color() {
        let secret = "1234";
        let guess = "1243";
        let result = colorify_guess(guess, secret);
        assert_eq!(
            result,
            "\u{1b}[32m1\u{1b}[0m\u{1b}[32m2\u{1b}[0m\u{1b}[33m4\u{1b}[0m\u{1b}[33m3\u{1b}[0m"
        );
    }
}
