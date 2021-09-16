/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
  sentence
    .bytes()
    .filter(|b| b.is_ascii_alphabetic())
    .map(|b| b.to_ascii_lowercase())
    .fold(0u32, |acc, ch| acc | (1 << (ch - b'a')))
    == 0b11_1111_1111_1111_1111_1111_1111
}
