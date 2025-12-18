use crate::deep_cfr::buffers::{AdvantageRecord, DeepCfrBuffers, StrategyRecord};
use crate::game::node_states::NodeType;
use crate::game::state::{Action, GameState, Street};
use crate::onnx_model::RustOnnxModel;
use std::collections::HashMap;

impl GameState {
    pub fn advance_state(&mut self) {
        if self.is_terminal() {
            return;
        }

        if self.betting_round_complete() {
            self.apply_street();
        }
    }

    pub fn node_type(&self) -> NodeType {
        if self.is_terminal {
            return NodeType::Terminal;
        } else if self.betting_round_complete() && self.street != Street::River {
            // 𝒮 = P also needs to be a chance node
            return NodeType::Chance;
        }
        NodeType::Player(self.curr_idx);
    }

    pub fn sample(&self) -> GameState {
        let mut next_state = self.clone();

        next_state.apply_street();
    }
}

// ts fucked up
fn cfr(state: &GameState) -> f32 {
    match state.node_type() {
        NodeType::Terminal => {
            return state.payoff(HERO); // hero_idx
        }
        NodeType::Chance => {
            let next_state = self.sample();
            return cfr(&next);
        }
        NodeType::Player(p) => {
            let legal_actions = state.legal_actions();
            todo!("need policy and information set stuff");
        }
    }
}
