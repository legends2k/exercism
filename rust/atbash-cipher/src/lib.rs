// NOTE
// 1. encode and decode are inverses of each other
// 2. add spacing for encode separately as a second step; avoid reseating by
//    constructing the output in a new string

fn atbash(data: u8) -> u8 {
  match data {
    b'0'..=b'9' => data,
    _ => b'z' + b'a' - data,
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
        Some(ch) => Some(char::from(atbash(ch.to_ascii_lowercase()))),
        None => None,
      }
    }
  });
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
    .map(|c| char::from(atbash(c as u8)))
    .collect()
}
