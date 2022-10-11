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
  // println!("Block1\n{}", block1);
  // let block2 = Block::from_cells(vec![0, 9, 18, 27, 36]);
  // println!("Block2\n{}", block2);
  // if gameboard.is_able_to_place_block(60, &block1) {
  let solution = Solver::solve(&mut gameboard, [&block1, &block2, &block3]);
    // gameboard.place_block(0, &block1);
    // gameboard.place_block(3, &block1);
    // gameboard.place_block(33, &block1);
    // gameboard.place_block(60, &block1);
    // gameboard.place_block(6, &block1);
  println!("{}", solution);

  // }
  // if gameboard.is_able_to_place_block(1, &block1) {
  //   gameboard.place_block(1, &block1);
  // }
  // if gameboard.is_able_to_place_block(1, &block2) {
  //   gameboard.place_block(1, &block2);
  // }
  println!("{}", gameboard);
  println!("{}\n", gameboard.calc_score());
}
