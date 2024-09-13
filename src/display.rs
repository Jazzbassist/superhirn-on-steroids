// display.rs
use colored::*;

pub fn display_guess(player: &str, guess: &str, secret: &str) {
    println!("{} guessed: ", player);
    for (s_char, g_char) in secret.chars().zip(guess.chars()) {
        if s_char == g_char {
            print!("{}", g_char.to_string().green());
        } else if secret.contains(g_char) {
            print!("{}", g_char.to_string().yellow());
        } else {
            print!("{}", g_char);
        }
    }
    println!();
}

pub fn display_previous_guesses(player: &str, previous_guesses: &[(String, (usize, usize))], secret: &str) {
    println!("\n{}'s previous guesses:", player);
    for (g, (b, c)) in previous_guesses {
        print!("Guess: ");
        display_guess(player, g, secret);
        println!(", Bulls: {}, Cows: {}", b, c);
    }
}

pub fn display_message(player: &str, message: &str) {
    println!("{}: {}", player, message);
}
