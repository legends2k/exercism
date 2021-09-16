pub fn check(candidate: &str) -> bool {
  let mut found: Vec<bool> = std::iter::repeat(false).take(26).collect();
  candidate
    .chars()
    .filter(|c| c.is_ascii_alphabetic())
    .all(|c| {
      let idx: usize = c.to_ascii_lowercase() as usize - 'a' as usize;
      found[idx] = !found[idx];
      found[idx]
    })
}
