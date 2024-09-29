use crate::game::*;
use crate::ui::*;

#[allow(dead_code)]
pub enum Variant {
    Classic,
    ChangeSecret,
    Curtail,
}

#[allow(dead_code)]
pub struct GameLoop<GAME: Game> {
    pub player: Player,
    pub is_over: bool,
    pub game: GAME,
}

impl GameLoop<GameStruct> {
    pub fn new_classic() -> GameLoop<GameStruct> {
        let game = GameStruct::new();
        GameLoop {
            player: Player::Keeper,
            is_over: false,
            game,
        }
    }
}

impl GameLoop<BufferGame> {
    pub fn new_buffer() -> GameLoop<BufferGame> {
        let game = BufferGame::new();
        GameLoop {
            player: Player::Keeper,
            is_over: false,
            game,
        }
    }
}

impl<GAME: Game> GameLoop<GAME> {
    pub fn prompt_input(&mut self) -> String {
        self.player.read_input()
    }

    pub fn take_input(&mut self, input: &str) {
        match self.player {
            Player::Keeper => self.attempt_change_secret(input),
            Player::Seeker => self.do_guess(input),
        }
    }

    fn do_guess(&mut self, new_guess: &str) {
        self.attempt_guess(new_guess);
    }

    fn switch_player(&mut self) {
        match self.player {
            Player::Keeper => self.player = Player::Seeker,
            Player::Seeker => self.player = Player::Keeper,
        }
        self.print_state();
    }

    fn print_state(&self) {
        self.player
            .display_guesses(self.game.get_previous_guesses());
    }

    fn attempt_change_secret(&mut self, new_secret: &str) {
        let result = self.game.change_secret(new_secret);
        match result {
            Ok(()) => {
                self.handle_successful_secret_change(new_secret);
            }
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

    fn handle_successful_secret_change(&mut self, _new_secret: &str) {
        self.switch_player();
    }

    // fn buffer_guess(&mut self, new_guess: &str) {
    //     if self.game.validate_guess(new_guess).is_ok() {
    //         self.guess_buffer = new_guess.to_string();
    //         self.switch_player();
    //     }
    // }

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

    pub fn run(&mut self) {
        while !self.is_over {
            let input = &self.prompt_input();
            self.take_input(&input);
        }
    }
}

#[cfg(test)]
mod tests {
    use colored::Colorize;

    use super::*;
    #[test]
    pub fn run_game() {
        let mut gameloop = GameLoop::new(Variant::ChangeSecret);
        let inputs = [
            "1234", //init secret
            "123",  //Guess: to short
            "1235", //Guess
            "1236", //Change Code
            "1237", //Guess
            "123",  //Change Code, to short
            "9210", //Change Code, invalid
            "1231", //Change Code
            "1231", //Guess, correct
        ];
        for input in inputs {
            assert!(!gameloop.is_over);
            println!("{}: {}", "INPUT".yellow(), input);
            gameloop.take_input(&input);
        }
        assert!(gameloop.is_over);
    }
}
