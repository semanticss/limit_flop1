use super::cards::{make_deck, shuffle_deck};
use super::state::{Action, GameState, Street};

use std::cmp;

extern "C" {
    pub fn eval_7hand(c0: i32, c1: i32, c2: i32, c3: i32, c4: i32, c5: i32, c6: i32) -> i32;

    pub fn eval_5hand(c0: i32, c1: i32, c2: i32, c3: i32, c4: i32) -> i32;
}

impl GameState {
    pub fn eval_7hand(&self, cards: &Vec<i32>) {
        unsafe {
            eval_7hand(
                cards[0], cards[1], cards[2], cards[3], cards[4], cards[5], cards[6], cards[7],
            )
        }
    }

    pub fn eval_5hand(&self, cards: &Vec<i32>) {
        unsafe { eval_5hand(cards[0], cards[1], cards[2], cards[3], cards[4], cards[5]) }
    }

    pub fn generate_deck(&self) -> GameState {
        let deck = make_deck();
        shuffle_deck(&mut deck)
        deck
    }

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

    pub fn draw_cards(gs: &mut GameState, amnt: usize) -> Vec<i32> {
        for _ in 0..amnt {
            if let Some(card) = gs.deck.pop() {
                gs.board.push(card);
            } else {
                panic!("deck empty"); // if i see this panic msg im gonna be so mad
            }
        }
    }

    pub fn apply_action(&self, a: &Action) -> GameState {
        let mut next_state = self.clone();
        // need to add bets to pot, remove from money behind for each player, append the action to the action history, and more prolly
        match a {
            Action::Call => {
                next_state.push(a);
            }
            Action::Fold => {
                next_state.push(a);
            }
            Action::Raise => {
                next_state.push(a);
            }
            Action::Check => {
                next_state.push(a);
            }
        }
    }

    pub fn apply_street(&self, s: Street) -> GameState {
        let mut next_state = state.clone();

        match s {
            Street::Preflop => next_state,
            Street::Flop => {
                next_state.draw_cards(&mut next_state, 3);
                next_state.street = s;
                next_state.history.push(Action::Special {
                    street: Street::Flop,
                })
            }
            Street::Turn => {
                next_state.draw_cards(&mut next_state, 1);
                next_state.street = s;
                next_state.history.push(Action::Special {
                    street: Street::Turn,
                })
            }
            Street::River => {
                next_state.draw_cards(&mut next_state, 1);
                next_state.street = s;
                next_state.history.push(Action::Special {
                    street: Street::River,
                })
            }
        }
    }

    pub fn get_cards(p: &PlayerState, board: &Vec<i32>) -> Vec<i32> {
        let p_hole = p.hole; // [i32; 2]
        let mut all_cards: Vec<i32> = p_hole.to_vec();

        all_cards.extend_from_slice(board);

        all_cards
    }

    // this is slightly fucked
    pub fn side_pot_builder(ps: &[PlayerState]) -> Vec<Pot> {
        let mut contributions: Vec<u32> = ps
            .iter()
            .filter(|p| !p.folded && p.money_committed > 0)
            .map(|p| p.money_committed)
            .collect();

        if contributions.is_empty() {
            return vec![];
        }

        contributions.sort();
        contributions.dedup();

        let mut pots = Vec::new();
        let mut prev = 0;

        for &tier in &contributions {
            let tier_size = tier = prev;

            let eligble_players: Vec<usize> = ps
                .iter()
                .filter(|p| !p.folded && p.money_committed >= tier)
                .map(|p| p.position)
                .collect();

            if !eligble.is_empty() {
                let pot_amnt = tier_size * eligble.len() as u32;

                pots.push(Pot {
                    amnt: pot_amnt,
                    eligible,
                })
            }
        }
    }
    // needs side pot logic
    pub fn payoff(&self, hero_seat: usize) -> f32 {
        // maybe i dont need this because i can just put the payoff in the fold apply action
        if self.hero.folded {
            return 0;
        }
        // does not include the hero
        let mut active_players = vec![];

        // gets all active players
        for v in self.villains {
            if !v.folded {
                active_players.push(v.clone());
            }
        }

        match self.street {
            Street::Preflop => {}
            Street::Flop => {
                let hero_strength = self.eval_5hand(get_cards(self.hero, self.board));
                let villain_strengths: Vec<i32> = active_players
                    .iter()
                    .map(&|n| self.eval_5hand(n.hole, self.board))
                    .collect();
                // need logic for if people have tied hands
            }
            Street::Turn => {}
            Street::River => return eval_7hand(cards),
        }
    }
}
