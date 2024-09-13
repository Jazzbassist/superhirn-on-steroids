// game.rs
use colored::Colorize; // Import the Colorize trait

pub struct Game<T: InputSource> {
    pub secret: String,
    pub previous_guesses: Vec<(String, (usize, usize))>,
    input_source: T,
}

impl<T: InputSource> Game<T> {
    pub fn new(secret: String, input_source: T) -> Self {
        Self {
            secret,
            previous_guesses: Vec::new(),
            input_source,
        }
    }

    pub fn add_guess(&mut self, guess: String, score: (usize, usize)) {
        self.previous_guesses.push((guess, score));
    }

    pub fn update_secret(&mut self, new_secret: String) -> SecretChangeResponse {
        if new_secret.len() != self.secret.len() {
            return SecretChangeResponse::Invalid("The new secret must be the same length as the original secret.".to_string());
        }

        if !new_secret.chars().all(|c| c.is_digit(10)) {
            return SecretChangeResponse::Invalid("The new secret must be composed of digits only.".to_string());
        }

        let mismatches = self.validate_secret(&new_secret);

        if mismatches.is_empty() {
            self.secret = new_secret;
            SecretChangeResponse::Valid
        } else {
            let feedback = format_mismatch_feedback(mismatches, &new_secret);
            SecretChangeResponse::Invalid(feedback)
        }
    }

    fn validate_secret(&self, new_secret: &str) -> Vec<(String, usize, usize, usize, usize)> {
        let mut mismatches = Vec::new();
        for (prev_guess, (prev_bulls, prev_cows)) in &self.previous_guesses {
            let (new_bulls, new_cows) = score_guess(&new_secret, prev_guess);
            if new_bulls != *prev_bulls || new_cows != *prev_cows {
                mismatches.push((prev_guess.clone(), *prev_bulls, *prev_cows, new_bulls, new_cows));
            }
        }
        mismatches
    }
}

pub trait InputSource {
    fn read_line(&mut self) -> String;
}

pub struct StdInput;

impl InputSource for StdInput {
    fn read_line(&mut self) -> String {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        input.trim().to_string()
    }
}

pub enum SecretChangeResponse {
    Valid,
    Invalid(String),
}

impl SecretChangeResponse {
    pub fn message(&self) -> &str {
        match self {
            SecretChangeResponse::Valid => "Secret updated successfully.",
            SecretChangeResponse::Invalid(msg) => &msg,
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

fn format_mismatch_feedback(
    mismatches: Vec<(String, usize, usize, usize, usize)>,
    new_secret: &str,
) -> String {
    let mut feedback = String::from("The new secret does not match the score for these guesses:\n");
    for (guess, expected_bulls, expected_cows, actual_bulls, actual_cows) in mismatches {
        feedback.push_str("Guess: ");
        let mut guess_display = String::new();
        for (s_char, g_char) in new_secret.chars().zip(guess.chars()) {
            if s_char == g_char {
                guess_display.push_str(&g_char.to_string().green().to_string());
            } else if new_secret.contains(g_char) {
                guess_display.push_str(&g_char.to_string().yellow().to_string());
            } else {
                guess_display.push(g_char);
            }
        }
        feedback.push_str(&guess_display);
        feedback.push_str(&format!(
            ", Expected {} bulls and {} cows, Found {} bulls and {} cows\n",
            expected_bulls, expected_cows, actual_bulls, actual_cows
        ));
    }
    feedback
}
