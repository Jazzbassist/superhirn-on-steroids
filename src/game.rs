use crate::code::code::{BasicCode, Code};
use crate::code::pin::PinColour;
use std::io;

enum GameState {
    Start,
    DefineCode,
    GuessCode,
    Evaluate(bool),
    End
}

enum Player {
    Encoder(),
    Guesser(),
}

impl Player {
    fn encoder() -> Player {
        Player::Encoder()
    }

    fn guesser() -> Player {
        Player::Guesser()
    }
}

impl GameState {
    fn next_state_standard(&self) -> GameState {
        match *self {
            GameState::Start => GameState::DefineCode,
            GameState::DefineCode => GameState::GuessCode,
            GameState::GuessCode => GameState::Evaluate(false),
            GameState::Evaluate(correct) =>  match correct {
                true => GameState::End,
                false => GameState::GuessCode,
            },
            GameState::End => GameState::End,
        }
    }
    fn next_state_variant(&self) -> GameState {
        match *self {
            GameState::Start => GameState::DefineCode,
            GameState::DefineCode => GameState::GuessCode,
            GameState::GuessCode => GameState::Evaluate(false),
            GameState::Evaluate(correct) =>  match correct {
                true => GameState::End,
                false => GameState::DefineCode,
            },
            GameState::End => GameState::End,
        }
    }
    fn active_player(&self) -> Player {
        match *self {
            GameState::GuessCode | GameState::End => Player::guesser(),
            _ => Player::encoder()
        }
    }
}

pub struct Game {
    game_state: GameState,
    guessed_code: BasicCode,
    code_to_guess: BasicCode
}

impl Game {
    fn new() -> Game {
        Game { game_state: GameState::Start, guessed_code: BasicCode::new(), code_to_guess: BasicCode::new() }
    }

    fn display(&self) {
        match self.game_state {
            GameState::Start => println!("Start!"),
            GameState::DefineCode => println!("Enter your Code:"),
            GameState::GuessCode => println!("Enter your Guess"),
            GameState::Evaluate(_) => println!("Evaluating..."),
            GameState::End => println!("The code was guessed!"),
        }
    }

    fn fetch_input(&self) {
        match self.game_state {
            GameState::Start => io::stdin().read_line(&mut String::new()),
            GameState::DefineCode => todo!(),
            GameState::GuessCode => todo!(),
            GameState::Evaluate(_) => todo!(),
            GameState::End => todo!(),
        };
    }

    fn fetch_code() -> BasicCode {
        let mut stdin = io::stdin();
        let input = &mut String::new();
        input.clear();
        stdin.read_line(input);
        let mut mycode = BasicCode::new();
        
        match input.as_str() {
            "blue" => mycode.set_pin(0, PinColour::Blue),
            _ => {},
        };
        mycode
    }
}