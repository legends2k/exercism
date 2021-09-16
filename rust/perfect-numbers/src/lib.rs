#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
  Abundant,
  Perfect,
  Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
  if num == 0 {
    return None;
  }
  match (1..=(num / 2)).filter(|f| num % f == 0).sum::<u64>() {
    x if x == num => Some(Classification::Perfect),
    x if x > num => Some(Classification::Abundant),
    x if x < num => Some(Classification::Deficient),
    _ => None,
  }
}
