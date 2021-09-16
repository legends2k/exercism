/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
  let n = isbn.len();
  // ISBN-10 can be 10 chars (no dashes) or 13 chars (3 dashes)
  if n < 10 || n > 13 {
    return false;
  }

  // skip the last character since it might be X
  let max_idx: usize = n
    - match isbn.ends_with("X") {
      true => 2,
      false => 1,
    };

  let total = &isbn[0..=max_idx]
    .chars()
    // not doing c.is_numeric() to handle invalid chars ∉ {0..9, -}
    .filter(|c| *c != '-')
    .enumerate()
    .fold(0.0, |acc, (i, c)| {
      // deter invalid char and strings longer than 10 chars
      if i > 9 || c.to_digit(10).is_none() {
        return std::f32::INFINITY;
      }
      acc + ((10.0 - i as f32) * c.to_digit(10).unwrap() as f32)
    })
    + match isbn.ends_with("X") {
      true => 10.0,
      false => 0.0,
    };
  (total % 11.0) == 0.0
}
