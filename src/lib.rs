mod ext;
pub mod game;

use ext::{Placement, SolveResult};
use game::{block::Block, game_board::GameBoard, solver::Solver};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn solve(
  gameboard_body: Vec<u8>,
  block1_body: Vec<u8>,
  block2_body: Vec<u8>,
  block3_body: Vec<u8>,
) -> JsValue {
  let mut gameboard = GameBoard::from_cells(gameboard_body);
  let block1 = Block::from_cells(block1_body);
  let block2 = Block::from_cells(block2_body);
  let block3 = Block::from_cells(block3_body);

  let solution = Solver::solve(&mut gameboard, [&block1, &block2, &block3]);
  gameboard.apply_solution(&solution);

  let listed_out_solution = solution.list_out();
  let placements: Vec<Placement> = listed_out_solution
    .iter()
    .map(|&(bl, ord, pos)| Placement {
      block: bl.get_cells(),
      order: ord,
      position: pos,
    })
    .collect();
  let solve_result = SolveResult {
    placements,
    next_state: gameboard.get_cells(),
  };
  to_value(&solve_result).unwrap()
}
