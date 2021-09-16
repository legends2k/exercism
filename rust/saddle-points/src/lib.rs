// All elements except the tested one are run through the closure;
// returns true if the closure never returned false
fn satisfies<F: Fn(u64) -> bool>(idx: usize, v: &Vec<u64>, cmp: F) -> bool {
  v.iter()
    .enumerate()
    .filter(|(i, _)| *i != idx)
    .all(|(_, &e)| cmp(e))
}

pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
  let mut saddles: Vec<(usize, usize)> = vec![];
  for (r, row) in input.iter().enumerate() {
    for (c, &e) in row.iter().enumerate() {
      if satisfies(c, &row, |i| e >= i) {
        println!("({}, {})", c, r);
        let col: Vec<u64> = input.iter().map(|v| v[c]).collect();
        if satisfies(r, &col, |i| e <= i) {
          saddles.push((r, c));
        }
      }
    }
  }
  saddles
}
