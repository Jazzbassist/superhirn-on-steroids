// game.rs
use crate::ui::format_mismatch_feedback;

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
    previous_guesses: Vec<(String, Score)>, // Use Score instead of tuple
}

impl Game {
    pub fn new(secret: String) -> Self {
        Game {
            secret,
            previous_guesses: Vec::new(),
        }
    }

    pub fn get_previous_guesses(&self) -> &[(String, Score)] {
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

    pub fn handle_guess(&mut self, guess: String) -> Result<Score, &'static str> {
        if guess.len() != self.secret.len() {
            return Err("Invalid guess length.");
        }

        let score = score_guess(&self.secret, &guess);
        self.add_guess(guess.clone(), score.clone());
        Ok(score)
    }

    fn update_secret_internal(&mut self, new_secret: String) -> SecretChangeResponse {
        if new_secret.len() != self.secret.len() {
            return SecretChangeResponse::Invalid("New secret length mismatch.".to_string());
        }

        if !new_secret.chars().all(|c| c.is_digit(10)) {
            return SecretChangeResponse::Invalid(
                "New secret must be composed of digits only.".to_string(),
            );
        }

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

        if mismatches.is_empty() {
            self.secret = new_secret;
            SecretChangeResponse::Valid
        } else {
            SecretChangeResponse::Impossible(mismatches, self.secret.clone())
        }
    }

    pub fn change_secret(&mut self, new_secret: String) -> SecretChangeResponse {
        // This can be exposed to the UI or main loop
        self.update_secret_internal(new_secret)
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
pub enum SecretChangeResponse {
    Valid,
    Invalid(String),
    Impossible(Vec<(String, Score)>, String),
}

impl SecretChangeResponse {
    pub fn message(&self) -> String {
        match self {
            SecretChangeResponse::Valid => "Secret updated successfully.".to_string(),
            SecretChangeResponse::Invalid(msg) => msg.to_string(),
            SecretChangeResponse::Impossible(mismatches, secret) => {
                format_mismatch_feedback(mismatches, secret)
            }
        }
    }

    pub fn is_valid(&self) -> bool {
        match self {
            SecretChangeResponse::Valid => true,
            _ => false,
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
    fn test_update_secret_valid() {
        let mut game = Game::new("1234".to_string());
        let response = game.change_secret("4321".to_string());
        assert_eq!(response, SecretChangeResponse::Valid);
        assert_eq!(game.secret, "4321".to_string());
    }

    #[test]
    fn test_update_secret_invalid_length() {
        let mut game = Game::new("1234".to_string());
        let response = game.change_secret("123".to_string());
        assert_eq!(
            response,
            SecretChangeResponse::Invalid("New secret length mismatch.".to_string())
        );
    }

    #[test]
    fn test_update_secret_invalid_digits() {
        let mut game = Game::new("1234".to_string());
        let response = game.change_secret("123a".to_string());
        assert_eq!(
            response,
            SecretChangeResponse::Invalid(
                "New secret must be composed of digits only.".to_string()
            )
        );
    }

    #[test]
    fn test_update_secret_invalid_mismatch() {
        let mut game = Game::new("1234".to_string());
        game.add_guess("5678".to_string(), Score::new(0, 0));
        let response = game.change_secret("1278".to_string());
        let expected = SecretChangeResponse::Impossible(
            vec![("5678".to_string(), Score::new(0, 0))],
            "1234".to_string(),
        );
        //let expected = format_mismatch_feedback(&vec![("5678".to_string(), (0, 0))], "5678");
        assert_eq!(response, expected);
    }
}
