use serde::{Deserialize, Serialize};
// use std::cmp;

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
    fn test_side_pot_integrity() {
        let mut state = test_state();

        state.players.push(PlayerState {
            idx: 2,
            hole: [5, 6],
            money_behind: 1000,
            money_committed_curr_round: 0,
            total_committed: 0,
            folded: false,
        });

        state.players[0].money_behind = 0;
        state.players[0].total_committed = 10;
        state.players[1].money_behind = 0;
        state.players[1].total_committed = 50;
        state.players[2].total_committed = 50;

        state.pot = 110;

        state.side_pot_builder();

        assert_eq!(state.side_pots.len(), 2);
        assert_eq!(state.side_pots[0].amnt, 30);
        assert_eq!(state.side_pots[1].amnt, 80);
    }

    #[test]
    fn test_dead_money_retention() {
        let mut state = test_state();

        state.players[0].total_committed = 10;
        state.players[1].total_committed = 10;

        state.players[0].folded = true;

        state.side_pot_builder();

        assert_eq!(state.side_pots[0].amnt, 20);
    }

    #[test]
    fn test_split_pot_math() {
        let mut state = test_state();
        state.pot = 100;
        state.players[0].total_committed = 50;
        state.players[1].total_committed = 50;
        state.board = vec![10, 11, 12, 13, 14];
        state.players[0].hole = [1, 2];
        state.players[1].hole = [1, 2];

        state.hero_idx = 0;
        let payoff = state.payoff();
        assert_eq!(payoff, 0);
    }

    #[test]
    fn fold_ended() {
        let mut state = test_state();

        state = state.apply_action(&Action::Raise);
        state = state.apply_action(&Action::Fold);

        assert!(state.is_terminal());
    }
    #[test]
    fn full_hand() {
        let mut state = test_state();

        state.deck = vec![20, 21, 22, 17, 10, 11, 12];

        assert_eq!(state.street, Street::Preflop);

        state = state.apply_action(&Action::Check);
        state = state.apply_action(&Action::Check);

        assert!(state.betting_round_complete());
        state.apply_street();

        assert_eq!(state.street, Street::Flop);
        // println!("{}", state.board.len());
        assert_eq!(state.board.len(), 3);

        state = state.apply_action(&Action::Check);
        state = state.apply_action(&Action::Check);

        state.apply_street();

        println!("{:?}", state.street);
        assert_eq!(state.street, Street::Turn);
        assert_eq!(state.board.len(), 4);

        state = state.apply_action(&Action::Check);
        state = state.apply_action(&Action::Check);

        state.apply_street();

        println!("{:?}", state.street);
        assert_eq!(state.street, Street::River);
        assert_eq!(state.board.len(), 5);

        state = state.apply_action(&Action::Check);
        state = state.apply_action(&Action::Check);

        assert!(state.is_terminal());

        let payoff = state.payoff();

        assert_eq!(state.pot, 0);
        assert_eq!(payoff, 0);
    }

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
}
