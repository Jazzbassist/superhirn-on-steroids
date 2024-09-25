// game.rs

#[derive(PartialEq, Clone, Debug)]
pub struct Score {
    pub bulls: usize,
    pub cows: usize,
}

impl Score {
    pub fn new(bulls: usize, cows: usize) -> Self {
        Score { bulls, cows }
    }

    pub fn display(&self) -> String {
        format!("Bulls: {}, Cows: {}", self.bulls, self.cows)
    }
}

pub struct Game {
    secret: String,
    previous_secrets: Vec<String>,
    previous_guesses: Vec<(String, Score)>, // Use Score instead of tuple
}

impl Game {
    pub fn new() -> Self {
        Game {
            secret: "".to_string(),
            previous_secrets: Vec::new(),
            previous_guesses: Vec::new(),
        }
    }

    pub fn get_previous_guesses(&self) -> &Vec<(String, Score)> {
        &self.previous_guesses
    }

    pub fn get_secret(&self) -> &String {
        &self.secret
    }

    pub fn get_secret_len(&self) -> usize {
        self.secret.len()
    }

    fn add_guess(&mut self, guess: String, score: Score) {
        self.previous_guesses.push((guess, score));
    }

    pub fn validate_guess(&mut self, guess: &str) -> Result<Score, &'static str> {
        if guess.len() != self.secret.len() {
            return Err("Invalid guess length.");
        } else {
            let score = score_guess(&self.secret, &guess);
            self.add_guess(guess.to_string(), score.clone());
            Ok(score)
        }
    }
    pub fn handle_guess(&mut self, guess: &str) -> Result<Score, &'static str> {
        let result = self.validate_guess(&guess);
        match result {
            Err(some) => Err(some),
            Ok(score) => {
                self.add_guess(guess.to_string(), score.clone());
                Ok(score)
            }
        }
    }

    pub fn change_secret(&mut self, new_secret: &str) -> Result<(), ErrResponse> {
        let result = self.validate_secret(new_secret);
        match result {
            Ok(some) => {
                self.previous_secrets
                    .push(std::mem::replace(&mut self.secret, new_secret.to_string()));
                Ok(some)
            }
            err => err,
        }
    }

    fn validate_secret(&mut self, new_secret: &str) -> Result<(), ErrResponse> {
        if new_secret.len() == 0 {
            return Err(ErrResponse::LengthMismatch(self.secret.len()));
        } else if self.secret.len() == 0 {
            self.secret = new_secret.to_string();
            return Ok(());
        } else if new_secret.len() != self.secret.len() {
            return Err(ErrResponse::LengthMismatch(self.secret.len()));
        } else if !new_secret.chars().all(|c| c.is_digit(10)) {
            return Err(ErrResponse::CharsetMismatch());
        } else if self.previous_secrets.contains(&new_secret.to_string()) {
            return Err(ErrResponse::NoSecretChange());
        } else {
            // Validate new secret against previous guesses
            let mismatches: Vec<(String, Score)> = self
                .previous_guesses
                .iter()
                .filter_map(|(prev_guess, prev_score)| {
                    let new_score = score_guess(&new_secret, prev_guess);
                    if new_score.bulls != prev_score.bulls || new_score.cows != prev_score.cows {
                        Some((prev_guess.clone(), prev_score.clone()))
                    } else {
                        None
                    }
                })
                .collect();

            if !mismatches.is_empty() {
                return Err(ErrResponse::GuessMismatch(mismatches));
            } else {
                return Ok(());
            }
        }
    }
}

pub fn score_guess(secret: &str, guess: &str) -> Score {
    let bulls = secret
        .chars()
        .zip(guess.chars())
        .filter(|(s, g)| s == g)
        .count();

    let cows = guess.chars().filter(|g| secret.contains(*g)).count() - bulls;

    Score::new(bulls, cows)
}

#[derive(PartialEq, Debug)]
pub enum ErrResponse {
    LengthMismatch(usize),
    CharsetMismatch(),
    NoSecretChange(),
    GuessMismatch(Vec<(String, Score)>),
}

impl ErrResponse {
    pub fn message(&self) -> String {
        match self {
            Self::LengthMismatch(len) => {
                format!("Input length did not match the secret. Expected {}", len)
            }
            Self::CharsetMismatch() => "Expected Digits only".to_string(),
            Self::GuessMismatch(_) => "Previous Guesses did not match the new secret".to_string(),
            Self::NoSecretChange() => "The secret was chosen before".to_string(),
        }
    }
}

// game.rs

// Existing code...

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score_guess_correct() {
        assert_eq!(score_guess("1234", "1234"), Score::new(4, 0));
    }

    #[test]
    fn test_score_guess_with_cows() {
        assert_eq!(score_guess("1234", "1243"), Score::new(2, 2));
    }

    #[test]
    fn test_init_secret_valid() {
        let mut game = Game::new();
        let response = game.change_secret("1234");
        assert!(response.is_ok());
        assert_eq!(game.secret, "1234".to_string());
    }

    #[test]
    fn test_init_secret_empty() {
        let mut game = Game::new();
        let response = game.change_secret("");
        assert!(response.is_err());
        assert_eq!(game.secret.len(), 0);
    }

    #[test]
    fn test_update_secret_valid() {
        let mut game = Game::new();
        let _ = game.change_secret("1234");
        let response = game.change_secret("4321");
        assert!(response.is_ok());
        assert_eq!(game.secret, "4321".to_string());
    }

    #[test]
    fn test_update_secret_invalid_length() {
        let mut game = Game::new();
        let _ = game.change_secret("1234");
        let response = game.change_secret("123");
        assert_eq!(response, Err(ErrResponse::LengthMismatch(4)));
    }

    #[test]
    fn test_update_secret_no_change() {
        let mut game = Game::new();
        let _ = game.change_secret("1234");
        let response = game.change_secret("1234");
        assert_eq!(response, Err(ErrResponse::NoSecretChange()));
    }

    #[test]
    fn test_update_secret_invalid_digits() {
        let mut game = Game::new();
        let _ = game.change_secret("1234");
        let response = game.change_secret("123a");
        assert_eq!(response, Err(ErrResponse::CharsetMismatch()));
    }

    #[test]
    fn test_update_secret_invalid_mismatch() {
        let mut game = Game::new();
        let _ = game.change_secret("1234");
        game.add_guess("5673".to_string(), Score::new(0, 1));
        let response = game.change_secret("1278");
        let expected = ErrResponse::GuessMismatch(vec![("5673".to_string(), Score::new(0, 1))]);
        //let expected = format_mismatch_feedback(&vec![("5678".to_string(), (0, 0))], "5678");
        assert_eq!(response, Err(expected))
    }
}
