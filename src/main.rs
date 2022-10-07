pub mod game;

use game::{
  game_board::GameBoard,
  block::Block,
};

fn main() {
  let mut gameboard = GameBoard::new();
  let block1 = Block::from_cells(vec![2, 9, 10, 11, 20]);
  // let block2 = Block::from_cells(vec![0, 9, 18, 27, 36]);
  if gameboard.is_able_to_place_block(30, &block1) {
    gameboard.place_block(57, &block1);
  }
  // if gameboard.is_able_to_place_block(1, &block1) {
  //   gameboard.place_block(1, &block1);
  // }
  // if gameboard.is_able_to_place_block(1, &block2) {
  //   gameboard.place_block(1, &block2);
  // }
  println!("{}", gameboard);
  println!("{}", gameboard.calc_score());
}
