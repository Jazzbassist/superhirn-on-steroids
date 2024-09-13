// input.rs
use std::io;
use colored::Colorize;

fn colored_player_name(player: &str) -> String {
    if player == "Player 1" {
        player.green()
    } else {
        player.red()
    }.to_string()
}

pub fn read_guess(player: &str, secret_len: usize) -> Option<String> {
    println!("{}: Enter your guess ({} digits):", colored_player_name(player), secret_len);
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read input");
    let guess = guess.trim().to_string();
    if guess.len() != secret_len {
        println!("{}: Your guess must be {} digits long!", colored_player_name(player), secret_len);
        None
    } else {
        Some(guess)
    }
}

pub fn read_secret(player: &str) -> String {
    println!("{}: Enter the secret code (digits only):", colored_player_name(player));
    let mut secret = String::new();
    io::stdin()
        .read_line(&mut secret)
        .expect("Failed to read input");
    secret.trim().to_string()
}

pub fn read_new_secret(player: &str) -> String {
    println!("{}: Enter the new secret code (digits only):", colored_player_name(player));
    let mut new_secret = String::new();
    io::stdin()
        .read_line(&mut new_secret)
        .expect("Failed to read input");
    new_secret.trim().to_string()
}