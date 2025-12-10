use crate::game::state::{Action, GameState};
use crate::deep_cfr::buffers::{AdvantageRecord, StrategyRecord, DeepCfrBuffers};
use crate::onnx_model::RustOnnxModel;
use std::collections::HashMap;

pub fn cfr_traversal(gs: &GameState, player: usize, adv_net: &RustOnnxModel, pol_net: &RustOnnxModel, buffers: &mut DeepCfrBuffers) -> f32 {

    if gs.terminal {
        
    }

    if state.is_chance_node {
        
    }


};