// main.rs
mod game;
mod ui;

use game::*;
use ui::*;

struct GameLoop {
    pub game: Game,
    pub player: Player,
    pub is_over: bool,
}

#[allow(dead_code)]
impl GameLoop {
    pub fn new(secret: String) -> GameLoop {
        GameLoop {
            game: Game::new(secret),
            player: Player::Seeker,
            is_over: false,
        }
    }

    fn switch_player(&mut self) {
        match self.player {
            Player::Keeper => self.player = Player::Seeker,
            Player::Seeker => self.player = Player::Keeper,
        }
        self.print_state();
    }

    pub fn prompt_input(&mut self) -> String {
        self.player.read_input()
    }

    fn print_state(&self) {
        self.player
            .display_guesses(self.game.get_previous_guesses());
    }

    pub fn take_input(&mut self, input: &str) {
        match self.player {
            Player::Keeper => self.attempt_change_secret(input),
            Player::Seeker => self.attempt_guess(input),
        }
    }

    fn attempt_change_secret(&mut self, new_secret: &str) {
        let result = self.game.change_secret(new_secret);
        match result {
            Ok(()) => self.switch_player(),
            Err(response) => {
                self.player.display_message(&response.message());
                match response {
                    ErrResponse::GuessMismatch(guesses) => self
                        .player
                        .display_guesses_colorified(&guesses, &new_secret),
                    _ => (),
                }
            }
        }
    }

    fn attempt_guess(&mut self, new_guess: &str) {
        let result = self.game.handle_guess(new_guess);
        match result {
            Ok(score) => {
                self.player.display_message(&score.display());
                if score.bulls == self.game.get_secret_len() {
                    self.player
                        .display_message("Congratulations! You've guessed the secret.");
                    self.player.display_guesses_colorified(
                        self.game.get_previous_guesses(),
                        &self.game.get_secret(),
                    );
                    self.is_over = true;
                }
                self.switch_player();
            }
            Err(msg) => {
                self.player.display_message(msg);
            }
        }
    }
}

fn main() {
    struct_game_loop()
}

fn struct_game_loop() {
    let secret = Player::Keeper.read_input();
    let mut gameloop = GameLoop::new(secret);
    while !gameloop.is_over {
        let input = &gameloop.prompt_input();
        gameloop.take_input(&input);
    }
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
        match game.handle_guess(&guess) {
            Ok(score) => {
                player.display_message(&score.display());
                if score.bulls == game.get_secret_len() {
                    player.display_message("Congratulations! You've guessed the secret.");
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

        player.display_guesses_colorified(game.get_previous_guesses(), &game.get_secret());
        // Handle secret change...
        'fetch_new_secret: loop {
            let new_secret = player.read_input();

            let result = game.change_secret(&new_secret);
            match result {
                Ok(()) => break 'fetch_new_secret,
                Err(response) => {
                    player.display_message(&response.message());
                    match response {
                        ErrResponse::GuessMismatch(guesses) => {
                            player.display_guesses_colorified(&guesses, &new_secret)
                        }
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
    use colored::Colorize;

    use super::*;
    #[test]
    pub fn run_game() {
        let secret = "1234".to_string();
        let mut gameloop = GameLoop::new(secret);
        let inputs = [
            "123", "1235", "1236", "1237", "123", "9210", "1231", "1231",
        ];
        for input in inputs {
            assert!(!gameloop.is_over);
            println!("{}: {}", "INPUT".yellow(), input);
            gameloop.take_input(&input);
        }
        assert!(gameloop.is_over);
    }
}
