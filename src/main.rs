use rpassword::read_password;
use std::io;

fn score_guess(secret: &str, guess: &str) -> (usize, usize) {
    let mut bulls = 0;
    let mut cows = 0;

    // Create arrays to keep track of how many times each digit appears
    let mut secret_counts = [0; 10];
    let mut guess_counts = [0; 10];

    // First, count the bulls (correct digit, correct position)
    for (i, (s_char, g_char)) in secret.chars().zip(guess.chars()).enumerate() {
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
    // Secret input by player 1, hidden from view
    println!("Player 1, enter the secret code (digits only):");

    // Use rpassword to hide the input
    let secret = read_password().expect("Failed to read secret");

    // Ensure secret is a valid digit string
    if !secret.chars().all(|c| c.is_digit(10)) {
        println!("The secret must be composed of digits only!");
        return;
    }

    loop {
        println!("Player 2, enter your guess ({} digits):", secret.len());

        // Read user input
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");

        // Trim the guess to remove any extra whitespace or newline
        let guess = guess.trim();

        // Ensure that the guess has the correct length
        if guess.len() != secret.len() {
            println!("Your guess must be {} digits long!", secret.len());
            continue; // Ask for input again if the guess length is invalid
        }

        // Score the guess
        let (bulls, cows) = score_guess(&secret, guess);

        // Print the result
        println!("Bulls: {}, Cows: {}", bulls, cows);

        // Break the loop if the guess matches the secret exactly
        if bulls == secret.len() {
            println!("Congratulations! You've guessed the secret.");
            break;
        }
    }
}
