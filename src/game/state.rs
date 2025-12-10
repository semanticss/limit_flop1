use serde::{Deseralize, Serialize};

#[derive(Clone, Debug, Serialize, Deseralize)]
pub enum Street {
    Preflop,
    Flop,
    Turn,
    River
}
#[derive(Clone, Debug, Serialize, Deseralize)]
pub enum Actions {
    Fold,
    Call,
    Raise,
    Check,
}


#[derive(Clone, Debug, Serialize, Deseralize)]
pub struct GameState {
    pub hole_p0: [u8; 2],
    pub hole_p1: [u8; 2],
    pub board: Vec<u8>,
    pub pot: i32,
    pub to_call: i32,
    pub street: Street, // could use just numbers
    pub next_player: usize,
    pub bets_on_street_curr: u8,
    pub is_chance_node: bool,
    pub is_terminal: bool,
    pub bets_remaining: i32,
    pub history: Vec<Action>,
}