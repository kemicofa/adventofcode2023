/*
    --- Day 4: Scratchcards ---
    --- Part Two ---
    Just as you're about to report your findings to the Elf, one of you realizes that the rules have actually been printed on the back of every card this whole time.

    There's no such thing as "points". Instead, scratchcards only cause you to win more scratchcards equal to the number of winning numbers you have.

    Specifically, you win copies of the scratchcards below the winning card equal to the number of matches. So, if card 10 were to have 5 matching numbers, you would win one copy each of cards 11, 12, 13, 14, and 15.

    Copies of scratchcards are scored like normal scratchcards and have the same card number as the card they copied. So, if you win a copy of card 10 and it has 5 matching numbers, it would then win a copy of the same cards that the original card 10 won: cards 11, 12, 13, 14, and 15. This process repeats until none of the copies cause you to win any more cards. (Cards will never make you copy a card past the end of the table.)

    This time, the above example goes differently:

    Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    Card 1 has four matching numbers, so you win one copy each of the next four cards: cards 2, 3, 4, and 5.
    Your original card 2 has two matching numbers, so you win one copy each of cards 3 and 4.
    Your copy of card 2 also wins one copy each of cards 3 and 4.
    Your four instances of card 3 (one original and three copies) have two matching numbers, so you win four copies each of cards 4 and 5.
    Your eight instances of card 4 (one original and seven copies) have one matching number, so you win eight copies of card 5.
    Your fourteen instances of card 5 (one original and thirteen copies) have no matching numbers and win no more cards.
    Your one instance of card 6 (one original) has no matching numbers and wins no more cards.
    Once all of the originals and copies have been processed, you end up with 1 instance of card 1, 2 instances of card 2, 4 instances of card 3, 8 instances of card 4, 14 instances of card 5, and 1 instance of card 6. In total, this example pile of scratchcards causes you to ultimately have 30 scratchcards!

    Process all of the original and copied scratchcards until no more scratchcards are won. Including the original set of scratchcards, how many total scratchcards do you end up with?
*/

use std::collections::HashMap;

use utils::split_and_clean_input_into_lines;

fn get_numbers_from_str(input: &str) -> Vec<u32> {
    input
        .trim()
        .split_whitespace()
        .map(|val| val.trim().parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
}

fn get_winning_and_selected_numbers_from_str(input: &str) -> (u32, Vec<u32>, Vec<u32>) {
    let (card_metadata, card_data) = input.split_once(':').unwrap();
    let card_id = card_metadata
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<u32>()
        .unwrap();
    let (winning_numbers_str, selected_numbers_str) = card_data.split_once('|').unwrap();

    let winning_numbers = get_numbers_from_str(winning_numbers_str);
    let selected_numbers = get_numbers_from_str(selected_numbers_str);

    (card_id, winning_numbers, selected_numbers)
}

fn parse_input(input: &str) -> Vec<(u32, Vec<u32>, Vec<u32>)> {
    split_and_clean_input_into_lines(input)
        .iter()
        .map(|card| {
            let (card_id, winning_numbers, selected_numbers) =
                get_winning_and_selected_numbers_from_str(card);
            return (card_id, winning_numbers, selected_numbers);
        })
        .collect::<Vec<(u32, Vec<u32>, Vec<u32>)>>()
}

pub fn scratch_cards(input: &str) -> u32 {
    let cards = parse_input(input);
    let mut instances_map: HashMap<u32, u32> = HashMap::new();
    cards
        .iter()
        .map(|(card_id, winning_numbers, selected_numbers)| {
            let winning_numbers_chosen_count = winning_numbers
                .iter()
                .filter(|val| selected_numbers.contains(val))
                .count() as u32;

            let instances = match instances_map.get(card_id) {
                Some(instances) => *instances,
                None => 1,
            };

            for i in 1..(winning_numbers_chosen_count + 1) {
                let key = card_id + i;
                if key > cards.len() as u32 {
                    break;
                }
                if let Some(next_instances) = instances_map.get_mut(&key) {
                    *next_instances += instances;
                } else {
                    instances_map.insert(key, 2 + (instances - 1));
                }
            }

            instances
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::consts::INPUT;

    use super::*;

    #[test]
    fn it_works() {
        let input = r#"
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        "#;
        assert_eq!(scratch_cards(input), 30);
    }

    #[test]
    fn it_works_with_puzzle() {
        assert_eq!(scratch_cards(INPUT), 5554894);
    }
}
