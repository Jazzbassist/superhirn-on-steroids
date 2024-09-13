use colored::*;
use std::io; // Import the colored crate

fn main() {
    // Player 1 enters the secret code, which is now visible
    println!("Player 1, enter the secret code (digits only):");

    // Read the secret code input as plain text
    let mut secret = String::new();
    io::stdin()
        .read_line(&mut secret)
        .expect("Failed to read secret");

    // Trim any extra whitespace or newline from the input
    let mut secret = secret.trim().to_string();

    // Ensure the secret is composed only of digits
    if !secret.chars().all(|c| c.is_digit(10)) {
        println!("The secret must be composed of digits only!");
        return;
    }

    // Vector to store previous guesses and their scores
    let mut previous_guesses: Vec<(String, (usize, usize))> = Vec::new();

    main_game_loop(&mut secret, &mut previous_guesses);

    final_feedback(&previous_guesses, &secret);
}

fn main_game_loop(secret: &mut String, previous_guesses: &mut Vec<(String, (usize, usize))>) {
    loop {
        // Main game loop with Player 2 guessing
        let guess = match read_guess(&secret) {
            Some(value) => value,
            None => continue,
        };

        let (bulls, cows) = score_guess(&secret, &guess);
        println!("Bulls: {}, Cows: {}", bulls, cows);
        previous_guesses.push((guess.clone(), (bulls, cows)));

        // Display previous guesses
        println!("\nPrevious guesses:");
        for (g, (b, c)) in &*previous_guesses {
            println!("Guess: {}, Bulls: {}, Cows: {}", g, b, c);
        }

        if bulls == secret.len() {
            println!("Congratulations! You've guessed the secret.");
            break;
        }

        let response = ask_to_change_secret();

        if response == "yes" {
            read_new_secret(secret, &previous_guesses);
        }
    }
}

fn read_guess(secret: &str) -> Option<String> {
    println!("Player 2, enter your guess ({} digits):", secret.len());
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read input");
    let guess = guess.trim().to_string();
    if guess.len() != secret.len() {
        println!("Your guess must be {} digits long!", secret.len());
        return None;
    }
    Some(guess)
}

fn read_new_secret(secret: &mut String, previous_guesses: &Vec<(String, (usize, usize))>) {
    loop {
        println!("Enter the new secret code (digits only):");
        let mut new_secret = String::new();
        io::stdin()
            .read_line(&mut new_secret)
            .expect("Failed to read new secret");

        let new_secret = new_secret.trim().to_string();

        if new_secret.len() != secret.len() {
            println!("The new secret must be the same length as the original secret.");
            continue;
        }

        if !new_secret.chars().all(|c| c.is_digit(10)) {
            println!("The new secret must be composed of digits only!");
            continue;
        }

        // Validate the new secret against previous guesses and provide feedback
        let mut valid = true;
        for (prev_guess, (prev_bulls, prev_cows)) in previous_guesses {
            let (new_bulls, new_cows) = score_guess(&new_secret, prev_guess);
            if new_bulls != *prev_bulls || new_cows != *prev_cows {
                println!(
                    "New secret does not match the score for guess: {}",
                    prev_guess
                );
                valid = false;
                break;
            }
        }

        if valid {
            // Update the secret
            *secret = new_secret;
            break;
        } else {
            println!("The new secret was invalid. Here is the detailed feedback:");
            final_feedback(previous_guesses, &new_secret);
        }
    }
}

fn ask_to_change_secret() -> String {
    // Allow Player 1 to change the secret
    println!("Would you like to change the secret? (yes/no):");
    let mut response = String::new();
    io::stdin()
        .read_line(&mut response)
        .expect("Failed to read input");
    let response = response.trim().to_lowercase();
    response
}

fn final_feedback(previous_guesses: &Vec<(String, (usize, usize))>, secret: &str) {
    // Display final feedback
    println!("\nDetailed Feedback:");
    for (guess, _) in previous_guesses {
        print!("Guess: ");
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
}

// Score guess function (for reference, unchanged)
fn score_guess(secret: &str, guess: &str) -> (usize, usize) {
    let bulls = secret
        .chars()
        .zip(guess.chars())
        .filter(|(s, g)| s == g)
        .count();
    let cows = guess.chars().filter(|g| secret.contains(*g)).count() - bulls;
    (bulls, cows)
}
