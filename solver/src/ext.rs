use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Placement {
  pub block: Vec<u8>,
  pub order: u8,
  pub position: u8,
}

#[derive(Serialize, Deserialize)]
pub struct SolveResult {
  pub placements: Vec<Placement>,
  pub next_state: Vec<u8>,
}