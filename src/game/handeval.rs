use crate::structure::table;

pub fn eval_7hand(cards: [i32; 7]) -> i32 {
    let hr = table::get_table();
    let mut p = hr[53 + cards[0] as usize] as usize;
    p = hr[p + cards[1] as usize] as usize;
    p = hr[p + cards[2] as usize] as usize;
    p = hr[p + cards[3] as usize] as usize;
    p = hr[p + cards[4] as usize] as usize;
    p = hr[p + cards[5] as usize] as usize;

    hr[p + cards[6] as usize]
}
