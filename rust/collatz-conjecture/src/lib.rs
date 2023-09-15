pub fn collatz(n: u64) -> Option<u64> {
    let (mut i, mut m) = (0, n);
    while m != 1 {
        m = match (m, m % 2) {
            (0, _) => return None,
            (_, 0) => m / 2,
            (_, 1) => m.checked_mul(3)?.checked_add(1)?,
            _ => unreachable!()
        };
        i += 1;
    }
    Some(i)
}
