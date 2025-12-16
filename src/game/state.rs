use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Street {
    Preflop,
    Flop,
    Turn,
    River,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Action {
    Fold,
    Call,
    Raise,
    Check,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Pot {
    pub amnt: u32,
    pub eligble: Vec<usize>,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct PlayerState {
    pub idx: u8,
    pub hole: [i32; 2],
    pub money_behind: u32,
    pub money_committed_curr_round: u32,
    pub total_committed: u32,
    pub folded: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameState {
    pub players: Vec<PlayerState>,
    pub curr_idx: usize,

    pub street: Street,
    pub current_bet: u32,
    pub raises_left: u8,
    pub pot: u32,
    pub side_pots: Vec<Pot>,
    pub deck: Vec<i32>,
    pub board: Vec<i32>,

    pub hero_idx: usize,
}
