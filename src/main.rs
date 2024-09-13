use std::io;
use colored::*;  // Import the colored crate

// Score guess function (for reference, unchanged)
fn score_guess(secret: &str, guess: &str) -> (usize, usize) {
    let bulls = secret.chars().zip(guess.chars()).filter(|(s, g)| s == g).count();
    let cows = guess
        .chars()
        .filter(|g| secret.contains(*g))
        .count() - bulls;
    (bulls, cows)
}

fn main() {
    // Player 1 enters the secret code, which is now visible
    println!("Player 1, enter the secret code (digits only):");

    // Read the secret code input as plain text
    let mut secret = String::new();
    io::stdin().read_line(&mut secret).expect("Failed to read secret");

    // Trim any extra whitespace or newline from the input
    let secret = secret.trim().to_string();

    // Ensure the secret is composed only of digits
    if !secret.chars().all(|c| c.is_digit(10)) {
        println!("The secret must be composed of digits only!");
        return;
    }

    // Vector to store previous guesses and their scores
    let mut previous_guesses: Vec<(String, (usize, usize))> = Vec::new();

    // Main game loop with Player 2 guessing
    loop {
        println!("Player 2, enter your guess ({} digits):", secret.len());

        // Read Player 2's guess
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read input");

        // Trim the guess input
        let guess = guess.trim().to_string();

        // Ensure the guess has the same length as the secret
        if guess.len() != secret.len() {
            println!("Your guess must be {} digits long!", secret.len());
            continue;  // If invalid, prompt again
        }

        // Score the guess
        let (bulls, cows) = score_guess(&secret, &guess);

        // Display result (only bulls and cows count for now)
        println!("Bulls: {}, Cows: {}", bulls, cows);

        // Store the guess and its score in the history
        previous_guesses.push((guess.clone(), (bulls, cows)));

        // Display previous guesses without any color
        println!("\nPrevious guesses:");
        for (g, (b, c)) in &previous_guesses {
            println!("Guess: {}, Bulls: {}, Cows: {}", g, b, c);
        }

        // Check if the guess is correct (all bulls)
        if bulls == secret.len() {
            println!("Congratulations! You've guessed the secret.");
            break;
        }
    }

    // After the game is won, display detailed feedback for all previous guesses
    println!("\nFinal Feedback:");
    for (guess, _) in &previous_guesses {
        print!("Guess: ");
        for (s_char, g_char) in secret.chars().zip(guess.chars()) {
            if s_char == g_char {
                // Bulls: correct digit and correct position (green)
                print!("{}", g_char.to_string().green());
            } else if secret.contains(g_char) {
                // Cows: correct digit, wrong position (yellow)
                print!("{}", g_char.to_string().yellow());
            } else {
                // Incorrect digit: leave uncolored
                print!("{}", g_char);
            }
        }
        println!();  // Newline after each guess feedback
    }
}