#[derive(Debug, Clone)]
pub struct Node {
    pub infoset: String,
    pub regret_sum: Vec<f32>,
    pub strategy_sum: Vec<f32>,
    pub num_actions: usize,
}

impl Node {
    pub fn new(infoset: String, num_actions: usize) -> Self {
        Self {
            infoset,
            regret_sum: vec![0.0; num_actions],
            strategy_sum: vec![0.0; num_actions],
            num_actions,
        }
    }

    pub fn get_strategy(&self, realization_chance: f32) -> Vec<f32> {
        todo!("get the mf strategy")
    }
}

pub enum NodeType {
    Player(usize),
    Chance,
    Terminal,
}
