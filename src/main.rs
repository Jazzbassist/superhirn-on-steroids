// main.rs
mod game;
mod gameloop;
mod ui;

use game::*;
use gameloop::*;

fn main() {
    struct_game_loop()
}

fn struct_game_loop() {
    let mut gameloop = GameLoop::new(Variant::ChangeSecret);
    while !gameloop.is_over {
        let input = &gameloop.prompt_input();
        gameloop.take_input(&input);
    }
}

#[cfg(test)]
mod tests {
    use colored::Colorize;

    use super::*;
    #[test]
    pub fn run_game() {
        let secret = "1234".to_string();
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
