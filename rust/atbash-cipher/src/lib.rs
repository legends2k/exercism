// NOTE: atbash cipher’s encode and decode are inverses of each other, so this
// kernel works for both
fn atbash(data: char) -> char {
  match data {
    '0'..='9' => data,
    _ => char::from(b'z' + b'a' - data as u8),
  }
}

struct AtbashIterator<'a> {
  data: &'a str,
  idx: usize,
}

impl Iterator for AtbashIterator<'_> {
  type Item = char;

  fn next(&mut self) -> Option<Self::Item> {
    while self.idx < self.data.len() {
      let ch = self.data.as_bytes()[self.idx] as char;
      self.idx += 1;
      if ch.is_ascii_alphanumeric() {
        return Some(atbash(ch.to_ascii_lowercase()));
      }
    }
    None
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    (0, Some(self.data.len() - self.idx))
  }
}

fn atbash_str(input: &str) -> AtbashIterator {
  AtbashIterator {
    data: input,
    idx: 0,
  }
}

// iterator mapping a char to a char with/out space
struct CharIter {
  data: char,
  ins_space: bool,
  has_ended: bool,
}

impl CharIter {
  fn new(ch: char, need_space: bool) -> Self {
    CharIter {
      data: ch,
      ins_space: need_space,
      has_ended: false,
    }
  }
}

impl Iterator for CharIter {
  type Item = char;

  fn next(&mut self) -> Option<Self::Item> {
    match self.has_ended {
      true => None,
      false => match self.ins_space {
        false => {
          self.has_ended = true;
          Some(self.data)
        }
        true => {
          self.ins_space = false;
          Some(' ')
        }
      },
    }
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    match (self.has_ended, self.ins_space) {
      (true, _) => (0, Some(0)),
      (false, true) => (2, Some(2)),
      (false, false) => (1, Some(1)),
    }
  }
}

// When taking an iterator input, prefer IntoIterator over Iterator for
// cleaner client code
// https://stackoverflow.com/a/34969944/183120
// Options to return an iterator.  `impl Trait`: static dispatch.
// https://stackoverflow.com/a/27535594/183120
fn add_spaces<It>(input: It) -> impl Iterator<Item = char>
where
  It: IntoIterator<Item = char>,
{
  input
    .into_iter() // Iterators return self
    .enumerate()
    .flat_map(|(i, x)| CharIter::new(x, i % 5 == 0))
    .skip(1) // skip leading white space
}

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
  add_spaces(atbash_str(plain)).collect()
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
  atbash_str(cipher).collect()
}
