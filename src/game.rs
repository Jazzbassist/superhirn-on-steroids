// game.rs
use crate::ui::{format_mismatch_feedback, Player};

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
