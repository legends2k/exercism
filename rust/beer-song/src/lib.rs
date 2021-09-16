// enum Phrase {
//     One,
//     Two,
//     Three,
//     Four,
// }

fn plurality_suffix(n: i32, /*, p: Phrase*/) -> (&'static str, &'static str) {
  match n {
    1 => ("", "s"),
    2 => ("s", ""),
    _ => ("s", "s"),
  }
  // match p {
  //     Phrase::Three => if n != 2 { "s" } else { "" },
  //     _ => if n != 1 { "s" } else { "" },
  // }
}

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
  //     // line 1
  //     let count0 = if n >= 1 { n.to_string() } else { String::from("No more") };
  //     let count1 = if n >= 1 { n.to_string() } else { String::from("no more") };
  //     let plural0 = if n != 1 { "s" } else { "" };
  //     let mut v = format!("{0} bottle{2} of beer on the wall, {1} bottle{2} of beer.\n", count0, count1, plural0);
  //     let count3 = if n != 1 { "one" } else { "it" };
  //     let count4 = if n > 1 { (n - 1).to_string() } else { String::from("no more") };
  //     let plural1 = if n != 2 { "s" } else { "" };
  //     let l2 = match n {
  //         1..=99 => format!("Take {} down and pass it around, {} bottle{} of beer on the wall.\n", count3, count4, plural1),
  //         _ => format!("{}", "Go to the store and buy some more, 99 bottles of beer on the wall.\n"),
  //     };
  //     v.push_str(&l2);
  //     v
  let (c1, c2, c3) = count(n as i32);
  let (s1, s2) = plurality_suffix(n as i32);
  let mut v = format!(
    "{} bottle{} of beer on the wall, {} bottle of beer",
    c1, s1, c2
  );
  // v.push_str(format!("{}, {} bottle{} of beer on the wall.", phrase3(n), c3, s2));
  v
}

pub fn sing(start: u32, end: u32) -> String {
  let mut v = Vec::with_capacity((start - end) as usize);
  for x in (end..=start).rev() {
    v.push(verse(x));
  }
  return v.join("\n");
}
