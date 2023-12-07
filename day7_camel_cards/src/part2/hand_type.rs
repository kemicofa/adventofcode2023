use std::collections::HashMap;

use super::{card::Cards, consts::JOKER};


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    pub fn cards_to_type(cards: Cards) -> Self {
        let mut map: HashMap<u8, u8> = HashMap::new();
        let mut jokers_count = 0;

        for card in cards {
            if card == JOKER {
                jokers_count += 1;
                continue;
            }

            if let Some(count) = map.get_mut(&card) {
                *count += 1;
            } else {
                map.insert(card, 1);
            }
        }

        let mut four_of_a_kind = false;
        let mut three_of_a_kind = false;
        let mut two_of_a_kind_count = 0;

        for (_, count) in map.iter() {
            match count {
                5 => return Self::FiveOfAKind,
                4 => {
                    four_of_a_kind = true;
                    break;
                }
                3 => three_of_a_kind = true,
                2 => two_of_a_kind_count += 1,
                1 => continue,
                _ => panic!("should never happen"),
            }
        }

        if jokers_count > 0 {
            match jokers_count {
                5 => return Self::FiveOfAKind,
                4 => return Self::FiveOfAKind,
                3 => {
                    return if two_of_a_kind_count == 1 {
                        Self::FiveOfAKind
                    } else {
                        Self::FourOfAKind
                    }
                }
                2 => {
                    return if three_of_a_kind {
                        Self::FiveOfAKind
                    } else if two_of_a_kind_count == 1 {
                        Self::FourOfAKind
                    } else {
                        Self::ThreeOfAKind
                    }
                }
                1 => {
                    return if four_of_a_kind {
                        Self::FiveOfAKind
                    } else if three_of_a_kind {
                        Self::FourOfAKind
                    } else if two_of_a_kind_count == 2 {
                        HandType::FullHouse
                    } else if two_of_a_kind_count == 1 {
                        HandType::ThreeOfAKind
                    } else {
                        HandType::OnePair
                    }
                }
                _ => {}
            }

            panic!("Should not arrive here");
        }

        if four_of_a_kind {
            return HandType::FourOfAKind;
        }

        if three_of_a_kind {
            if two_of_a_kind_count == 1 {
                return Self::FullHouse;
            }
            return Self::ThreeOfAKind;
        }

        match two_of_a_kind_count {
            2 => Self::TwoPair,
            1 => Self::OnePair,
            _ => HandType::HighCard,
        }
    }
}
