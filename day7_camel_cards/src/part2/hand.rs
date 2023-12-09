use std::cmp::Ordering;

use super::{card::Cards, hand_type::HandType};

#[derive(Debug)]
pub struct Hand {
    cards: Cards,
    t: HandType,
    hash: String,
}

impl Hand {
    fn hash(cards: Cards) -> String {
        let mut cards_clone = cards.clone();
        cards_clone.sort();
        cards_clone.map(|val| val.to_string()).join(":")
    }

    pub fn new(cards: Cards) -> Self {
        let t = HandType::cards_to_type(cards);
        let hash = Self::hash(cards);
        Self { cards, t, hash }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, _: &Self) -> Option<std::cmp::Ordering> {
        panic!("We should not need to partial compare here");
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let ordering = self.t.cmp(&other.t);

        if ordering != Ordering::Equal {
            return ordering;
        }

        for i in 0..self.cards.len() {
            match self.cards[i].cmp(&other.cards[i]) {
                Ordering::Equal => continue,
                Ordering::Less => return Ordering::Less,
                Ordering::Greater => return Ordering::Greater,
            }
        }

        panic!("Should have found proper ordering!");
    }
}
