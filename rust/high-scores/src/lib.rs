#[derive(Debug)]
pub struct HighScores<'a> {
  scores: &'a [u32],
}

impl<'a> HighScores<'a> {
  pub fn new(scores: &'a [u32]) -> Self {
    HighScores { scores: scores }
  }

  pub fn scores(&self) -> &[u32] {
    self.scores
  }

  pub fn latest(&self) -> Option<u32> {
    // Convert Option<&T> to Option<T>:
    // Rust 1.0.0 introduced .cloned() exactly for this
    // see https://stackoverflow.com/a/63903840/183120
    self.scores.last().map(|x| *x)
  }

  pub fn personal_best(&self) -> Option<u32> {
    // max().copied() also works here sicne u32 is copyable
    self.scores.iter().max().cloned()
  }

  pub fn personal_top_three(&self) -> Vec<u32> {
    let mut scores_copy = self.scores.to_vec();
    scores_copy.sort_unstable();
    // copied()/cloned() converts &T to T.  Alternatively,
    // into_iter().rev().take(3).collect() works too.
    scores_copy.iter().rev().take(3).copied().collect()
  }
}
