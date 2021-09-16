use unicode_segmentation::UnicodeSegmentation;

// see this on why String::chars shouldn’t be used
// https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=779bfd82f20caf0e588901f3a5051a6d

pub fn reverse(input: &str) -> String {
  input
    .graphemes(true /*extended*/)
    .rev()
    .collect::<String>()
}
