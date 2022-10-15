use super::block::Block;
use std::collections::HashMap;
use std::fmt::{Display, Result};

impl<'a> Display for Solution<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
    let mut output = String::new();
    let listed_out = self.list_out();
    for (_, &(block, _, position)) in listed_out.iter().enumerate() {
      output = output + format!("{}Position: {}\n", block.get_shape(), position).as_str();
    }
    write!(f, "{}", output)
  }
}

pub struct Solution<'a> {
  placement: HashMap<&'a Block, (u8, u8)>,
}

impl<'a> Solution<'a> {
  pub fn new() -> Self {
    Solution {
      placement: HashMap::new(),
    }
  }

  pub fn record(&mut self, block: &'a Block, order: u8, position: u8) {
    self.placement.insert(block, (order, position));
  }

  pub fn list_out(&self) -> Vec<(&Block, u8, u8)> {
    let mut result: Vec<(&Block, u8, u8)> = vec![];
    for (&block, &(order, position)) in self.placement.iter() {
      result.push((block, order, position))
    }
    result.sort_by(|a, b| a.1.cmp(&b.1));
    result
  }
}
