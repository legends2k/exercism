/// Determines whether the supplied string is a valid ISBN number
// Multiply each digit with weights decreasing from 10, sum, check mod 11
// Examples: 0198526636, 0-19-852663-6, 8-42-261586-X
pub fn is_valid_isbn(isbn: &str) -> bool {
  let n = isbn.len();
  // ISBN-10 can be 10 chars (no dashes) or 13 chars (3 dashes)
  if n < 10 || n > 13 {
    return false;
  }

  // skip the last character since it might be X
  let (max_idx, base) = match isbn.ends_with("X") {
    true => (n - 2, 10.0),
    false => (n - 1, 0.0),
  };

  let checksum = &isbn[0..=max_idx]
    .chars()
    // not doing c.is_numeric() to handle invalid chars i.e. ∉ {0..=9, -}
    .filter(|&c| c != '-')
    .enumerate()
    .fold(base, |acc, (i, c)| {
      // deter invalid char and strings longer than 10 digits
      if i > 9 || c.to_digit(10).is_none() {
        return std::f32::INFINITY;
      }
      acc + ((10.0 - i as f32) * c.to_digit(10).unwrap() as f32)
    });
  (checksum % 11.0) == 0.0
}
