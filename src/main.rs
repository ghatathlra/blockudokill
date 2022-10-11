pub mod game;

use game::{block::Block, game_board::GameBoard, solver::Solver};
use std::io::stdin;

fn main() {
  let mut gameboard = GameBoard::new();
  loop {
    println!("3 blocks:");
    let mut block1_str = String::new();
    let mut block2_str = String::new();
    let mut block3_str = String::new();

    stdin()
      .read_line(&mut block1_str)
      .expect("Error parsing body of block 1");
    stdin()
      .read_line(&mut block2_str)
      .expect("Error parsing body of block 2");
    stdin()
      .read_line(&mut block3_str)
      .expect("Error parsing body of block 3");

    let block1_vec: Vec<u8> = block1_str
      .split_ascii_whitespace()
      .map(|x| u8::from_str_radix(x, 10).expect("Error parsing block 1"))
      .collect();
    let block2_vec: Vec<u8> = block2_str
      .split_ascii_whitespace()
      .map(|x| u8::from_str_radix(x, 10).expect("Error parsing block 2"))
      .collect();
    let block3_vec: Vec<u8> = block3_str
      .split_ascii_whitespace()
      .map(|x| u8::from_str_radix(x, 10).expect("Error parsing block 3"))
      .collect();

    let block1 = Block::from_cells(block1_vec);
    let block2 = Block::from_cells(block2_vec);
    let block3 = Block::from_cells(block3_vec);

    let solution = Solver::solve(&mut gameboard, [&block1, &block2, &block3]);
    gameboard.apply_solution(&solution);

    println!("{}", solution);
    println!("{}", gameboard);
    println!("{}\n", gameboard.calc_score());
  }
}
