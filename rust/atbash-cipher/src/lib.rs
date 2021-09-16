use std::char;

const A: u32 = 97;
const Z: u32 = 122;

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
  plain
    .chars()
    .filter(|x| x.is_ascii_alphanumeric())
    .enumerate()
    .map(|(idx, ci)| -> String {
      let co = match ci {
        '0'..='9' => ci,
        _ => char::from_u32(Z + A - ci.to_ascii_lowercase() as u32).unwrap(),
      };
      // check for 0 needed to avoid off-by-one error
      if (idx != 0) && (idx % 5) == 0 {
        format!(" {}", co)
      } else {
        co.to_string()
      }
    })
    .collect()
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
  cipher
    .chars()
    .filter(|c| c.is_ascii_alphanumeric()) // remove spaces
    .map(|c| match c {
      '0'..='9' => c,
      _ => char::from_u32(Z + A - c as u32).unwrap(),
    })
    .collect()
}
