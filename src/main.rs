// main.rs
mod game;
mod display;

use game::*;
use display::*;
use std::io;

fn main() {
    println!("Player 1, enter the secret code (digits only):");

    let mut secret = String::new();
    io::stdin()
        .read_line(&mut secret)
        .expect("Failed to read secret");

    let secret = secret.trim().to_string();

    if !secret.chars().all(|c| c.is_digit(10)) {
        println!("The secret must be composed of digits only!");
        return;
    }

    let mut game = Game::new(secret);

    main_game_loop(&mut game);
}

fn main_game_loop(game: &mut Game) {
    loop {
        let guess = match read_guess(game.secret.len()) {
            Some(value) => value,
            None => continue,
        };

        let (bulls, cows) = score_guess(&game.secret, &guess);
        display_message("Player 2", &format!("Bulls: {}, Cows: {}", bulls, cows));
        game.add_guess(guess.clone(), (bulls, cows));

        display_previous_guesses("Player 2", &game.previous_guesses, &game.secret);

        if bulls == game.secret.len() {
            display_message("Player 2", "Congratulations! You've guessed the secret.");
            break;
        }

        read_new_secret(game);
    }
}

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
                display_message("Player 1", SecretChangeResponse::Valid.message());
                break;
            },
            SecretChangeResponse::Invalid(msg) => {
                display_message("Player 1", &msg);
            },
        }
    }
}
