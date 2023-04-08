use std::ops::Add;

pub struct Progress {
  pub correct: usize,
  pub incorrect: usize,
  pub total: usize,
}

impl Add for Progress {
  type Output = Self;
  fn add(self, other: Self) -> Self {
    Progress {
      correct: self.correct + other.correct,
      incorrect: self.incorrect + other.incorrect,
      total: self.total + other.total,
    }
  }
}
