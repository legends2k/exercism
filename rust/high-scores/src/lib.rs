#[derive(Debug)]
pub struct HighScores {
  scores: Vec<u32>,
}

impl HighScores {
  pub fn new(scores: &[u32]) -> Self {
    HighScores {
      scores: scores.to_vec(),
    }
  }

  pub fn scores(&self) -> &[u32] {
    &self.scores
  }

  pub fn latest(&self) -> Option<u32> {
    self.scores.last().map(|x| *x)
  }

  pub fn personal_best(&self) -> Option<u32> {
    self.scores.iter().max().map(|x| *x)
  }

  pub fn personal_top_three(&self) -> Vec<u32> {
    let mut scores_copy = self.scores.clone();
    scores_copy.sort_unstable();
    scores_copy.iter().rev().take(3).map(|&x| x).collect()
  }
}
