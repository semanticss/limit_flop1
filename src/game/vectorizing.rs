use crate::game::state::*;

const MAX_BOARD: usize = 5;
const HISTORY_LEN: usize = 50;
const VECTOR_SIZE: usize = 52 + 52 + 4 + 2 + 1 + (HISTORY_LEN * 2);
const MAX_POT: f32 = 20000.0;

pub struct InfoSet {
    hole: [i32; 2], // hero hole cards
    board: Vec<i32>,
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
    pub fn vectorize(&self) -> Vec<f32> {
        let mut vec = vec![0.0; VECTOR_SIZE];
        let mut cursor = 0;

        // indices 0-51 are for the hole cards
        for &card in &self.hole {
            vec[cursor + card as usize] = 1.0;
        }
        cursor += 52;

        for &card in &self.board {
            vec[cursor + card as usize] = 1.0;
        }
        cursor += 52;

        let street_int = match self.street {
            Street::Preflop => 0,
            Street::Flop => 1,
            Street::Turn => 2,
            Street::River => 3,
        };

        vec[cursor + street_int] = 1.0;

        cursor += 4;

        let normal_max_pot = MAX_POT.ln();

        todo!("finish normalizing and then add betting history");
    }
}
