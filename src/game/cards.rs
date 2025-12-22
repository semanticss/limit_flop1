// use rand::seq::SliceRandom;
// use rand::thread_rng;

// pub const CLUB: i32 = 0;
// pub const DIAMOND: i32 = 1;
// pub const HEART: i32 = 2;
// pub const SPADE: i32 = 3;

// pub const RANKS: [i32; 13] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
// pub const SUITS: [i32; 4] = [CLUB, DIAMOND, HEART, SPADE];

// pub fn make_deck() -> Vec<i32> {
//     let mut deck = Vec::with_capacity(52);

//     for &rank in &RANKS {
//         for &suit in &SUITS {
//             let card = (rank * 4) + suit + 1;
//             deck.push(card);
//         }
//     }
//     deck
// }

// pub fn shuffle_deck(deck: &mut [i32]) {
//     deck.shuffle(&mut thread_rng());
// }

// #[cfg(test)]

// mod tests {
//     use crate::game::helpers::card_to_string;

//     use super::*;

//     #[test]
//     fn print_deck_as_i32() {
//         let deck: Vec<i32> = make_deck();

//         for card in deck {
//             println!("{}", card);
//         }
//     }

//     #[test]

//     fn print_deck_as_string() {
//         let deck: Vec<i32> = make_deck();

//         for card in deck {
//             print!("{}", card_to_string(card));
//         }
//     }
// }
