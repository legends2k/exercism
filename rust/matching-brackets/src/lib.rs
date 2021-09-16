pub fn brackets_are_balanced(string: &str) -> bool {
  let open = b"[{(";
  let close = b"]})";

  if string
    .bytes()
    .filter(|ch| open.contains(ch) || close.contains(ch))
    .count()
    % 2
    != 0
  {
    return false;
  }

  string
    .bytes()
    .filter(|ch| open.contains(ch) || close.contains(ch))
    .fold(Vec::<u8>::with_capacity(16), |mut tally, bracket| {
      if let Some((idx, _)) =
        close.iter().enumerate().find(|&(_, &i)| i == bracket)
      {
        if Some(&open[idx as usize]) == tally.last() {
          tally.pop();
        } else {
          tally.push(b'x');
        }
      } else {
        tally.push(bracket);
      }
      tally
    })
    .is_empty()
}
