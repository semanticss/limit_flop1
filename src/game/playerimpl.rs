use super::state::PlayerState;

impl PlayerState {
    pub fn default() -> Self {
        Self {
            hole: [0, 0],
            money_behind: 100,
            folded: false,
            money_committed: 0,
            position: 0,
        }
    }

    pub fn set_hole_cards(&mut self, cards: [i32; 2]) {
        self.hole = cards;
    }

    pub fn fold(&mut self) {
        self.folded = true;
    }

    pub fn set_position(&mut self, pos: u8) {
        self.position = pos;
    }
}
