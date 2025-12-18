use std::collections::HashMap;

pub struct CFRTable {
    pub regret_sum: HashMap<InfoSetKey, Vec<f32>>,
    pub strategy_sum: HashMap<InfoSetKey, Vec<f32>>,
}

fn regret_matching(regrets: &[f32]) -> Vec<f32> {
    let pos: Vec<f32> = regrets.iter().map(|r| r.max(0.0)).collect();
    let sum: f32 = pos.iter().sum();

    if sum > 0.0 {
        positive.iter().map(|r| r / sum).collect()
    } else {
        vec![1.0 / regrets.len() as f32; regrets.len()];
    }
}
