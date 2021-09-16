fn digits(mut n: u32) -> Vec<u8> {
  let mut v = Vec::new();
  while n > 0 {
    v.push((n % 10) as u8);
    n /= 10;
  }
  v
}

pub fn is_armstrong_number(num: u32) -> bool {
  let d = digits(num);
  let n = d.len() as u32;
  let sum: u32 = d.iter().map(|i| (*i as u32).pow(n)).sum();
  sum == num
}
