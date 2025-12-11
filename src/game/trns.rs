use super::state::{Action, GameState, Street};

extern "C" {
    pub fn eval_7hand(c0: i32, c1: i32, c2: i32, c3: i32, c4: i32, c5: i32, c6: i32) -> i32;
}

impl GameState {
    pub fn eval_7hand(&self, cards: &[i32]) {
        unsafe {
            eval_7hand(
                cards[0], cards[1], cards[2], cards[3], cards[4], cards[5], cards[6], cards[7],
            )
        }
    }
    pub fn generate_deck(&self) -> GameState {}
    pub fn legal_actions(&self) -> Vec<Action> {
        // if u are the first to act you can either call the bb, raise, or fold
        if gs.history.is_empty() {
            return vec![Call, Raise, Fold];
        }

        // if the person behind u checked, u can check raise or fold
        if gs.history.last() == Check {
            return vec![Check, Raise, Fold];
        }

        // also, if you are the first to act in a hand, you can check. issue is if the person in the previous hand called, then you should also be able to check. for this case, i am going to create a special action called Special that indicates a street has passed between betting.

        if gs.history.last() == Special {
            return vec![Check, Raise, Fold];
        }

        // idk i dont think the action should ever get back to someone who is all in
    }

    pub fn apply_action(&self, a: &Action) -> GameState {
        let mut next_state = self.clone();

        match a {
            Action::Call => {}
            Action::Fold => {}
            Action::Raise => {}
            Action::Check => {}
        }
    }

    pub fn apply_street(&self, s: Street) -> GameState {
        let mut next_state = state.clone();

        // take card from self.deck, remove it from the deck, add it to the board

        // use Some in remaining arms
        match s {
            Street::Preflop => None,
            Street::Flop => {}
            Street::Turn => {}
            Street::River => {}
        }
    }

    pub fn is_terminal(&self) -> bool {}

    // this only works if we have gotten to the river
    pub fn payoff(&self) -> f32 {
        let all_cards: Vec<i32> = self
            .hole
            .iter()
            .map(|&card| card as i32)
            .chain(self.board.iter().copied())
            .collect();

        self.eval_7_cards(&all_cards) as f32
    }
}
