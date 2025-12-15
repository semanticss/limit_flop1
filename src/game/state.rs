use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Street {
    Preflop,
    Flop,
    Turn,
    River,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Action {
    Fold { position: usize },
    Call { amnt: f32, position: usize },
    Raise { amnt: f32, position: usize },
    Check { position: usize },
    Special { street: Street },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Pot {
    pub amnt: u32,
    pub eligble: Vec<usize>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlayerState {
    pub hole: [i32; 2],
    pub money_behind: f32,
    pub folded: bool,
    pub money_committed: f32,
    pub position: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
// maybe we dont care if someone has folded, but whether they are eligible for a pot?
pub struct GameState {
    pub hero_idx: u8,
    pub players: Vec<PlayerState>, // length 6, hero is stored here
    pub deck: Vec<i32>,
    pub board: Vec<i32>,
    pub pot: Pot, // only the big pot, no side pots included
    pub side_pots: Vec<Pot>,
    pub to_call: i32,   // maybe dont need this
    pub street: Street, // could use just numbers
    pub next_player: usize,
    // pub bets_on_street_curr: u8,
    pub is_chance_node: bool,
    pub is_terminal: bool,
    // pub bets_remaining: i32,
    pub history: Vec<Action>,
}
