struct SeqPrimeChecker {
  primes: Vec<u32>,
}

// NOTE: This checker wouldn’t work for non-sequential queries
// e.g. below code won’t work since c.primes would be unpopulated.
//
//   let c = PrimeChecker::new();
//   c.is_prime(34);
//
// Populating isn’t an option, since that’d mean having a prime generator inside
// the checker!
impl SeqPrimeChecker {
  fn new() -> Self {
    SeqPrimeChecker { primes: vec![] }
  }

  fn is_prime(&mut self, n: u32) -> bool {
    // Primality test optimisations employed here follows from two important
    // observations:
    //
    //   1. Not all numbers from [2, n-1] need to be tested; integers > √n
    //      needn’t be checked as, when n = a × b, one of the two factors
    //      a or b ≤ √n; otherwise one needs to be a real number; however for
    //      primality we need both to be integers.
    //   2. Only prime divisors need to be checked
    //
    // So given n, check if any prime number z ∈ [2, ⌊√n⌋] evenly divides n.
    // Additionally avoid using sqrt by squaring on both sides.
    //
    // REFERENCES
    //   https://en.wikipedia.org/wiki/Prime_number#Trial_division
    //   https://en.wikipedia.org/wiki/Primality_test
    //   https://stackoverflow.com/q/5811151/183120
    if n <= 1 {
      return false;
    }
    // test clean division with existing primes
    if self
      .primes
      .iter() // take_while stops when predicate is false, while filter doesn’t
      .take_while(|&p| ((p * p) <= n))
      .any(|p| (n % p) == 0)
    {
      return false;
    }
    // none cleanly divided n, store this prime number for future checks
    self.primes.push(n);
    true
  }
}

pub fn nth(n: u32) -> u32 {
  let mut checker = SeqPrimeChecker::new();
  (2..)
    .filter(|z| checker.is_prime(*z))
    .nth(n as usize)
    .unwrap()
  // inspired by https://exercism.io/tracks/rust/exercises/nth-prime/
  //                     solutions/65a44ef3d18a4e7e9fa85605eecb3fc8
}
