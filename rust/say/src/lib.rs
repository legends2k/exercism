use std::str;

fn units(n: u8) -> String {
  assert!((0..10).contains(&n));
  match n {
    0 => "zero",
    1 => "one",
    2 => "two",
    3 => "three",
    4 => "four",
    5 => "five",
    6 => "six",
    7 => "seven",
    8 => "eight",
    9 => "nine",
    _ => unreachable!(),
  }
  .to_string()
}

fn tens(mut n: u8) -> String {
  assert!((10..=99).contains(&n));
  let unit = match (10..=20).contains(&n) {
    true => 0u8,
    false => {
      let rem = n % 10;
      n -= rem;
      rem
    }
  };
  let tens = match n {
    10 => "ten",
    11 => "eleven",
    12 => "twelve",
    13 => "thirteen",
    14 => "fourteen",
    15 => "fifteen",
    16 => "sixteen",
    17 => "seventeen",
    18 => "eighteen",
    19 => "nineteen",
    20 => "twenty",
    30 => "thirty",
    40 => "forty",
    50 => "fifty",
    60 => "sixty",
    70 => "seventy",
    80 => "eighty",
    90 => "ninety",
    _ => unreachable!(),
  };
  match unit {
    0 => tens.into(),
    _ => format!("{}-{}", tens, units(unit)),
  }
}

fn hundreds(n: u16) -> String {
  let t = n % 100;
  let h = n / 100;
  let hundreds_words = units(h as u8);
  match t {
    0 => format!("{} hundred", hundreds_words),
    1..=9 => format!("{} hundred {}", hundreds_words, units(t as u8)),
    _ => format!("{} hundred {}", hundreds_words, tens(t as u8)),
  }
}

fn encode_util(num: u16) -> String {
  match num {
    0..=9 => units(num as u8),
    10..=99 => tens(num as u8),
    100..=999 => hundreds(num),
    _ => unreachable!(),
  }
}

pub fn encode(num: u64) -> String {
  match num {
    0..=999 => encode_util(num as u16),
    _ => {
      // u64.MAX = 2⁶⁴ - 1 = 18,446,744,073,709,551,615
      let mappings = [
        "thousand",
        "million",
        "billion",
        "trillion",
        "quadrillion",
        "quintillion",
      ];
      num
        .to_string()
        .as_bytes()
        .rchunks(3)
        .enumerate()
        .rev()
        .map(|(id, sub)| {
          let num = str::from_utf8(sub)
            .map(|s| s.parse::<u16>())
            .unwrap()
            .unwrap();
          match num {
            0 => None,
            _ => {
              let words = encode_util(num as u16);
              match id {
                0 => Some(words),
                1..=6 => Some(format!("{} {}", words, mappings[id - 1])),
                _ => unreachable!(),
              }
            }
          }
        })
        .fold(String::new(), |acc, s| match (acc.is_empty(), s) {
          (_, None) => acc,
          (true, Some(sub)) => sub,
          (false, Some(sub)) => acc + " " + &sub,
        })
    }
  }
}
