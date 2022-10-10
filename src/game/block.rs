use std::fmt::{Display, Result};

pub struct Block {
  cells: Vec<u8>,
  width: u8,
  height: u8,
}

impl Display for Block {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
    let mut serialized_cells = String::new();
    let mut iterator = self.cells.iter();
    while let Some(ref_val) = iterator.next() {
      serialized_cells = serialized_cells + ref_val.to_string().as_str();
      serialized_cells.push(' ');
    }
    let output = format!(
      "Width: {}\nHeight: {}\nCells: {}\n",
      self.width, self.height, serialized_cells
    );
    write!(f, "{}", output)
  }
}

impl Block {
  pub fn from_cells(cells: Vec<u8>) -> Self {
    let mut width: u8 = 0;
    let mut height: u8 = 0;
    for (_, &val) in cells.iter().enumerate() {
      let x = (val % 9) + 1;
      let y = (val / 9) + 1;
      width = if x > width { x } else { width };
      height = if y > height { y } else { height };
    }
    Block {
      cells,
      width,
      height,
    }
  }

  pub fn get_cells(&self) -> &Vec<u8> {
    &self.cells
  }

  pub fn get_width(&self) -> u8 {
    self.width
  }

  pub fn get_height(&self) -> u8 {
    self.height
  }
}
