use rand::seq::SliceRandom;
use rand::thread_rng;

pub const CLUB: i32 = 0x8000;
pub const DIAMOND: i32 = 0x4000;
pub const HEART: i32 = 0x2000;
pub const SPADE: i32 = 0x1000;

// 2,3,...,king,ace
pub const RANKS: [i32; 13] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];

pub const SUITS: [i32; 4] = [CLUB, DIAMOND, HEART, SPADE];

pub fn make_deck() -> Vec<i32> {
    let mut deck = Vec::with_capacity(52);

    for &suit in &SUITS {
        for &rank in &RANKS {
            let card = suit | rank;
            deck.push(card);
        }
    }

    deck
}

pub fn shuffle_deck(deck: &mut [i32]) {
    deck.shuffle(&mut thread_rng());
}
