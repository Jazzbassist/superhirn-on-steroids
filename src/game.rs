// game.rs
use crate::ui::format_mismatch_feedback;

pub struct Game {
    pub secret: String,
    pub previous_guesses: Vec<(String, (usize, usize))>,
}

impl Game {
    pub fn new(secret: String) -> Self {
        Game {
            secret,
            previous_guesses: Vec::new(),
        }
    }

    pub fn add_guess(&mut self, guess: String, score: (usize, usize)) {
        self.previous_guesses.push((guess, score));
    }

    pub fn update_secret(&mut self, new_secret: String) -> SecretChangeResponse {
        if new_secret.len() != self.secret.len() {
            return SecretChangeResponse::Invalid("New secret length mismatch.".to_string());
        }

        if !new_secret.chars().all(|c| c.is_digit(10)) {
            return SecretChangeResponse::Invalid("New secret must be composed of digits only.".to_string());
        }

        // Validate new secret against previous guesses
        let mismatches: Vec<(String, (usize, usize))> = self.previous_guesses.iter()
            .filter_map(|(prev_guess, (prev_bulls, prev_cows))| {
                let (new_bulls, new_cows) = score_guess(&new_secret, prev_guess);
                if new_bulls != *prev_bulls || new_cows != *prev_cows {
                    Some((prev_guess.clone(), (*prev_bulls, *prev_cows)))
                } else {
                    None
                }
            })
            .collect();

        if mismatches.is_empty() {
            self.secret = new_secret;
            SecretChangeResponse::Valid
        } else {
            SecretChangeResponse::Invalid(format_mismatch_feedback(&mismatches, &new_secret))
        }
    }
}

pub fn score_guess(secret: &str, guess: &str) -> (usize, usize) {
    let bulls = secret
        .chars()
        .zip(guess.chars())
        .filter(|(s, g)| s == g)
        .count();
    let cows = guess.chars().filter(|g| secret.contains(*g)).count() - bulls;
    (bulls, cows)
}

#[derive(PartialEq, Debug)]
pub enum SecretChangeResponse {
    Valid,
    Invalid(String),
}

impl SecretChangeResponse {
    pub fn message(&self) -> &str {
        match self {
            SecretChangeResponse::Valid => "Secret updated successfully.",
            SecretChangeResponse::Invalid(msg) => msg,
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
        assert_eq!(score_guess("1234", "1234"), (4, 0));
    }

    #[test]
    fn test_score_guess_with_cows() {
        assert_eq!(score_guess("1234", "1243"), (2, 2));
    }

    #[test]
    fn test_update_secret_valid() {
        let mut game = Game::new("1234".to_string());
        let response = game.update_secret("4321".to_string());
        assert_eq!(response, SecretChangeResponse::Valid);
        assert_eq!(game.secret, "4321".to_string());
    }

    #[test]
    fn test_update_secret_invalid_length() {
        let mut game = Game::new("1234".to_string());
        let response = game.update_secret("123".to_string());
        assert_eq!(response, SecretChangeResponse::Invalid("New secret length mismatch.".to_string()));
    }

    #[test]
    fn test_update_secret_invalid_digits() {
        let mut game = Game::new("1234".to_string());
        let response = game.update_secret("123a".to_string());
        assert_eq!(response, SecretChangeResponse::Invalid("New secret must be composed of digits only.".to_string()));
    }

    #[test]
    fn test_update_secret_invalid_mismatch() {
        let mut game = Game::new("1234".to_string());
        game.add_guess("5678".to_string(), (0, 0));
        let response = game.update_secret("5678".to_string());
        let expected = "New secret does not match the score for these guesses:\nGuess: \u{1b}[32m5\u{1b}[0m\u{1b}[32m6\u{1b}[0m\u{1b}[32m7\u{1b}[0m\u{1b}[32m8\u{1b}[0m, Expected 0 bulls and 0 cows\n".to_string();
        //let expected = format_mismatch_feedback(&vec![("5678".to_string(), (0, 0))], "5678");
        assert_eq!(response, SecretChangeResponse::Invalid(expected));
    }
}
