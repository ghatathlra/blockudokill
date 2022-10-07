use super::{block::Block, score_distribution::ScoreDistribution};
use std::fmt::Display;

pub struct GameBoard {
  body: [u8; 81],
}

impl Display for GameBoard {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut output = String::new();
    for i in (0..self.body.len()).step_by(9) {
      for j in 0..8 {
        let val = self.body[i + j];
        output.push(if val == 0 { '.' } else { '#' });
      }
      output.push('\n');
    }
    write!(f, "{}", output)
  }
}

impl GameBoard {
  pub fn new() -> Self {
    GameBoard { body: [0; 81] }
  }

  pub fn place_block(&mut self, place_position: u8, block: &Block) -> &Self {
    let cells = block.get_cells();
    for (_, &val) in cells.iter().enumerate() {
      self.body[place_position as usize + val as usize] = 1;
    }
    self
  }

  pub fn is_able_to_place_block(&mut self, place_position: u8, block: &Block) -> bool {
    let mut is_able: bool = true;
    let cells = block.get_cells();
    for (_, &val) in cells.iter().enumerate() {
      if self.body[place_position as usize + val as usize] == 1 {
        is_able = false;
      }
    }
    is_able
  }

  pub fn calc_score(&self) -> f32 {
    let mut score: f32 = 0.0;
    for (i, &val) in self.body.iter().enumerate() {
      if val == 0 {
        score += 1.0 + ScoreDistribution::get_score_at(i as u8);
      }
    }
    score
  }
}
