pub mod game;

use game::{
  game_board::GameBoard,
  block::Block,
  solver::Solver,
};

fn main() {
  let mut gameboard = GameBoard::new();
  let block1 = Block::from_cells(vec![2, 9, 10, 11, 20]);
  let block2 = Block::from_cells(vec![0, 1, 2, 3, 4]);
  let block3 = Block::from_cells(vec![0, 9, 18, 27, 36]);

  let solution = Solver::solve(&mut gameboard, [&block1, &block2, &block3]);
  gameboard.apply_solution(&solution);

  println!("{}", solution);
  println!("{}", gameboard);
  println!("{}\n", gameboard.calc_score());
}
