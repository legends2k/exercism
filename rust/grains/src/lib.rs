pub fn square(s: u32) -> u64 {
  match s {
    1..=64 => 1u64 << (s - 1),
    _ => panic!("Square must be between 1 and 64"),
  }
}

// Summation of a Geometric Series
// a + ar² + ar³ + … + arⁿ⁻¹ = a(1 - rⁿ) / (1 - r)
// https://en.wikipedia.org/wiki/Geometric_series#Sum
// Essentially the total is u64::MAX = 1 + 2 + … + 2⁶³ = 2⁶⁴ - 1
// With n-bits we can represent 2ⁿ states; precisely the values [0, 2ⁿ-1]
pub fn total() -> u64 {
  std::u64::MAX
}
