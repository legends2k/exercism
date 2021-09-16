extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
  let mut result = String::with_capacity(input.len());
  for gc in UnicodeSegmentation::graphemes(input, false /*extended*/).rev() {
    result.push_str(gc)
  }
  result
}
