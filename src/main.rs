use colored::*;
use std::io;

// Define a struct to encapsulate game state
struct Game {
    secret: String,
    previous_guesses: Vec<(String, (usize, usize))>,
}

impl Game {
    fn new(secret: String) -> Self {
        Self {
            secret,
            previous_guesses: Vec::new(),
        }
    }

    fn add_guess(&mut self, guess: String, score: (usize, usize)) {
        self.previous_guesses.push((guess, score));
    }

    fn update_secret(&mut self, new_secret: String) -> SecretChangeResponse {
        if new_secret.len() != self.secret.len() {
            return SecretChangeResponse::Invalid("The new secret must be the same length as the original secret.".to_string());
        }

        if !new_secret.chars().all(|c| c.is_digit(10)) {
            return SecretChangeResponse::Invalid("The new secret must be composed of digits only.".to_string());
        }

        let mismatches = self.validate_secret(&new_secret);

        if mismatches.is_empty() {
            self.secret = new_secret;
            SecretChangeResponse::Valid
        } else {
            let feedback = format_mismatch_feedback(mismatches, &new_secret);
            SecretChangeResponse::Invalid(feedback)
        }
    }

    fn validate_secret(&self, new_secret: &str) -> Vec<(String, usize, usize, usize, usize)> {
        let mut mismatches = Vec::new();
        for (prev_guess, (prev_bulls, prev_cows)) in &self.previous_guesses {
            let (new_bulls, new_cows) = score_guess(&new_secret, prev_guess);
            if new_bulls != *prev_bulls || new_cows != *prev_cows {
                mismatches.push((prev_guess.clone(), *prev_bulls, *prev_cows, new_bulls, new_cows));
            }
        }
        mismatches
    }
}

// Enum to handle responses for secret changes
enum SecretChangeResponse {
    Valid,
    Invalid(String), // Change from &'static str to String
}

impl SecretChangeResponse {
    fn message(&self) -> &str {
        match self {
            SecretChangeResponse::Valid => "Secret updated successfully.",
            SecretChangeResponse::Invalid(msg) => &msg,
        }
    }
}


// Function to score the guess against the secret
fn score_guess(secret: &str, guess: &str) -> (usize, usize) {
    let bulls = secret
        .chars()
        .zip(guess.chars())
        .filter(|(s, g)| s == g)
        .count();
    let cows = guess.chars().filter(|g| secret.contains(*g)).count() - bulls;
    (bulls, cows)
}

// Function to print feedback with color coding
fn print_feedback(guess: &str, secret: &str) {
    for (s_char, g_char) in secret.chars().zip(guess.chars()) {
        if s_char == g_char {
            print!("{}", g_char.to_string().green());
        } else if secret.contains(g_char) {
            print!("{}", g_char.to_string().yellow());
        } else {
            print!("{}", g_char);
        }
    }
}

// Function to format mismatch feedback into a String
fn format_mismatch_feedback(
    mismatches: Vec<(String, usize, usize, usize, usize)>,
    new_secret: &str,
) -> String {
    let mut feedback = String::from("The new secret does not match the score for these guesses:\n");
    for (guess, expected_bulls, expected_cows, actual_bulls, actual_cows) in mismatches {
        feedback.push_str("Guess: ");
        let mut guess_display = String::new();
        for (s_char, g_char) in new_secret.chars().zip(guess.chars()) {
            if s_char == g_char {
                guess_display.push_str(&g_char.to_string().green().to_string());
            } else if new_secret.contains(g_char) {
                guess_display.push_str(&g_char.to_string().yellow().to_string());
            } else {
                guess_display.push(g_char);
            }
        }
        feedback.push_str(&guess_display);
        feedback.push_str(&format!(
            ", Expected {} bulls and {} cows, Found {} bulls and {} cows\n",
            expected_bulls, expected_cows, actual_bulls, actual_cows
        ));
    }
    feedback
}

// Function to read a guess from the player
fn read_guess(secret_len: usize) -> Option<String> {
    println!("Player 2, enter your guess ({} digits):", secret_len);
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read input");
    let guess = guess.trim().to_string();
    if guess.len() != secret_len {
        println!("Your guess must be {} digits long!", secret_len);
        None
    } else {
        Some(guess)
    }
}

// Function to read a new secret code
fn read_new_secret(game: &mut Game) {
    loop {
        println!("Enter the new secret code (digits only):");
        let mut new_secret = String::new();
        io::stdin()
            .read_line(&mut new_secret)
            .expect("Failed to read new secret");

        let new_secret = new_secret.trim().to_string();

        match game.update_secret(new_secret.clone()) {
            SecretChangeResponse::Valid => {
                println!("{}", SecretChangeResponse::Valid.message());
                break;
            },
            SecretChangeResponse::Invalid(msg) => {
                println!("{}", msg);
            },
        }
    }
}

// Main game loop function
fn main_game_loop(game: &mut Game) {
    loop {
        let guess = match read_guess(game.secret.len()) {
            Some(value) => value,
            None => continue,
        };

        let (bulls, cows) = score_guess(&game.secret, &guess);
        println!("Bulls: {}, Cows: {}", bulls, cows);
        game.add_guess(guess.clone(), (bulls, cows));

        // Display previous guesses
        println!("\nPrevious guesses:");
        for (g, (b, c)) in &game.previous_guesses {
            print!("Guess: ");
            print_feedback(g, &game.secret);
            println!(", Bulls: {}, Cows: {}", b, c);
        }

        if bulls == game.secret.len() {
            println!("Congratulations! You've guessed the secret.");
            break;
        }

        // Read and apply new secret without asking for confirmation
        read_new_secret(game);
    }
}

// Entry point for the application
fn main() {
    // Player 1 enters the secret code, which is now visible
    println!("Player 1, enter the secret code (digits only):");

    // Read the secret code input as plain text
    let mut secret = String::new();
    io::stdin()
        .read_line(&mut secret)
        .expect("Failed to read secret");

    // Trim any extra whitespace or newline from the input
    let secret = secret.trim().to_string();

    // Ensure the secret is composed only of digits
    if !secret.chars().all(|c| c.is_digit(10)) {
        println!("The secret must be composed of digits only!");
        return;
    }

    let mut game = Game::new(secret);

    main_game_loop(&mut game);
}
