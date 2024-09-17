// ui.rs
use crate::game::*;
use colored::Colorize;
use std::io;

pub enum Player {
    Keeper,
    Seeker,
}

impl Player {
    fn as_str(&self) -> &str {
        match self {
            Player::Keeper => "Keeper",
            Player::Seeker => "Seeker",
        }
    }

    fn colored_name(&self) -> String {
        match self {
            Player::Keeper => self.as_str().green(),
            Player::Seeker => self.as_str().red(),
        }
        .to_string()
    }
}

pub fn format_guess_for_display(guess: &str, secret: &str) -> String {
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

pub fn format_mismatch_feedback(mismatches: &Vec<(String, Score)>, secret: &str) -> String {
    let mut feedback = "New secret does not match the score for these guesses:\n".to_string();
    for (guess, score) in mismatches {
        let formatted_guess = format_guess_for_display(guess, secret);
        feedback.push_str(&format!(
            "Guess: {}, Expected {} bulls and {} cows\n",
            formatted_guess, score.bulls, score.cows
        ));
    }
    feedback
}

pub fn display_message(player: &Player, message: &str) {
    println!("{}: {}", player.colored_name(), message);
}

pub fn display_previous_guesses(
    player: &Player,
    previous_guesses: &[(String, Score)],
    secret: &str,
    colorify: bool,
) {
    display_message(player, "Previous guesses:");
    for (guess, score) in previous_guesses {
        let guess_display = if colorify {
            format_guess_for_display(guess, secret)
        } else {
            guess.to_string()
        };
        display_message(
            player,
            &format!(
                "Guess: {}, Bulls: {}, Cows: {}",
                guess_display, score.bulls, score.cows
            ),
        );
    }
}

pub fn read_input(player: &Player) -> String {
    display_message(
        player,
        &match player {
            Player::Keeper => "Enter the new secret code (digits only):",
            Player::Seeker => "Enter your guess:",
        },
    );
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read new secret");
    input.trim().to_string()
}

// ui.rs

// Existing code...

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_format_guess_for_display_with_color() {
        let secret = "1234";
        let guess = "1243";
        let result = format_guess_for_display(guess, secret);
        assert_eq!(
            result,
            "\u{1b}[32m1\u{1b}[0m\u{1b}[32m2\u{1b}[0m\u{1b}[33m4\u{1b}[0m\u{1b}[33m3\u{1b}[0m"
        );
    }
}
