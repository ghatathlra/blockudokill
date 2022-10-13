use super::{
  block::Block, ripe_cases::RipeCases, score_distribution::ScoreDistribution, solution::Solution,
};
use std::fmt::Display;

pub struct GameBoard {
  body: [u8; 81],
  history: Vec<[u8; 81]>,
}

impl Display for GameBoard {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut output = String::new();
    for i in (0..self.body.len()).step_by(9) {
      for j in 0..9 {
        let val = self.body[i + j];
        output.push(if val == 0 { '.' } else { '\u{2588}' });
      }
      output.push('\n');
    }
    write!(f, "{}", output)
  }
}

impl GameBoard {
  pub fn new() -> Self {
    GameBoard {
      body: [0; 81],
      history: vec![],
    }
  }

  pub fn from_cells(cells: Vec<u8>) -> Self {
    let mut body: [u8; 81] = [0; 81];
    for (_, &cell) in cells.iter().enumerate() {
      body[cell as usize] = 1;
    }
    GameBoard {
      body,
      history: vec![],
    }
  }

  pub fn get_cells(&self) -> Vec<u8> {
    let mut cells: Vec<u8> = vec![];
    for (i, &val) in self.body.iter().enumerate() {
      if val == 1 {
        cells.push(i as u8);
      }
    }
    cells
  }

  pub fn place_block(&mut self, place_position: u8, block: &Block) -> () {
    let cells = block.get_cells();
    for (_, &val) in cells.iter().enumerate() {
      self.body[place_position as usize + val as usize] = 1;
    }
    self.harvest();
    self.commit();
  }

  pub fn is_able_to_place_block(&self, place_position: u8, block: &Block) -> bool {
    let mut is_able: bool = true;
    let x = place_position % 9;
    let y = place_position / 9;
    let cells = block.get_cells();

    if x + block.get_width() > 9 || y + block.get_height() > 9 {
      is_able = false;
    } else {
      for (_, &val) in cells.iter().enumerate() {
        if self.body[place_position as usize + val as usize] == 1 {
          is_able = false;
        }
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

  fn harvest(&mut self) -> () {
    let mut satisfied_ripe_cases: Vec<&[u8; 9]> = vec![];
    for (_, case) in RipeCases::get_cases().iter().enumerate() {
      if self.is_satisfied_ripe_case(case) {
        satisfied_ripe_cases.push(case);
      }
    }
    self.harvest_ripe_cases(&satisfied_ripe_cases);
  }

  fn is_satisfied_ripe_case(&self, ripe_case: &[u8; 9]) -> bool {
    let mut is_satisfied = true;
    for (_, &position) in ripe_case.iter().enumerate() {
      if self.body[position as usize] == 0 {
        is_satisfied = false;
      }
    }
    is_satisfied
  }

  fn harvest_ripe_cases(&mut self, ripe_cases: &Vec<&[u8; 9]>) -> () {
    let mut ripe_cases_iter = ripe_cases.iter();
    while let Some(&ripe_case) = ripe_cases_iter.next() {
      for (_, &position) in ripe_case.iter().enumerate() {
        self.body[position as usize] = 0;
      }
    }
  }

  fn commit(&mut self) -> () {
    self.history.push(self.body);
  }

  pub fn rollback(&mut self) -> () {
    self.history.pop();
    match self.history.last() {
      Some(&previous_body) => self.body = previous_body,
      None => self.body = [0; 81],
    }
  }

  pub fn get_body(&self) -> [u8; 81] {
    self.body
  }

  pub fn apply_solution(&mut self, solution: &Solution) -> () {
    let listed_solution = solution.list_out();
    for (_, &(block, _, position)) in listed_solution.iter().enumerate() {
      self.place_block(position, block);
    }
  }
}
