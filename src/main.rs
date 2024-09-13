use std::io;

fn score_guess(secret: &str, guess: &str) -> (usize, usize) {
    let mut bulls = 0;
    let mut cows = 0;

    // Create arrays to keep track of how many times each digit appears
    let mut secret_counts = [0; 10];
    let mut guess_counts = [0; 10];

    // First, count the bulls (correct digit, correct position)
    for (_i, (s_char, g_char)) in secret.chars().zip(guess.chars()).enumerate() {
        if s_char == g_char {
            bulls += 1;
        } else {
            // Convert chars to digits, increase the count
            let s_digit = s_char.to_digit(10).unwrap();
            let g_digit = g_char.to_digit(10).unwrap();
            secret_counts[s_digit as usize] += 1;
            guess_counts[g_digit as usize] += 1;
        }
    }

    // Now count the cows (correct digit, wrong position)
    for i in 0..10 {
        cows += std::cmp::min(secret_counts[i], guess_counts[i]);
    }

    (bulls, cows)
}

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

    // Vector to store previous guesses and their scores
    let mut previous_guesses: Vec<(String, (usize, usize))> = Vec::new();

    // Main game loop with Player 2 guessing
    loop {
        println!("Player 2, enter your guess ({} digits):", secret.len());

        // Read Player 2's guess
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");

        // Trim the guess input
        let guess = guess.trim().to_string();

        // Ensure the guess has the same length as the secret
        if guess.len() != secret.len() {
            println!("Your guess must be {} digits long!", secret.len());
            continue; // If invalid, prompt again
        }

        // Score the guess
        let (bulls, cows) = score_guess(&secret, &guess);

        // Display result
        println!("Bulls: {}, Cows: {}", bulls, cows);

        // Store the guess and its score in the history
        previous_guesses.push((guess.clone(), (bulls, cows)));

        // Print previous guesses and their scores for Player 1's reference
        println!("Previous guesses:");
        for (g, (b, c)) in &previous_guesses {
            println!("Guess: {}, Bulls: {}, Cows: {}", g, b, c);
        }

        // Check if the guess is correct (all bulls)
        if bulls == secret.len() {
            println!("Congratulations! You've guessed the secret.");
            break;
        }
    }
}
