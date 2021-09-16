// 6k + 1 Primality Test
// Primality test optimisations employed here follows from two important
// observations:
//
//   1. Not all numbers from [2, n-1] need to be tested; integers > √n needn’t
//      be checked as, when n = a × b, one of the two factors a or b ≤ √n;
//      otherwise one needs to be a real number; however for primality we need
//      both to be integers.
//   2. Only prime divisors need to be checked; however, we don’t wanna generate
//      primes when checking for one; instead since all primes above 5 can be
//      written as 6k±1, check for divisibility by 2, 3 and all numbers of the
//      form 6k±1 < ⌊√n⌋.
//   3. Squaring both sides gets rid of the expensive `sqrt` function
//
// (2) follows from the fact that all integers can be written as 6k+i for some k
// and i ∈ {-1, 0, 1, 2, 3, 4}.  2 cleanly divides (6k+0), (6k+2), (6k+4) and
// 3 cleanly divides (6k+3), leaving us with 6k±1.
//
// REFERENCES
//   https://en.wikipedia.org/wiki/Primality_test
//   https://stackoverflow.com/q/5811151/183120
//
#[allow(dead_code)]
fn is_prime(n: u64) -> bool {
  match n {
    0 | 1 => false,
    2 | 3 => true,
    // https://blog.rust-lang.org/2015/04/17/Enums-match-mutation-and-moves.html
    _ => match (n % 2, n % 3) {
      (0, _) | (_, 0) => false,
      _ => (5..) // instead of checking from 2 to n-1, check only till √n
        .step_by(6) // further, i <= n.sqrt() can be written (i * i) <= n
        .take_while(|f| (f * f) <= n)
        .all(|f| (n % f) != 0 && (n % (f + 2) != 0)),
    },
  }
}

struct SeqPrimeChecker {
  primes: Vec<u32>,
}

// NOTE: This checker wouldn’t work for non-sequential queries; use above fn.
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
