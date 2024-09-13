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

fn colored_player_name(player: &Player) -> String {
    player.colored_name()
}

pub fn display_message(player: &Player, message: &str) {
    println!("{}: {}", colored_player_name(player), message);
}

pub fn display_previous_guesses(player: &Player, previous_guesses: &[(String, (usize, usize))], secret: &str, colorify: bool) {
    println!("\n{}: Previous guesses:", colored_player_name(player));
    for (guess, (bulls, cows)) in previous_guesses {
        let guess_display = if colorify {
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
            guess.clone()
        };

        println!("{}: Guess: {}, Bulls: {}, Cows: {}", colored_player_name(player), guess_display, bulls, cows);
    }
}

pub fn read_secret(player: &Player) -> String {
    println!("{}: Enter the secret code (digits only):", colored_player_name(player));
    let mut secret = String::new();
    io::stdin().read_line(&mut secret).expect("Failed to read secret");
    secret.trim().to_string()
}

pub fn read_guess(player: &Player, length: usize) -> Option<String> {
    println!("{}: Enter your guess ({} digits):", colored_player_name(player), length);
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read input");
    let guess = guess.trim().to_string();
    if guess.len() != length {
        println!("{}: Your guess must be {} digits long!", colored_player_name(player), length);
        None
    } else {
        Some(guess)
    }
}

pub fn read_new_secret(player: &Player) -> String {
    println!("{}: Enter the new secret code (digits only):", colored_player_name(player));
    let mut new_secret = String::new();
    io::stdin().read_line(&mut new_secret).expect("Failed to read new secret");
    new_secret.trim().to_string()
}
