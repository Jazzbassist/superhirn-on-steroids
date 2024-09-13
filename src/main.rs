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
    loop {
        // Show guesses without color during the game
        display_previous_guesses(&Player::Player2, &game.previous_guesses, &game.secret, false);
        
        let guess = match read_guess(&Player::Player2, game.secret.len()) {
            Some(value) => value,
            None => continue,
        };

        let (bulls, cows) = score_guess(&game.secret, &guess);
        display_message(&Player::Player2, &format!("Bulls: {}, Cows: {}", bulls, cows));
        game.add_guess(guess.clone(), (bulls, cows));

        if bulls == game.secret.len() {
            display_message(&Player::Player2, "Congratulations! You've guessed the secret.");
            display_previous_guesses(&Player::Player2, &game.previous_guesses, &game.secret, true);
            break;
        }

        // Show guesses with color after secret is updated
        display_previous_guesses(&Player::Player1, &game.previous_guesses, &game.secret, true);

        'fetch_new_secret: loop {
            let new_secret = read_new_secret(&Player::Player1);

            match game.update_secret(new_secret.clone()) {
                SecretChangeResponse::Valid => {
                    display_message(&Player::Player1, SecretChangeResponse::Valid.message());
                    break 'fetch_new_secret;
                }
                SecretChangeResponse::Invalid(msg) => {
                    display_message(&Player::Player1, &msg);
                    continue 'fetch_new_secret; // Allow Player 1 to try again
                }
            }
        }
    }
}
