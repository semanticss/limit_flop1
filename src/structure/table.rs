use crate::game::handeval;
use std::fs::File;
use std::io::Read;
use std::sync::OnceLock;

static HAND_RANKS: OnceLock<Vec<i32>> = OnceLock::new();

pub fn get_table() -> &'static [i32] {
    HAND_RANKS.get_or_init(|| {
        println!("loading handranks");
        load_handranks("HandRanks.dat") // could be wrong filepath
    })
}

fn load_handranks(path: &str) -> Vec<i32> {
    let mut file = File::open(path).expect("handranks.dat not found");
    let metadata = file.metadata().expect("no metadata / cant read metadata");
    let mut buffer = Vec::with_capacity(metadata.len() as usize);

    file.read_to_end(&mut buffer).expect("failed to read file");

    buffer
        .chunks_exact(4)
        .map(|chunk| i32::from_le_bytes(chunk.try_into().unwrap()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hand_eval() {
        let royal_flush = [52, 48, 44, 40, 36, 1, 5];
        let res1 = handeval::eval_7hand(royal_flush);
        assert_eq!(res1 >> 12, 9, "should be a royal flush");

        let four_aces = [49, 50, 51, 52, 45, 46, 1];
        let res2 = handeval::eval_7hand(four_aces);
        assert_eq!(res2 >> 12, 8, "Should be Four of a Kind");

        let aces_full_of_kings = [49, 50, 51, 45, 46, 1, 5];
        let res3 = handeval::eval_7hand(aces_full_of_kings);
        assert_eq!(res3 >> 12, 7, "Should be a Full House");

        let high_card = [21, 14, 11, 8, 1, 38, 32];
        let res4 = handeval::eval_7hand(high_card);
        assert_eq!(res4 >> 12, 1, "Should be High Card");
    }
}
