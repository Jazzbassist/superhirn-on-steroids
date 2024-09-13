// main.rs
mod game;
mod display;
mod input;

use game::*;
use display::*;
use input::*;

fn main() {
    let secret = read_secret("Player 1");

    if !secret.chars().all(|c| c.is_digit(10)) {
        println!("The secret must be composed of digits only!");
        return;
    }

    let mut game = Game::new(secret);

    main_game_loop(&mut game);
}

fn main_game_loop(game: &mut Game) {
    loop {
        let guess = match read_guess("Player 2", game.secret.len()) {
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

        let new_secret = read_new_secret("Player 1");

        match game.update_secret(new_secret.clone()) {
            SecretChangeResponse::Valid => {
                display_message("Player 1", SecretChangeResponse::Valid.message());
                break;
            },
            SecretChangeResponse::Invalid(msg) => {
                display_message("Player 1", &msg);
                continue; // Allow Player 1 to try again
            },
        }
    }
}
