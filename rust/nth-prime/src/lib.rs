fn is_prime(n: u32, p: &Vec<u32>) -> bool {
  // Integers larger than the square root do not need to be checked because,
  // whenever n = a × b, one of the two factors a and b ≤ √n or one of them
  // needs to be a real number; however for primality we need both to be
  // integers. So given n, check if any z ∈ [2, ⌊√n⌋] evenly divides n.
  // OPTIMIZATIONS
  // 1. Avoid using the usually expensive `sqrt`
  // 2. Only check prime numbers in the range
  // REFERENCES
  // https://en.wikipedia.org/wiki/Prime_number#Trial_division
  // https://en.wikipedia.org/wiki/Primality_test
  for factor in p {
    // break if we’ve exceeded √n
    if (factor * factor) > n {
      break;
    } else if (n % factor) == 0 {
      return false;
    }
  }
  true
}

pub fn nth(n: u32) -> u32 {
  let mut p = vec![2, 3];
  let mut z = *p.last().unwrap();
  while n > (p.len() - 1) as u32 {
    z = z + 1;
    if is_prime(z, &p) {
      p.push(z);
    }
  }
  p[n as usize]
}
