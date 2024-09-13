// display.rs
use colored::Colorize;

fn colored_player_name(player: &str) -> String {
    if player == "Player 1" {
        player.green()
    } else {
        player.red()
    }.to_string()
}

pub fn display_message(player: &str, message: &str) {
    println!("{}: {}", colored_player_name(player), message);
}

pub fn display_previous_guesses(player: &str, previous_guesses: &[(String, (usize, usize))], secret: &str) {
    println!("\n{}: Previous guesses:", colored_player_name(player));
    for (guess, (bulls, cows)) in previous_guesses {
        let mut guess_display = String::new();
        for (s_char, g_char) in secret.chars().zip(guess.chars()) {
            if s_char == g_char {
                guess_display.push_str(&g_char.to_string().green().to_string());
            } else if secret.contains(g_char) {
                guess_display.push_str(&g_char.to_string().yellow().to_string());
            } else {
                guess_display.push(g_char);
            }
        }
        println!("{}: Guess: {}, Bulls: {}, Cows: {}", colored_player_name(player), guess_display, bulls, cows);
    }
}
