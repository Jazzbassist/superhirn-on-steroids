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
            player: Player::Seeker
        }
    }

    pub fn output_state(&mut self) {
        match self.player {
            Player::Keeper => display_previous_guesses(&self.player, self.game.get_previous_guesses(), self.game.get_secret(), true),
            Player::Seeker => display_previous_guesses(&self.player, self.game.get_previous_guesses(), self.game.get_secret(), false),
        }
    }

    pub fn take_input(&mut self, input: String) {
        match self.player {
            Player::Keeper => self.attempt_change_secret(input),
            Player::Seeker => self.attempt_guess(input),
            
        }
    }

    fn attempt_change_secret(&mut self, new_secret: String) {
        let response = self.game.change_secret(new_secret);
        if response.is_valid() {
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

fn main_game_loop_by_struct() {
    let mut game_loop = GameLoop::new(read_secret(&Player::Keeper));
    game_loop.output_state();

}

pub fn old_game_loop() {
    let secret = read_secret(&Player::Keeper);

    if !secret.chars().all(|c| c.is_digit(10)) {
        println!("The secret must be composed of digits only!");
        return;
    }

    let mut game = Game::new(secret);

    main_game_loop(&mut game);
}

fn main_game_loop(game: &mut Game) {
    loop {
        display_previous_guesses(&Player::Seeker, game.get_previous_guesses(), &game.get_secret(), false);

        let guess = match read_guess(&Player::Seeker, game.get_secret_len()) {
            Some(value) => value,
            None => continue,
        };

        match game.handle_guess(guess.clone()) {
            Ok(score) => {
                display_message(&Player::Seeker, &score.display());
                if score.bulls == game.get_secret_len() {
                    display_message(&Player::Seeker, "Congratulations! You've guessed the secret.");
                    display_previous_guesses(&Player::Seeker, game.get_previous_guesses(), &game.get_secret(), true);
                    break;
                }
            },
            Err(msg) => display_message(&Player::Seeker, msg),
        }

        display_previous_guesses(&Player::Keeper, game.get_previous_guesses(), &game.get_secret(), true);
        // Handle secret change...
        'fetch_new_secret: loop {
            let new_secret = read_new_secret(&Player::Keeper);
        
            let response = game.change_secret(new_secret.clone());
            display_message(&Player::Keeper, &response.message());
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