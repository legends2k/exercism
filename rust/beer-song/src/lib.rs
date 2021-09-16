pub fn verse(n: u32) -> String {
  // line 1
  let count0 = if n >= 1 {
    n.to_string()
  } else {
    String::from("No more")
  };
  let count1 = if n >= 1 {
    n.to_string()
  } else {
    String::from("no more")
  };
  let plural0 = if n != 1 { "s" } else { "" };
  let mut v = format!(
    "{0} bottle{2} of beer on the wall, {1} bottle{2} of beer.\n",
    count0, count1, plural0
  );
  let count3 = if n != 1 { "one" } else { "it" };
  let count4 = if n > 1 {
    (n - 1).to_string()
  } else {
    String::from("no more")
  };
  let plural1 = if n != 2 { "s" } else { "" };
  let l2 = match n {
    1..=99 => format!(
      "Take {} down and pass it around, {} bottle{} of beer on the wall.\n",
      count3, count4, plural1
    ),
    _ => format!(
      "{}",
      "Go to the store and buy some more, 99 bottles of beer on the wall.\n"
    ),
  };
  v.push_str(&l2);
  v
}

pub fn sing(start: u32, end: u32) -> String {
  let mut v: Vec<String> = Vec::with_capacity((start - end) as usize);
  for x in (end..=start).rev() {
    v.push(verse(x));
  }
  return v.join("\n");
}
