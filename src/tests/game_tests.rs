// tests/game_tests.rs
use crate::game::*;

#[test]
fn test_score_guess() {
    let secret = "1234";
    let guess = "1243";
    let (bulls, cows) = score_guess(secret, guess);
    assert_eq!(bulls, 2);
    assert_eq!(cows, 2);
}

#[test]
fn test_update_secret_valid() {
    let mut game = Game::new("1234".to_string());
    let result = game.update_secret("1245".to_string());
    assert_eq!(result, SecretChangeResponse::Valid);
    assert_eq!(game.secret, "1245".to_string());
}

#[test]
fn test_update_secret_invalid_length() {
    let mut game = Game::new("1234".to_string());
    let result = game.update_secret("123".to_string());
    assert_eq!(result, SecretChangeResponse::Invalid("New secret length mismatch.".to_string()));
}

#[test]
fn test_update_secret_invalid_characters() {
    let mut game = Game::new("1234".to_string());
    let result = game.update_secret("12a4".to_string());
    assert_eq!(result, SecretChangeResponse::Invalid("New secret must be composed of digits only.".to_string()));
}

#[test]
fn test_format_guess_for_display() {
    let formatted = format_guess_for_display("1243", "1234", true);
    assert_eq!(formatted, "1".green().to_string() + "2".green().to_string() + "4".yellow().to_string() + "3".yellow().to_string());
}
