#[derive(PartialEq, Clone, Debug)]
pub struct Score {
    pub bulls: usize,
    pub cows: usize,
}

impl Score {
    pub fn new(bulls: usize, cows: usize) -> Self {
        Score { bulls, cows }
    }
}