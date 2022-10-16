use super::{block::Block, game_board::GameBoard, solution::Solution, score::Score};

pub struct Solver;

impl Solver {
  pub fn solve<'a>(gameboard: &mut GameBoard, blocks: [&'a Block; 3]) -> Solution<'a> {
    let mut best_score: Score = Score::new();
    let mut solution = Solution::new();
    let block_permutations = Solver::generate_block_permutations(blocks);
    for (_, &blckpermutation) in block_permutations.iter().enumerate() {
      let (score, (p1, p2, p3)) =
        Solver::solve_on_one_block_permutation(gameboard, blckpermutation);
      if score > best_score {
        best_score = score;
        solution.record(blckpermutation[0], 0, p1);
        solution.record(blckpermutation[1], 1, p2);
        solution.record(blckpermutation[2], 2, p3);
      }
    }
    solution
  }

  fn solve_on_one_block_permutation<'a>(
    gameboard: &mut GameBoard,
    blocks: [&'a Block; 3],
  ) -> (Score, (u8, u8, u8)) {
    let mut best_state: (Score, (u8, u8, u8)) = (Score::new(), (0, 0, 0));
    let gameboard_body = gameboard.get_body();

    for (i, _) in gameboard_body.iter().enumerate() {
      if gameboard.is_able_to_place_block(i as u8, blocks[0]) {
        gameboard.place_block(i as u8, blocks[0]);

        for (j, _) in gameboard_body.iter().enumerate() {
          if gameboard.is_able_to_place_block(j as u8, blocks[1]) {
            gameboard.place_block(j as u8, blocks[1]);

            for (k, _) in gameboard_body.iter().enumerate() {
              if gameboard.is_able_to_place_block(k as u8, blocks[2]) {
                gameboard.place_block(k as u8, blocks[2]);
                let current_score = gameboard.calc_score();
                if current_score > best_state.0 {
                  best_state = (current_score, (i as u8, j as u8, k as u8))
                }
                gameboard.rollback();
              }
            }

            gameboard.rollback();
          }
        }

        gameboard.rollback();
      }
    }
    best_state
  }

  fn generate_block_permutations<'a>(blocks: [&'a Block; 3]) -> [[&'a Block; 3]; 6] {
    [
      [blocks[0], blocks[1], blocks[2]],
      [blocks[0], blocks[2], blocks[1]],
      [blocks[1], blocks[0], blocks[2]],
      [blocks[1], blocks[2], blocks[0]],
      [blocks[2], blocks[0], blocks[1]],
      [blocks[2], blocks[1], blocks[0]],
    ]
  }
}
