use std::collections::{HashMap, HashSet};

fn char_counts(word: &str) -> HashMap<char, u8> {
  let mut counts = HashMap::<char, u8>::with_capacity(word.len());
  word.chars().for_each(|c| {
    *counts.entry(c).or_default() += 1;
  });
  counts
}

pub fn anagrams_for<'a>(
  word: &str,
  possible_anagrams: &[&'a str],
) -> HashSet<&'a str> {
  let word_lowercase = word.to_lowercase();
  let counts = char_counts(&word_lowercase);
  possible_anagrams
    .iter()
    .filter(|&&s| {
      let s_lowercase = s.to_lowercase();
      if word_lowercase == s_lowercase {
        return false;
      }
      counts == char_counts(&s_lowercase)
    })
    .copied()
    .collect()
}
