use serde::{Deserialize, Serialize};
use std::cmp;

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum Street {
    Preflop,
    Flop,
    Turn,
    River,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum Action {
    Fold,
    Call,
    Raise,
    Check,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Pot {
    pub amnt: u32,
    pub eligible: Vec<u32>,
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

fn test_state() -> GameState {
    GameState {
        players: vec![
            PlayerState {
                idx: 0,
                hole: [1, 2],
                money_behind: 10,
                money_committed_curr_round: 0,
                total_committed: 0,
                folded: false,
            },
            PlayerState {
                idx: 1,
                hole: [3, 4],
                money_behind: 10,
                money_committed_curr_round: 0,
                total_committed: 0,
                folded: false,
            },
        ],
        curr_idx: 0,
        street: Street::Preflop,
        current_bet: 0,
        raises_left: 3,
        pot: 0,
        side_pots: vec![],
        deck: vec![10, 11, 12, 13, 14],
        board: vec![],
        hero_idx: 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_legal_actions_no_bet() {
        let state = test_state();

        let actions = state.legal_actions();

        assert!(actions.contains(&Action::Check));
        assert!(actions.contains(&Action::Raise));
        assert!(!actions.contains(&Action::Call));
    }

    #[test]
    fn test_legal_actions_facing_bet() {
        let mut state = test_state();
        state.current_bet = 2;

        let acts = state.legal_actions();

        assert!(acts.contains(&Action::Fold));
        assert!(acts.contains(&Action::Call));
        assert!(!acts.contains(&Action::Check));
    }

    #[test]
    fn test_no_actions_when_all_in() {
        let mut state = test_state();
        state.players[0].money_behind = 0;

        assert!(state.legal_actions().is_empty());
    }

    #[test]
    fn test_call_trns() {
        let mut state = test_state();

        state.current_bet = 2;

        let next_state = state.apply_action(&Action::Call);

        let p = &next_state.players[0];

        assert_eq!(p.money_behind, 8);
        assert_eq!(p.money_committed_curr_round, 2);
        assert_eq!(next_state.pot, 2);
    }

    #[test]
    fn test_raise_trns() {
        let mut state = test_state();
    }
}
