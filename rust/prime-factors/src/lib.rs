/// Extracts all f-ness is n
/// e.g. extract all evenness in n by passing (n, 2)
fn extract(n: &mut u64, f: u64) -> u8 {
  let mut count = 0;
  while *n % f == 0 {
    *n /= f;
    count += 1;
  }
  count
}

pub fn factors(n: u64) -> Vec<u64> {
  let mut m = n;
  let i = extract(&mut m, 2);
  let mut v = vec![2; i as usize];
  let mut f = 3;
  // important optimisation: run only till √n, anything pending will
  // be a prime number; factors of a number can’t be greater
  // https://stackoverflow.com/q/5811151/183120
  while (f * f) <= n {
    let i = extract(&mut m, f);
    v.append(&mut vec![f; i as usize]);
    f += 2;
  }
  // we’ve a prime number pending
  if m != 1 {
    v.push(m);
  }
  v
}
