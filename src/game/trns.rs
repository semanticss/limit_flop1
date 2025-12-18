// this file determines the next gamestate given the current gamestate

use super::cards::{make_deck, shuffle_deck};
use super::state::{Action, GameState, Street};

use std::cmp;

extern "C" {
    pub fn eval_7hand(c0: i32, c1: i32, c2: i32, c3: i32, c4: i32, c5: i32, c6: i32) -> i32;
}

impl GameState {
    pub fn eval_7hand(&self, cards: &[i32; 7]) -> i32 {
        unsafe {
            eval_7hand(
                cards[0], cards[1], cards[2], cards[3], cards[4], cards[5], cards[6],
            )
        }
    }

    pub fn raise_size(&self) -> u32 {
        match self.street {
            Street::Preflop | Street::Flop => 1,
            Street::Turn | Street::River => 2,
        }
    }

    pub fn legal_actions(&self) -> Vec<Action> {
        let p = &self.players[self.curr_idx];

        if p.folded || p.money_behind == 0 {
            return vec![];
        }

        let to_call = self
            .current_bet
            .saturating_sub(p.money_committed_curr_round);

        // case 1: facing no bet

        if to_call == 0 {
            let mut actions = vec![Action::Check];
            if self.raises_left > 0 && p.money_behind > 0 {
                actions.push(Action::Raise);
            }
            return actions;
        }

        //case 2: facing a bet

        let mut actions = vec![Action::Fold, Action::Call];
        if self.raises_left > 0 && p.money_behind > to_call {
            actions.push(Action::Raise);
        }

        actions
    }

    // chance node
    pub fn draw_cards(&mut self, amnt: usize) {
        for _ in 0..amnt {
            let card = self.deck.pop().expect("deck empty");
            self.board.push(card);
        }
    }

    pub fn advance_player(&mut self) {
        let mut next_state = self.clone();
    }

    pub fn betting_round_complete(&self) -> bool {
        for p in &self.players {
            if p.folded {
                continue;
            }

            if p.money_behind > 0 && p.money_committed_curr_round != self.current_bet {
                return false;
            }
        }
        true
    }

    // unfinished
    pub fn apply_action(&self, a: &Action) -> GameState {
        let mut next_state = self.clone();
        let idx = next_state.curr_idx;
        let player = &mut next_state.players[idx];
        // need to add bets to pot, remove from money behind for each player, append the action to the action history, and more prolly
        match a {
            Action::Call => {
                let to_call = next_state.current_bet - player.money_committed_curr_round;
                let amount = to_call.min(player.money_behind);

                player.money_behind -= amount;
                player.money_committed_curr_round += amount;
                player.total_committed += amount;
                next_state.pot += amount;
            }
            Action::Fold => {
                player.folded = true;
            }
            Action::Raise => {
                let raise_amount = next_state.raise_size();
                let to_call = next_state.current_bet - player.money_committed_curr_round;
                let total = to_call + raise_amount;

                let paid = total.min(player.money_behind);

                player.money_behind -= paid;
                player.committed_this_round += paid;
                player.total_committed += paid;

                next_state.current_bet += raise_amount;
                next_state.raises_left -= 1;
                next_state.last_raiser_idx = Some(idx);
                next_state.pot += paid;
            }
            Action::Check => {}
        }

        next_state.move_turn();
        next_state
    }

    pub fn move_turn(&mut self) {
        let n = self.players.len();

        loop {
            self.curr_idx = (self.curr_idx + 1) % n;
            let p = &self.players[self.curr_idx];

            if !p.folded && p.money_behind > 0 {
                break;
            }
        }
    }

    pub fn apply_street(&mut self) {
        for p in &mut self.players {
            p.money_committed_curr_round = 0;
        }

        self.current_bet = 0;
        self.raises_left = 3; // this could be wrong
        self.last_raiser_idx = None;

        self.street = match self.street {
            Street::Preflop => Street::Flop,
            Street::Flop => Street::Turn,
            Street::Turn => Street::River,
            Street::River => panic!("mf cmon"),
        }
    }

    pub fn get_cards(p: &PlayerState, board: &Vec<i32>) -> Vec<i32> {
        let p_hole = p.hole; // [i32; 2]
        let mut all_cards: Vec<i32> = p_hole.to_vec();

        all_cards.extend_from_slice(board);

        all_cards
    }

    // maybe this works
    pub fn side_pot_builder(&mut self) {
        self.side_pots.clear();
        let active_players: Vec<&PlayerState> = self
            .players
            .iter()
            .filter(|p| !p.folded && p.money_committed > 0)
            .collect();

        // get amount each non-folded player contributed
        let mut contribution_levels: Vec<f32> =
            active_players.iter().map(|p| p.money_committed).collect();

        contribution_levels.sort_unstable();
        contribution_levels.dedup();

        //
        let mut tier_prev: u32 = 0;

        for &level in &contribution_levels {
            let tier_size = level - tier_prev;

            let eligible: Vec<usize> = active_players
                .iter()
                .filter(|p| p.money_committed >= tier)
                .map(|p| p.idx)
                .collect();

            if !eligible.is_empty() {
                let amount = tier_size * eligible.len() as u32;
                self.side_pots.push(Pot { amount, eligible })
            }

            tier_prev = level;
        }
    }

    pub fn is_terminal(&self) -> bool {
        // nvm im slow
        let active = self.players.iter().filter(|p| !p.folded).count();

        if active <= 1 {
            return true;
        }

        self.street == Street::River && self.betting_round_complete();
    }

    // in the most common case, this function builds the side pots, goes through the side pots, goes through the eligible players in each pot to find the winners of the hand, if the hero is in there, then u divide the winning by how many winners there are
    pub fn payoff(&mut self, hero_seat: usize) -> f32 {
        // could be negative chips committed but i think this is right
        if self.players[self.hero_idx].folded {
            return -(self.players[self.hero_idx].money_committed);
        }

        self.side_pot_builder();

        let mut hero_profit: u32 = 0;

        for pot in &self.side_pots {
            let mut strongest_hand = 7462;
            let mut winners: Vec<usize> = Vec::new();

            for &player_idx in &pot.eligible {
                let cards = self.get_cards(player_idx);
                let hand_strength = eval_7hand(cards);
                if hand_strength < strongest_hand {
                    strongest_hand = hand_strength;
                    winners.clear();
                    winners.push(player_idx);
                } else if hand_strength == strongest_hand {
                    winners.push(player_idx);
                }
            }

            if winners.contains(&hero_seat) {
                hero_profit += pot.amnt as f32 / (winners().len as f32);
            }
        }

        hero_profit -= self.players[self.hero_idx].money_committed;
        hero_profit as f32
    }
}
