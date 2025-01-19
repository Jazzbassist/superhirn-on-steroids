// ui.rs
use colored::Colorize;
use std::io;

use super::Ui;
use super::Score;
use super::UiBehavior;


pub struct TerminalUi {
    ui_behavior: UiBehavior
}

impl Ui for TerminalUi {
    fn display_guesses(&self, guesses: &Vec<(String, Score)>) {
        let formatted = [
            vec!["Previous Guesses:".to_string()],
            format_guesses(&guesses),
        ]
        .concat();
        self.display_message(&formatted.join("\n\t"));
    }

    fn display_guesses_with_info(&self, guesses: &Vec<(String, Score)>, new_secret: &str) {
        let colorified = colorify_guesses(guesses, new_secret);
        self.display_guesses(&colorified);
    }

    fn display_message(&self, message: &str) {
        println!("{}: {}", self.colored_name(), message);
    }

    fn read_input(&self) -> String {
        self.display_message(&match self.ui_behavior {
            UiBehavior::Informed => "Enter the new secret code (digits only):",
            UiBehavior::Ignorant => "Enter your guess:",
        });
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read new secret");
        input.trim().to_string()
    }

    fn display_score(&self, score:&Score) {
        format_score(&score);
    }
}

impl TerminalUi {
    pub fn new(ui_behavior: UiBehavior) -> TerminalUi {
        TerminalUi {
            ui_behavior,
        }
    }

    fn as_str(&self) -> &str {
        match self.ui_behavior {
            UiBehavior::Informed => "Keeper",
            UiBehavior::Ignorant => "Seeker",
        }
    }

    fn colored_name(&self) -> String {
        match self.ui_behavior {
            UiBehavior::Informed => self.as_str().green(),
            UiBehavior::Ignorant => self.as_str().red(),
        }
        .to_string()
    }
}

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
        .map(|(guess, score)| format!("{}, {}", guess, format_score(score)))
        .collect()
}

fn format_score(score: &Score) -> String {
    format!("Bulls: {}, Cows: {}", score.bulls, score.cows)
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
