pub fn reply(message: &str) -> &str {
  let m = message.trim();
  if m.is_empty() {
    "Fine. Be that way!"
  } else {
    let question = m.ends_with("?");
    let yell = m
      .chars()
      .filter(|c| c.is_ascii_alphabetic())
      .enumerate()
      .fold(false, |res, (idx, c)| {
        ((idx == 0) || res) && c.is_ascii_uppercase()
      });
    match (question, yell) {
      (true, true) => "Calm down, I know what I'm doing!",
      (false, true) => "Whoa, chill out!",
      (true, false) => "Sure.",
      _ => "Whatever.",
    }
  }
}
