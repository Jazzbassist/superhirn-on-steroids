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
    pub fn take_input(&mut self, input: String) {
        match self.player {
            Player::Player1 => self.attempt_change_secret(input),
            Player::Player2 => self.attempt_guess(input),
            
        }
    }

    fn attempt_change_secret(&mut self, new_secret: String) {
        let response = self.game.change_secret(new_secret);
        if response.is_valid() {
            self.player = Player::Player2;
        }
    }

    fn attempt_guess(&mut self, new_guess: String) {
        let result = self.game.handle_guess(new_guess);
        match result {
            Ok(_) => self.player = Player::Player1,
            Err(_) => self.player = Player::Player2,
        }
    }
}
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
        display_previous_guesses(&Player::Player2, game.get_previous_guesses(), &game.get_secret(), false);

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
        
            let response = game.change_secret(new_secret.clone());
            display_message(&Player::Player1, &response.message());
            if response.is_valid() {
                break 'fetch_new_secret;
            }
            else {
                continue 'fetch_new_secret;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    pub fn _run_game() {

    }

}