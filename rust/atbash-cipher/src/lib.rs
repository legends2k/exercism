// NOTE: atbash cipher’s encode and decode are inverses of each other, so this
// kernel works for both
fn atbash(data: char) -> char {
  match data {
    '0'..='9' => data,
    _ => char::from(b'z' + b'a' - data as u8),
  }
}

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
  let mut chars = plain
    .as_bytes()
    .iter()
    .filter(|b| b.is_ascii_alphanumeric());
  let mut out_idx = 0;
  let iter = std::iter::from_fn(|| {
    out_idx += 1;
    if out_idx % 6 == 0 {
      Some(char::from(b' '))
    } else {
      match chars.next() {
        Some(ch) => Some(atbash(char::from(ch.to_ascii_lowercase()))),
        None => None,
      }
    }
  });
  // avoid iter.collect() here to prevent multiple allocations
  let mut cipher = String::with_capacity(plain.len() + plain.len() / 5);
  cipher.extend(iter);
  if cipher.ends_with(" ") {
    cipher.pop();
  }
  return cipher;
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
  cipher
    .chars()
    .filter(|c| c.is_ascii_alphanumeric()) // remove spaces
    .map(atbash)
    .collect()
}
