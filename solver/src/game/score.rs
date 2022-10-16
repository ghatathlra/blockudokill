use std::clone::Clone;
use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::fmt::{Display, Formatter, Result};
use std::marker::Copy;

#[derive(Clone, Copy)]
pub struct Score {
  pub spaces: u8,
  pub holes: u8,
  pub bonus: f32,
}

impl Display for Score {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    let output = format!(
      "Spaces: {}, Height: {}, Bonus: {}",
      self.spaces, self.holes, self.bonus
    );
    write!(f, "{}", output)
  }
}

impl PartialEq for Score {
  fn eq(&self, other: &Self) -> bool {
    self.spaces == other.spaces && self.holes == other.holes && self.bonus == other.bonus
  }

  fn ne(&self, other: &Self) -> bool {
    !self.eq(other)
  }
}

impl Eq for Score {}

impl PartialOrd for Score {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for Score {
  fn cmp(&self, other: &Self) -> Ordering {
    let result = if self.spaces == other.spaces {
      match self.holes.cmp(&other.holes) {
        Ordering::Less => Ordering::Greater,
        Ordering::Greater => Ordering::Less,
        Ordering::Equal => self.bonus.total_cmp(&other.bonus),
      }
    } else {
      self.spaces.cmp(&other.spaces)
    };
    result
  }
}

impl Score {
  pub fn new() -> Self {
    Score {
      spaces: 0,
      holes: 0,
      bonus: 0.0,
    }
  }
}
