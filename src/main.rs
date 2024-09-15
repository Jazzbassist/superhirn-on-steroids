// main.rs
mod game;
mod ui;

use game::*;
use ui::*;

fn main() {
    let secret = read_secret(&Player::Player1);

    if !secret.chars().all(|c| c.is_digit(10)) {
        println!("The secret must be composed of digits only!");
        return;
    }

    let mut game = Game::new(secret);

    main_game_loop(&mut game);
}

fn main_game_loop(game: &mut Game) {
    loop {display_previous_guesses(&Player::Player2, game.get_previous_guesses(), &game.get_secret(), false);

        let guess = match read_guess(&Player::Player2, game.get_secret_len()) {
            Some(value) => value,
            None => continue,
        };

        match game.handle_guess(guess.clone()) {
            Ok(score) => {
                display_message(&Player::Player2, &score.display());
                if score.bulls == game.get_secret_len() {
                    display_message(&Player::Player2, "Congratulations! You've guessed the secret.");
                    display_previous_guesses(&Player::Player2, game.get_previous_guesses(), &game.get_secret(), true);
                    break;
                }
            },
            Err(msg) => display_message(&Player::Player2, msg),
        }

        display_previous_guesses(&Player::Player1, game.get_previous_guesses(), &game.get_secret(), true);
        // Handle secret change...
        'fetch_new_secret: loop {
            let new_secret = read_new_secret(&Player::Player1);
        
            match game.change_secret(new_secret.clone()) {
                SecretChangeResponse::Valid => {
                    display_message(&Player::Player1, SecretChangeResponse::Valid.message());
                    break 'fetch_new_secret;
                }
                SecretChangeResponse::Invalid(msg) => {
                    display_message(&Player::Player1, &msg);
                    continue 'fetch_new_secret;
                }
            }
        }
    }
}

