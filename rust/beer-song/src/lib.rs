fn count(n: i32) -> (String, String, String) {
  let n_str = n.to_string();
  let l_str = (n - 1).to_string();
  match n {
    0 => (
      String::from("No more"),
      String::from("no more"),
      99.to_string(),
    ),
    1 => (n_str.clone(), n_str, String::from("no more")),
    _ => (n_str.clone(), n_str, l_str),
  }
}

fn plurality_suffix(n: i32) -> (&'static str, &'static str) {
  match n {
    1 => ("", "s"),
    2 => ("s", ""),
    _ => ("s", "s"),
  }
}

fn phrase3(n: u32) -> String {
  match n {
    0 => String::from("Go to the store and buy some more"),
    _ => format!(
      "Take {} down and pass it around",
      if n != 1 { "one" } else { "it" }
    ),
  }
}

pub fn verse(n: u32) -> String {
  let (count1, count2, count3) = count(n as i32);
  let (suffix1, suffix2) = plurality_suffix(n as i32);
  format!(
    "{} bottle{} of beer on the wall, {} bottle{} of beer.\n\
     {}, {} bottle{} of beer on the wall.\n",
    count1,
    suffix1,
    count2,
    suffix1,
    phrase3(n),
    count3,
    suffix2
  )
}

pub fn sing(start: u32, end: u32) -> String {
  (end..=start)
    .rev()
    .map(|i| verse(i))
    .collect::<Vec<String>>()
    .join("\n")
}
