pub fn is_leap_year(year: u64) -> bool {
  // https://news.ycombinator.com/item?id=21928681
  let factor = match year % 100 {
    0 => year / 100,
    x => x,
  };
  (factor % 4) == 0
}
