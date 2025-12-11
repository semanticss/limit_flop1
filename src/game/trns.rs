extern "C" {
    pub fn eval_7hand_safe(
        c0: i32, c1: i32, c2: i32, c3: i32, c4: i32, c5: i32, c6: i32
    ) -> i32;
}


pub fn legal_actions(gs: &GameState) -> Vec<Action> {

    

}; // what actions can actually be taken

pub fn apply_action(gs: &GameState, a: &Action) -> GameState {

};

pub fn is_terminal(gs: &GameState) -> bool {

};

pub fn payoff(gs: &GameState, player: usize) -> f32 {
    eval_7_hand_safe()
};