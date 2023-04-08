use std::ops::Add;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Coord {
  pub row: u16,
  pub col: u16,
}

impl Add for Coord {
  type Output = Self;
  fn add(self, other: Self) -> Self {
    Coord {
      row: self.row + other.row,
      col: self.col + other.col,
    }
  }
}
