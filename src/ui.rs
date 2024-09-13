// ui.rs
use colored::Colorize;
use std::io;

pub enum Player {
    Player1,
    Player2,
}

impl Player {
    fn as_str(&self) -> &str {
        match self {
            Player::Player1 => "Player 1",
            Player::Player2 => "Player 2",
        }
    }

    fn colored_name(&self) -> String {
        match self {
            Player::Player1 => self.as_str().green(),
            Player::Player2 => self.as_str().red(),
        }.to_string()
    }
}

pub fn format_guess_for_display(guess: &str, secret: &str, colorify: bool) -> String {
    if colorify {
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
    } else {
        guess.to_string()
    }
}

pub fn format_mismatch_feedback(mismatches: &[(String, (usize, usize))], secret: &str) -> String {
    let mut feedback = "New secret does not match the score for these guesses:\n".to_string();
    for (guess, (expected_bulls, expected_cows)) in mismatches {
        let formatted_guess = format_guess_for_display(guess, secret, true);
        feedback.push_str(&format!(
            "Guess: {}, Expected {} bulls and {} cows\n",
            formatted_guess, expected_bulls, expected_cows
        ));
    }
    feedback
}

pub fn display_message(player: &Player, message: &str) {
    println!("{}: {}", player.colored_name(), message);
}

pub fn display_previous_guesses(player: &Player, previous_guesses: &[(String, (usize, usize))], secret: &str, colorify: bool) {
    println!("\n{}: Previous guesses:", player.colored_name());
    for (guess, (bulls, cows)) in previous_guesses {
        let guess_display = format_guess_for_display(guess, secret, colorify);
        println!("{}: Guess: {}, Bulls: {}, Cows: {}", player.colored_name(), guess_display, bulls, cows);
    }
}

pub fn read_secret(player: &Player) -> String {
    println!("{}: Enter the secret code (digits only):", player.colored_name());
    let mut secret = String::new();
    io::stdin().read_line(&mut secret).expect("Failed to read secret");
    secret.trim().to_string()
}

pub fn read_guess(player: &Player, length: usize) -> Option<String> {
    println!("{}: Enter your guess ({} digits):", player.colored_name(), length);
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read input");
    let guess = guess.trim().to_string();
    if guess.len() != length {
        println!("{}: Your guess must be {} digits long!", player.colored_name(), length);
        None
    } else {
        Some(guess)
    }
}

pub fn read_new_secret(player: &Player) -> String {
    println!("{}: Enter the new secret code (digits only):", player.colored_name());
    let mut new_secret = String::new();
    io::stdin().read_line(&mut new_secret).expect("Failed to read new secret");
    new_secret.trim().to_string()
}
