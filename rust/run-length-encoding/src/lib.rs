fn push_repeat_char(out: &mut String, c: char, repeats: u32) {
  out.pop();
  let temp = format!("{}{}", repeats + 1, c);
  out.push_str(&temp);
}

pub fn encode(source: &str) -> String {
  let mut enc = String::with_capacity(source.len());
  let mut prev: char = '\0';
  let mut repeats = 0u32;
  // the push, pop, push_str drama can be avoided; see
  // https://exercism.io/tracks/rust/exercises/run-length-encoding/solutions/399849c55d1f485b8cccfc62db49c4cb
  for c in source.chars() {
    if c != prev {
      if repeats > 0 {
        push_repeat_char(&mut enc, prev, repeats);
        repeats = 0;
      }
      enc.push(c);
      prev = c;
    } else {
      repeats += 1;
    }
  }
  if repeats > 0 {
    if let Some(c) = enc.chars().last() {
      push_repeat_char(&mut enc, c, repeats);
    }
  }
  enc
}

pub fn decode(source: &str) -> String {
  let mut dec = String::new();
  let mut count = String::new();
  // do it functionally with filter_map; see
  // https://exercism.io/tracks/rust/exercises/run-length-encoding/solutions/7fb896dce83c4703adff8b05586cf7f2
  for c in source.chars() {
    if c.is_numeric() {
      count.push(c);
    } else {
      let n = count.parse().unwrap_or(1);
      count.clear();
      let rep_c: String = std::iter::repeat(c).take(n).collect();
      dec.push_str(&rep_c);
    }
  }
  dec
}
