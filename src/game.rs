enum GameState {
    Start,
    DefineCode,
    GuessCode,
    Evaluate(bool),
    End
}

enum Player {
    Encoder,
    Guesser,
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
            GameState::GuessCode | GameState::End => Player::Guesser,
            _ => Player::Encoder
        }
    }
}