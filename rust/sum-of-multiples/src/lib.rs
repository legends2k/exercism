pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
  let mut v: Vec<u32> = factors
    .iter()
    .filter(|&f| *f > 0) // step_by can’t be 0
    .flat_map(|&f| (f..).step_by(f as usize).take_while(|f| *f < limit))
    .collect();
  v.sort_unstable();
  v.dedup();
  v.iter().sum()
}
