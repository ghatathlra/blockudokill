use super::block::Block;
use std::fmt::{Display, Formatter, Result};

pub struct AllBlockCells {
  blockcells: [Vec<u8>; 47],
}

impl Display for AllBlockCells {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    let mut output = String::new();
    for (i, blockcells) in self.blockcells.iter().enumerate() {
      output.push_str(
        format!(
          "{}\n{}",
          i,
          Block::from_cells(blockcells.clone()).get_shape()
        )
        .as_str(),
      );
    }
    write!(f, "{}", output)
  }
}

impl AllBlockCells {
  pub fn new() -> Self {
    AllBlockCells {
      blockcells: [
        vec![0],
        vec![0, 1],
        vec![0, 9],
        vec![0, 10],
        vec![1, 9],
        vec![0, 1, 2],
        vec![0, 9, 18],
        vec![0, 1, 9],
        vec![0, 1, 10],
        vec![0, 9, 10],
        vec![1, 9, 10],
        vec![0, 10, 20],
        vec![2, 10, 18],
        vec![0, 1, 9, 10],
        vec![0, 1, 10, 11],
        vec![1, 9, 10, 18],
        vec![1, 2, 9, 10],
        vec![0, 9, 10, 19],
        vec![0, 1, 2, 3],
        vec![0, 9, 18, 27],
        vec![0, 1, 2, 11],
        vec![1, 10, 18, 19],
        vec![0, 9, 10, 11],
        vec![0, 1, 9, 18],
        vec![0, 1, 2, 9],
        vec![0, 9, 18, 19],
        vec![2, 9, 10, 11],
        vec![0, 1, 10, 19],
        vec![0, 1, 2, 10],
        vec![1, 9, 10, 19],
        vec![1, 9, 10, 11],
        vec![0, 9, 10, 18],
        vec![0, 1, 2, 3, 4],
        vec![0, 9, 18, 27, 36],
        vec![0, 1, 2, 9, 18],
        vec![0, 9, 18, 19, 20],
        vec![2, 11, 18, 19, 20],
        vec![0, 1, 2, 11, 20],
        vec![0, 1, 2, 10, 19],
        vec![2, 9, 10, 11, 20],
        vec![1, 10, 18, 19, 20],
        vec![0, 9, 10, 11, 18],
        vec![0, 1, 2, 9, 11],
        vec![0, 1, 10, 18, 19],
        vec![0, 2, 9, 10, 11],
        vec![0, 1, 9, 18, 19],
        vec![1, 9, 10, 11, 19],
      ],
    }
  }

  pub fn get_block_cells_at(&self, i: usize) -> Vec<u8> {
    self.blockcells[i].clone()
  }
}
