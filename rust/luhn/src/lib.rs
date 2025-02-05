/// Check a Luhn checksum.
pub fn is_valid(input: &str) -> bool {
  // Refer for a purely function impl. using try_fold and map_or
  // https://exercism.org/tracks/rust/exercises/luhn/solutions/JaneL
  let mut sum = 0;
  let mut len = 0;
  for (i, c) in input
    .chars()
    .filter(|c| !c.is_whitespace())
    .rev()
    .enumerate()
  {
    len = i + 1;
    sum += match c.to_digit(10) {
      None => return false,
      Some(d) => {
        let n = d + (d * (i % 2) as u32);
        if n > 9 {
          n - 9
        } else {
          n
        }
      }
    };
  }
  (len > 1) && (sum % 10) == 0
}
