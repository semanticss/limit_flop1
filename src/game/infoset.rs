use crate::game::state::Street;

pub struct InfoSet {
    hole: [i32; 2],
    board: [i32; 2], // or Vec idk
    street: Street,
    pot: u32,
    to_call: u32,
    raises_left: u8,
    position: u8,
}
