use std::fmt::{Display, Result};

pub struct Block {
  cells: Vec<u8>,
}

impl Display for Block {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
    let mut output = String::new();
    let mut iterator = self.cells.iter();
    while let Some(ref_val) = iterator.next() {
      let value = *ref_val as u32;
      output.push(char::from_digit(value, 10).unwrap_or('?'));
      output.push(' ');
    }
    write!(f, "{}", output)
  }
}

impl Block {
  pub fn from_cells(cells: Vec<u8>) -> Self {
    Block { cells }
  }

  pub fn get_cells(&self) -> &Vec<u8> {
    &self.cells
  }
}
