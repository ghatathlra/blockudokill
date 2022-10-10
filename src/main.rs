pub mod game;

use game::{
  game_board::GameBoard,
  block::Block,
};

fn main() {
  let mut gameboard = GameBoard::new();
  let block1 = Block::from_cells(vec![2, 9, 10, 11, 20]);
  // println!("Block1\n{}", block1);
  // let block2 = Block::from_cells(vec![0, 9, 18, 27, 36]);
  // println!("Block2\n{}", block2);
  // if gameboard.is_able_to_place_block(60, &block1) {
    gameboard.place_block(0, &block1);
    gameboard.place_block(3, &block1);
    gameboard.place_block(33, &block1);
    gameboard.place_block(60, &block1);
    gameboard.place_block(6, &block1);

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
