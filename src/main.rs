extern crate blockudokill;

use blockudokill::game::{
  all_blocks::AllBlockCells, block::Block, game_board::GameBoard, solver::Solver,
};
use std::io::stdin;

fn main() {
  let mut gameboard = GameBoard::new();
  let allblockcells = AllBlockCells::new();
  println!("{}", allblockcells);
  loop {
    println!("3 blocks:");
    let mut blocks_str = String::new();

    stdin()
      .read_line(&mut blocks_str)
      .expect("Error parsing blocks");

    let block_indices: Vec<usize> = blocks_str
      .split_ascii_whitespace()
      .map(|x| usize::from_str_radix(x, 10).expect("Error parsing block index"))
      .collect();

    let block1 = Block::from_cells(allblockcells.get_block_cells_at(block_indices[0]));
    let block2 = Block::from_cells(allblockcells.get_block_cells_at(block_indices[1]));
    let block3 = Block::from_cells(allblockcells.get_block_cells_at(block_indices[2]));

    let solution = Solver::solve(&mut gameboard, [&block1, &block2, &block3]);
    gameboard.apply_solution(&solution);

    println!("{}", solution);
    println!("{}", gameboard);
    println!("{}\n", gameboard.calc_score());
  }
}
