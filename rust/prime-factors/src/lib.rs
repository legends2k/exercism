use std::iter;

/// Extracts all f-ness is n
/// e.g. extract all evenness in n by passing `(n, 2)`
fn extract(n: &mut u64, f: u64) -> u32 {
  let mut count = 0;
  while *n % f == 0 {
    *n /= f;
    count += 1;
  }
  count
}

// Basic Idea is to iterate over (2..=n), test for clean divisibility, divide
// and append factor.  However, once factor 2 iteration is done, all further
// divisiors/factors will be odd, so idx f can be double incremented.  Special
// case 2 and do regular iteration from 3 onwards.  Note: we don’t check if f is
// prime since we’re iterating from 2, we’d’ve already tried to divide n by
// prime factors some composite/non-prime number is composed of.
//
// REFERENCES
//     https://en.wikipedia.org/wiki/Integer_factorization
//     https://en.wikipedia.org/wiki/Trial_division
//     https://www.geeksforgeeks.org/print-all-prime-factors-of-a-given-number/
pub fn factors(n: u64) -> Vec<u64> {
  let mut m = n;
  let i = extract(&mut m, 2);
  let mut v = vec![2; i as usize];
  let mut f = 3;
  // Optimisation 1: run only till √n as factors of a number can’t be
  // greater. Anything pending will be a prime number.
  // https://stackoverflow.com/q/5811151/183120
  while (f * f) <= n {
    let i = extract(&mut m, f);
    v.extend(iter::repeat(f).take(i as usize));
    f += 2;
    // Optimisation 2: skip even numbers since all evenness
    // has been extracted from n.
  }
  // we’ve a prime number pending
  if m != 1 {
    v.push(m);
  }
  v
}
