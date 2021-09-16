use std::ops::Sub;

#[derive(Debug, Clone, Copy)]
pub struct ChessPosition {
  x: i32,
  y: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct Queen {
  pos: ChessPosition,
}

impl ChessPosition {
  pub fn new(rank: i32, file: i32) -> Option<Self> {
    match (rank, file) {
      (0..=7, 0..=7) => Some(ChessPosition { x: rank, y: file }),
      _ => None,
    }
  }
}

impl Sub for ChessPosition {
  type Output = [i32; 2];
  fn sub(self, other: ChessPosition) -> [i32; 2] {
    [self.x - other.x, self.y - other.y]
  }
}

impl Queen {
  pub fn new(position: ChessPosition) -> Self {
    Queen { pos: position }
  }

  pub fn can_attack(&self, other: &Queen) -> bool {
    let d = self.pos - other.pos;
    (d[0] == 0) || (d[1] == 0) || (d[0].abs() == d[1].abs())
  }
}
