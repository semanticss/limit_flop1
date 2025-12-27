// this file determines the next gamestate given the current gamestate

use super::state::{Action, GameState, PlayerState, Pot, Street};
use crate::game::handeval;

impl GameState {
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
        // need to add bets to pot, remove from money behind for each player, append the action to the action history, and more prolly
        match a {
            Action::Call => {
                let player = &mut next_state.players[idx];
                let to_call = next_state.current_bet - player.money_committed_curr_round;
                let amount = to_call.min(player.money_behind);

                player.money_behind -= amount;
                player.money_committed_curr_round += amount;
                player.total_committed += amount;
                next_state.pot += amount;
            }
            Action::Fold => {
                let player = &mut next_state.players[idx];
                player.folded = true;
            }
            Action::Raise => {
                let raise_amount = next_state.raise_size();
                let player = &mut next_state.players[idx];
                let to_call = next_state.current_bet - player.money_committed_curr_round;
                let total = to_call + raise_amount;

                let paid = total.min(player.money_behind);

                player.money_behind -= paid;
                player.money_committed_curr_round += paid;
                player.total_committed += paid;

                next_state.current_bet += raise_amount;
                next_state.raises_left -= 1;
                next_state.pot += paid;
            }
            Action::Check => {}
        }

        next_state.move_turn();
        next_state
    }

    pub fn move_turn(&mut self) {
        let n = self.players.len();

        let players_able_to_act = self
            .players
            .iter()
            .filter(|p| !p.folded && p.money_behind > 0)
            .count();

        if players_able_to_act <= 1 {
            return;
        }

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

        match self.street {
            Street::Preflop => {
                self.draw_cards(3);
                self.street = Street::Flop;
            }
            Street::Flop => {
                self.draw_cards(1);
                self.street = Street::Turn;
            }
            Street::Turn => {
                self.draw_cards(1);
                self.street = Street::River;
            }
            Street::River => panic!("mf cmon"),
        }
    }

    pub fn get_cards(&self, p: usize) -> [i32; 7] {
        let p_hole = &self.players[p].hole;
        let board = &self.board;
        let mut result = [0i32; 7];

        result[0..2].copy_from_slice(p_hole);
        result[2..7].copy_from_slice(board);

        result
    }

    // maybe this works
    pub fn side_pot_builder(&mut self) {
        self.side_pots.clear();
        let active_players: Vec<&PlayerState> = self
            .players
            .iter()
            .filter(|p| !p.folded && p.total_committed > 0)
            .collect();

        // get amount each non-folded player contributed
        let mut contribution_levels: Vec<u32> =
            active_players.iter().map(|p| p.total_committed).collect();

        contribution_levels.sort_unstable();
        contribution_levels.dedup();

        //
        let mut tier_prev: u32 = 0;

        for &level in &contribution_levels {
            let tier_size = level - tier_prev;

            let eligible: Vec<u32> = active_players
                .iter()
                .filter(|p| p.total_committed >= level)
                .map(|p| p.idx as u32)
                .collect();

            let contributors = self
                .players
                .iter()
                .filter(|p| p.total_committed >= level)
                .count();

            if !eligible.is_empty() {
                let amnt = tier_size * contributors as u32; // include money from people who folded as well
                self.side_pots.push(Pot { amnt, eligible })
            }

            tier_prev = level;
        }
    }

    pub fn is_terminal(&self) -> bool {
        let active = self.players.iter().filter(|p| !p.folded).count();

        if active <= 1 {
            return true;
        }

        if self.street == Street::River && self.betting_round_complete() {
            return true;
        }

        let players_with_chips = self
            .players
            .iter()
            .filter(|p| !p.folded && p.money_behind > 0)
            .count();

        if players_with_chips < 2 && self.betting_round_complete() {
            return true;
        }

        false
    }

    // in the most common case, this function builds the side pots, goes through the side pots, goes through the eligible players in each pot to find the winners of the hand, if the hero is in there, then u divide the winning by how many winners there are
    // ts messed up rn
    pub fn payoff(&mut self) -> i32 {
        // also need to make sure the board has 5 cards
        while self.board.len() < 5 {
            if let Some(card) = self.deck.pop() {
                self.board.push(card);
            } else {
                panic!("mf there are no more cards somehow");
            }
        }

        assert!(self.board.len() == 5);

        if self.players[self.hero_idx].folded {
            return (self.players[self.hero_idx].total_committed as i32) * -1;
        }

        self.side_pot_builder();

        let mut hero_profit: u32 = 0;

        for pot in &self.side_pots {
            let mut strongest_hand = i32::MAX;
            let mut winners: Vec<usize> = Vec::new();

            for &player_idx in &pot.eligible {
                let cards: [i32; 7] = self.get_cards(player_idx as usize); //by this point the board should have just run out, or just take the next cards from the deck
                let hand_strength = handeval::eval_7hand(cards);
                if hand_strength < strongest_hand {
                    strongest_hand = hand_strength;
                    winners.clear();
                    winners.push(player_idx as usize);
                } else if hand_strength == strongest_hand {
                    winners.push(player_idx as usize);
                }
            }

            if winners.contains(&self.hero_idx) {
                hero_profit += pot.amnt / (winners.len() as u32); //floats maybe
            }
        }

        hero_profit -= self.players[self.hero_idx].total_committed;
        hero_profit as i32
    }
}
