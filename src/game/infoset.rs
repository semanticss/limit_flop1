use crate::game::state::*;

pub struct InfoSet {
    hole: [i32; 2],  // hero hole cards
    board: Vec<i32>, // or Vec idk
    street: Street,
    pot: u32,
    to_call: u32,
    raises_left: u8,
    position: u8,
    betting_history: Option<Vec<(usize, Action)>>,
}

impl InfoSet {
    // this function should return a list of the betting histories. so,
    // 0. person in first position raised, 1. person after them called 2. person after them folded. etc...
}
