pub fn annotate(minefield: &[&str]) -> Vec<String> {
  let (r, c) = match minefield.is_empty() {
    true => (0, 0),
    false => (minefield.len(), minefield[0].len()),
  };
  minefield
    .iter()
    .enumerate()
    .map(|(i, &s)| -> String {
      s.bytes()
        .enumerate()
        .map(|(j, b)| match b {
          b'*' => '*',
          _ => {
            let left = j.saturating_sub(1);
            let right = c.min(j + 2);
            let top = i.saturating_sub(1);
            let bottom = r.min(i + 2);
            mines_to_char(
              minefield[top..bottom]
                .iter()
                .map(|&row| {
                  row.as_bytes()[left..right]
                    .iter()
                    .filter_map(|x| if x != &b { Some(mine(*x)) } else { None })
                    .sum::<u8>()
                })
                .sum(),
            )
          }
        })
        .collect()
    })
    .collect()
}

fn mine(b: u8) -> u8 {
  match b {
    b' ' => 0,
    b'*' => 1,
    _ => unreachable!(),
  }
}

fn mines_to_char(c: u8) -> char {
  match c {
    0 => ' ',
    _ => (b'0' + c) as char,
  }
}
