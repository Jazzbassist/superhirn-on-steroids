// main.rs
mod game;
mod ui;

use game::*;
use ui::*;

struct GameLoop {
    pub game: Game,
    pub player: Player,
}

#[allow(dead_code)]
impl GameLoop {
    pub fn new(secret: String) -> GameLoop {
        GameLoop {
            game: Game::new(secret),
            player: Player::Seeker,
        }
    }


    pub fn take_input(&mut self, input: String) {
        match self.player {
            Player::Keeper => self.attempt_change_secret(input),
            Player::Seeker => self.attempt_guess(input),
        }
    }

    fn attempt_change_secret(&mut self, new_secret: String) {
        let response = self.game.change_secret(&new_secret);
        if response.is_ok() {
            self.player = Player::Seeker;
        }
    }

    fn attempt_guess(&mut self, new_guess: String) {
        let result = self.game.handle_guess(new_guess);
        match result {
            Ok(_) => self.player = Player::Keeper,
            Err(_) => self.player = Player::Seeker,
        }
    }
}
fn main() {
    old_game_loop()
}


pub fn old_game_loop() {
    let secret = Player::Keeper.read_input();

    if !secret.chars().all(|c| c.is_digit(10)) {
        println!("The secret must be composed of digits only!");
        return;
    }

    let mut game = Game::new(secret);

    main_game_loop(&mut game);
}

fn main_game_loop(game: &mut Game) {
    loop {
        let player = Player::Seeker;
        player.display_guesses(game.get_previous_guesses());

        let guess = player.read_input();
        match game.handle_guess(guess) {
            Ok(score) => {
                player.display_message(&score.display());
                if score.bulls == game.get_secret_len() {
                    player.display_message(
                        "Congratulations! You've guessed the secret.",
                    );
                    player.display_guesses_colorified(
                        game.get_previous_guesses(),
                        &game.get_secret(),
                    );
                    break;
                }
            }
            Err(msg) => {
                player.display_message(msg);
                continue;
            }
        }

        let player = Player::Keeper;

        player.display_guesses_colorified(
            game.get_previous_guesses(),
            &game.get_secret(),
        );
        // Handle secret change...
        'fetch_new_secret: loop {
            let new_secret = player.read_input();

            let result = game.change_secret(&new_secret);
            match result {
                Ok(()) => break 'fetch_new_secret,
                Err(response) => {
                    player.display_message(&response.message());
                    match response {
                        ErrResponse::GuessMismatch(guesses) => player.display_guesses_colorified(&guesses, &new_secret),
                        _ => (),
                    }
                    continue 'fetch_new_secret;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    pub fn _run_game() {}
}
