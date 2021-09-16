fn is_prime(n: u32) -> bool {
  for i in 2..n {
    if (n % i) == 0 {
      return false;
    }
  }
  true
}

pub fn nth(n: u32) -> u32 {
  let mut i = 2u32;
  let mut found = 1;
  while (n + 1) > found {
    i = i + 1;
    found = found + if is_prime(i) { 1 } else { 0 };
  }
  i
}
