struct PrimeChecker {
  primes: Vec<u32>,
}

impl PrimeChecker {
  fn is_prime(&mut self, n: u32) -> bool {
    // Integers larger than the square root do not need to be checked
    // because, whenever n = a × b, one of the two factors a and b ≤ √n or
    // one of them  needs to be a real number; however for primality we need
    // both to be integers. So given n, check if any z ∈ [2, ⌊√n⌋] evenly
    // divides n.
    // OPTIMIZATIONS
    // 1. Avoid using the usually expensive `sqrt`
    // 2. Only check prime numbers in the range
    // REFERENCES
    // https://en.wikipedia.org/wiki/Prime_number#Trial_division
    // https://en.wikipedia.org/wiki/Primality_test
    //
    // NOTE: a generalized PrimeChecker getting non-sequential queries
    // should first check if n is in primes before remainder checking. Also
    // if n < primes.last() then skip remainder checking completely.
    for factor in self.primes.iter() {
      // break if we’ve exceeded √n
      if (factor * factor) > n {
        break;
      } else if (n % factor) == 0 {
        return false;
      }
    }
    self.primes.push(n);
    true
  }
}

pub fn nth(n: u32) -> u32 {
  // NOTE: an optimization would be include first few primes in this LUT
  let mut checker = PrimeChecker { primes: vec![] };
  (2..)
    .filter(|z| checker.is_prime(*z))
    .nth(n as usize)
    .unwrap()
}
