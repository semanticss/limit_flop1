// helper functions for testing and stuff
pub const RANK_NAMES: [&str; 13] = [
    "2", "3", "4", "5", "6", "7", "8", "9", "T", "J", "Q", "K", "A",
];

pub const SUIT_NAMES: [&str; 4] = ["c", "d", "h", "s"];

pub fn card_to_string(card: i32) -> String {
    let zero_based = card - 1;
    let rank = (zero_based / 4) as usize;
    let suit = (zero_based % 4) as usize;

    format!("{}{}", &RANK_NAMES[rank], &SUIT_NAMES[suit])
}
