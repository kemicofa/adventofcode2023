mod card;
mod consts;
mod hand;
mod hand_type;

/*
    --- Day 7: Camel Cards ---
    --- Part Two ---
    To make things a little more interesting, the Elf introduces one additional rule. Now, J cards are jokers - wildcards that can act like whatever card would make the hand the strongest type possible.

    To balance this, J cards are now the weakest individual cards, weaker even than 2. The other cards stay in the same order: A, K, Q, T, 9, 8, 7, 6, 5, 4, 3, 2, J.

    J cards can pretend to be whatever card is best for the purpose of determining hand type; for example, QJJQ2 is now considered four of a kind. However, for the purpose of breaking ties between two hands of the same type, J is always treated as J, not the card it's pretending to be: JKKK2 is weaker than QQQQ2 because J is weaker than Q.

    Now, the above example goes very differently:

    32T3K 765
    T55J5 684
    KK677 28
    KTJJT 220
    QQQJA 483
    32T3K is still the only one pair; it doesn't contain any jokers, so its strength doesn't increase.
    KK677 is now the only two pair, making it the second-weakest hand.
    T55J5, KTJJT, and QQQJA are now all four of a kind! T55J5 gets rank 3, QQQJA gets rank 4, and KTJJT gets rank 5.
    With the new joker rule, the total winnings in this example are 5905.

    Using the new joker rule, find the rank of every hand in your set. What are the new total winnings?
*/

use utils::split_and_clean_input_into_lines;

use self::{consts::JOKER, hand::Hand};

fn map_card_type_to_value(c: char) -> u8 {
    match c {
        'A' => 0xD,
        'K' => 0xC,
        'Q' => 0xB,
        'J' => JOKER, // now the weakest card
        'T' => 0xA,
        _ => (c.to_digit(16).unwrap()).try_into().unwrap(),
    }
}

fn parse_str_to_hand(input: &str) -> Hand {
    let hand_of_numbers = input
        .chars()
        .map(|c| map_card_type_to_value(c))
        .collect::<Vec<u8>>();
    Hand::new(hand_of_numbers.try_into().unwrap())
}

fn parse_input(input: &str) -> Vec<(Hand, u32)> {
    split_and_clean_input_into_lines(input)
        .iter()
        .map(|line| {
            let (raw_hand, raw_bid) = line.split_once(' ').unwrap();
            let bid = raw_bid.parse::<u32>().unwrap();

            (parse_str_to_hand(raw_hand), bid)
        })
        .collect::<Vec<(Hand, u32)>>()
}

pub fn solve(input: &str) -> u32 {
    let mut hands_and_bids = parse_input(input);
    hands_and_bids.sort_by(|(hand_a, _), (hand_b, _)| hand_a.cmp(hand_b));

    let mut result: u32 = 0;
    for i in 0..hands_and_bids.len() {
        result += (i as u32 + 1) * hands_and_bids[i].1;
    }
    return result;
}

#[cfg(test)]
mod tests {
    use crate::consts::INPUT;

    use super::*;

    #[test]
    fn it_works() {
        let input = r#"
            32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483
        "#;

        assert_eq!(solve(input), 5905);
    }

    #[test]
    fn it_works_with_puzzle() {
        assert_eq!(solve(INPUT), 251224870);
    }
}
