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
    input.trim()
        .split(' ')
        .filter(|val| !val.is_empty())
        .map(|val| val.trim().parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
}

fn get_winning_and_selected_numbers_from_str(input: &str) -> (u32, Vec<u32>, Vec<u32>) {
    let (card_metadata, card_data) = input.split_once(':').unwrap();
    let card_id = card_metadata.split_whitespace().last().unwrap().parse::<u32>().unwrap();
    let (winning_numbers_str, selected_numbers_str) = 
    card_data
    .split_once('|')
    .unwrap();

    let winning_numbers = get_numbers_from_str(winning_numbers_str);
    let selected_numbers = get_numbers_from_str(selected_numbers_str);

    (card_id, winning_numbers, selected_numbers)
}

fn parse_input(input: &str) -> Vec<(u32, Vec<u32>, Vec<u32>)> {
    split_and_clean_input_into_lines(input)
        .iter()
        .map(|card| {
            let (card_id, winning_numbers, selected_numbers) = get_winning_and_selected_numbers_from_str(card);
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
                None => 1
            };

            for i in 1..(winning_numbers_chosen_count+1) {
                let key = card_id + i;
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
        let input = r#"
            Card   1: 24 12 26 39 19 98 74 16 82 77 | 80 11 51  1 74 60 77 68 42 35 39 78 21 12 29 19 25 98 65 91 33 17 59 24 31
            Card   2: 33 21 96 64  9 38 65 74 16 91 | 14 51 85  1 64 29 74 18 15 38 13 96 16 88 84 21 95 49  9 27 33 63 65 91 90
            Card   3: 31 97 18 93 71 54 24 12 19 87 | 59 96 72 94  4 51 74 84 47 10 57 89 65 37 39 16 31 91 26 85 44 30 24 40  2
            Card   4: 77 20 66 48 23 32 76  9 75 89 | 31 57 89 51  8 34  9 59 39 73  4 32 18 77 94 76 29 23 41 27 66 20 40 48 75
            Card   5: 25 40 65 27 41  7  1 80 26 47 |  7 25 40  1 16 67 42 34 82 19 58  9 91  6 47 80 36 46 77 27 72 41 26 33 81
            Card   6: 92 39 18 64  7 71 48 29  3 38 | 55 29 73 31 15 75 13 71 94 48 78 23 54  7 10 86 34 82 91 85 67 14 57 64  3
            Card   7: 67 43 34 90 14 32 38 49 11 75 | 15 11 38 30 32 29 59 73 72 42  1  4 55 71 57 64 19 90 24 78 31 67 49 98 43
            Card   8: 70 64 82  4 16  6 19 13  9 29 | 21 93 37 69 24 62 60  3 90 83  8 66 20 34 55 22  6 84 99 50 33 26 65 98 86
            Card   9: 31 80 67 81 28 90 23  4 48 86 | 90 80 32  8 15 13 81 63 46 45 50 47 17 96 48 28 57 98 88 86  1 34  4 87  5
            Card  10: 40 46 22 70 59 97 32 20  4 65 | 22 25 59 88 30 82 39 91  4 72  9 42 86 65 96 93  5 84 67 40 57 20 36  8 80
            Card  11: 53 10  4 91 42 12 32 86 38 60 |  3 60 20 65 85 94 58 38 10 76 44 81 51  6 99 19 31 43 84 95 59 36 86 82 11
            Card  12: 73 87 16 92 67 13 28 82 97 86 | 60 45 83  4 90 43 54 55 93 62  7 94 65 72 32 25 23 38 88 61 87 76 35 20 18
            Card  13: 92 13 96 89 25 81 80 72 76 48 | 92 32 31 23 61 74 13 95 71 78 17  1 22 86 62 55 60 41 96 11 77 93 63 99 76
            Card  14:  8 59 91 73 10 61 65 34 29 81 |  9 32 87 78 26 16 90 49 74 61 56 11 57 93 77 62 75 46 36 59 85  3 19 34 28
            Card  15: 87 53 99 88 20 39 28 61 38 68 | 60 16  1  5 10 53 45 56 89 42 80 21  2 37 98 99 74 71 72 59 84 41 87 95 67
            Card  16:  2 19 58 83 91 76  9 63 71 77 | 37  3 59 79 72 53 70  9 43 36 97 98  6 85 90 16 55 11 56  1  8 49 52 15 22
            Card  17: 29 83 12 76 32 82 40 91 84 97 | 21 79  3 31  1 36 85 17 59 30 87 46 27 63 75 56 72 67 11 42 84 62 53 68 38
            Card  18: 39 63 75 71 70 41 49 15 82 78 | 44 74  3 24 35 41 12 47 34 54 91 45 67 57 62 60 95  8 99 19  9 79 80  4 92
            Card  19: 82 33 76 22 93 10 49 46  9 63 | 70 79 80 44 92 15 90 31 75 17 54 81 12 21 71 61 45 60 72 87 91  8 65 83 47
            Card  20: 60 22 86  1 94 25 72 51 73 57 | 28 66 60 25 16  1 54 36 94  7 73 29 57 55 61 22 51 44 39 69 86 23 72  9 71
            Card  21:  6 30 81 11 69 96 45 76  4 78 | 45 96 51 87 65 95 78 82 76 24 32 28 11 50  4 85 74 81 30 33 69  6 34 79 41
            Card  22: 91 88 99  7 98  1 34 81 36 44 | 69 98 15 50 22  7 27 56 52 97 66 88 89  5 31 34 99 11 91 36  1 33 44 81 55
            Card  23: 94 78 99 13 27 56 81  1 62 84 | 84 33 13 70 81 99 63 56 34 72 64 27 93 87 78  1 29  4 62 44 19 94 37 86 18
            Card  24:  8 81 12 30 78 84 33 39 74 20 | 39 51  4 93 62 74 67 75  8 12 28 33 30 99 84 97 95  5 14  1 20 78 81 61 60
            Card  25: 79 74 91 69  3 96 42 98 87 82 | 53 92 74 17 25 81 96 61 56 15 59 27 82 97  2 71 68 95 83 13 10 47 41 75 87
            Card  26: 20 87 17 34 69 97 36 77 96 66 | 66 10 11 96 83 17 13 57  6 73 88 72  2 39 89 48 79 33 22 19 56 14  9 85 26
            Card  27: 18 10 33 21 67 25  7 86 12 72 | 91 53  6 30 85 90 61 20 40 31 23 83 78 72 68 81 63 58 97 42 16 50 79 37 66
            Card  28: 96 36 98 66 37  8 78 41 55  7 | 77 70 42 37 74  8 96 76 63 64 93 98 78 30 66  1  9 55  7 41 90 29  4 36 22
            Card  29: 51 44 97 56 70 67 82 47 61 60 | 90 94 15 67 75  4 52 21 87 33 77 97 76 20 13 35 45 39 78 82 48 70 89 44 19
            Card  30: 57 24 50 79 36 12 45 68 41 14 | 66 90 12 44 29 79 91 73 57 69 68 24 36 56 33 14  1 83 41 45  9 40 60 20 50
            Card  31: 61 53 23 10 85 72 62 52 26 57 | 37 50 73  7 77 26 63 23 83 44 29 96 48 91 27 53 34 33 61 72 20 42 10 22 25
            Card  32: 76 41 35 73 45 66 54  3 69 12 |  8  2 42 85 28 93 58 89 76 73 84 17  3 56 12 52 68 62 66 54 69 45 41 82 35
            Card  33: 32 29 97 52 45 95 92 18 42 47 | 65 97 46 95 96 39 37 16 52 69 45 92 50 17 29 18  1 47 42 77 56 81 71  7 32
            Card  34: 26 96 10 87 15 16 84 99 75  4 | 29 53 75 48 91  8 89 30  5 22 21 87 13 36 49 16 25 94  2 41 18 69 70 68 67
            Card  35: 57 89  3 81  5 35 32 29 91 65 | 50 68 11 61 39 91 36 31  5 89 12  9 60 13 78 75  4 51 21 45 54 14 73  1 62
            Card  36: 80 46 17 70 42  1 63 87 92 98 | 37 18 64 47 46 94 35  2 10 86 91 19  1 80  4 71  5 78 81 52 31 17 92 40 68
            Card  37: 22 15 30  2 82 24 76 84 85  5 | 30 85 22 24 77 12 49 84 93  1 82 17 27 47  4 10  2 86  6 63 31 33 60 55 57
            Card  38: 63 51 26 56 99 22 32 59  7  3 | 73 11 13  5 37 18 84 35 98 42  4 55 15 58 60 78 22 41  3 89 33 74 93 56 46
            Card  39:  5 96 13 31 59 88 87 55 27 11 | 32 46 56 28 43 45 69 27  5 39 91 79 98 63 35 21 29 38 30 82 71 93 49 80 70
            Card  40: 62 47 56 14 20 91 86  5 12 33 | 88 65 66 10 17 30 51 63 15 83  3 55 27 28 64 35  7 99 76 68 39 23 48 74 41
            Card  41: 48 67 38 82 73 65  1 11 78 53 | 50 76 31 66 11 98 80 29 93  6 82 15 86 74 78 28 60 63 12 59 71 24 68 36 56
            Card  42: 91 27 55 36 14 71 45 10 42  5 | 87 44 80 24 43 84 95 99 53 76 33 23 96 20 62 51 72 49 79 78 39 94 74 61 89
            Card  43: 81 61 13 78 28 75 58 67 54 11 | 96 51 53 73 76 24 82 85 42 25 93 57 35 91  9 64 99 11 88 79 39 44 74 65 29
            Card  44: 82 69 37 22 80 81 55 35 27 19 | 12 63 85 74 39 33 95 75 93 73 44 40  2 76 14 94 42  8 17  9  6 84 34 23  4
            Card  45: 20 86 52 74 80 69 53 33 98 41 | 69 51 18 74 98 90 42 97  7 24 41 62 30 58 95 52  5 31 33 53 20 87 38 25 80
            Card  46: 67 72 36  1 54 24 86 61 85 52 | 19 22 86 64 41 88 31 72 17  1 21 85 67 29 82 54 36 61 75 56 70 53 95 52 24
            Card  47: 62 23 80 46 56 10 52 89 50  5 | 37 55 43 79 98 60 30 44 10 97 38 90 88 83 36 35 61 28 76 21 65 87 40 84 91
            Card  48: 11 26 39 71  9 51 86 65 47 13 | 54 23 60 47 26  9 71 13 85 92 39 19 79 14 73 74 45 22 36 15 65 11 63 10 33
            Card  49: 25 62 23 78 98 83 33  7 37 10 | 85 81 71 67 83 98 60 35 10 79 33  7 52 89 62 93 87 23 37 11 25 45 78 95 82
            Card  50: 98 18 87 92 43 45 67 12 46 69 | 38 74 44 31 89  9 62 53 34 43 41 96 11  4 47  8  5 42  3 95 71 28 73 97 33
            Card  51: 72 33 31 38  5 67 25  1  4 79 |  9 77 58 97 48 30 85 91 79 70 49 10 98 76 47 90 89 88 25  5 51 26 99 32  7
            Card  52: 92 18 67 66 45 23 10 83 78 57 | 70 17 97  6  9 95 37  4 69 40 93  2 11 12 61  5 30 42 74  7 87 85 20 52 75
            Card  53:  4 59  6 44 12 60 14 86 93 11 | 14 44 74  7 87  6 82 84 55 48 17 37 18 60 23 59 97 62 31  4 93 12 11 13 58
            Card  54:  2 64 63 60 87 72 33 46 88 92 | 96  5 46 94 92  7 63 93 17 13 25 60 73 55  9 98 20 49 29 57 61 64 48 52 19
            Card  55: 29 93  5 53 72 82 12 67 64 50 | 52 33 77 16 45 31 56 28 18 72 79  4 73 41 59 35  2 38 68 98 87 84 94 66 12
            Card  56: 26 18 58 78 53 22 94 23 76 93 |  4 36 64 38 26 72 21 53 68 91 17 18 24 78 33 16  5  9 79  2 98 52 99 40 76
            Card  57: 85 36 66 93 50 62 90 98  4 31 | 23  6 31 76 79 80 15 38 42 81 85 14 56 88 90 65 50 54 62 78 27 77 35 10 70
            Card  58: 80 61 53 48 94 11 87 89 72 43 |  3 55 54 77 57 95 31  7 12 52 92 59 25 23 80 14 45 85 81 94 30 24 67 53 16
            Card  59: 78 54 15 18 17 28 89 12 88 81 | 55 51 97 22 32 72 88 73 11 28  8 65 14 20  6 79 41 87 29  2 40 74 17 99 52
            Card  60: 64 74 67 76 78 43  6 14 79 96 | 82 23 87 37 51 64 89 15  1 56 86 66 10  9  8 53  2 11 24 88 76 35 54 69 68
            Card  61: 53 29 18 65 93 62 75 20 15 35 |  9 77 14 60 38 36 98 24 47 28  6 95 27 12 10 94 61 71 68 22 72 37 96  4 81
            Card  62: 43 85 70 78 16  8  3 18 26 67 | 63 86 95 90 15 13  4 31 82 74 34 76 96 14 49 94 55 80 39 84 79 68 54 81  1
            Card  63:  9 54 71 62 38 75  3 12 46  8 |  8 12 38 69  9  3 79 16  2  6 54 25 75 71 21 60 29 13 30 50 62 98 46 15 47
            Card  64: 98 90 48 49 86 78 45 60 92 81 | 84 67 21 27 37 76 99  1 74 20 23 73 58 10 60 17 57 26 29 86 11 64 31 54 66
            Card  65: 45 21 33 32 64 38 90 47 15 35 | 64 82 38 89 21 32 90 33 16 63 54 65 39 15 37 42 96  7 40 26 50 45 30 85 22
            Card  66: 95 88 40 57 32 31 89 23 68 17 | 64 94 68  4 55 58  8  6  1 17 88 89 32 95 13 78 93 45 23 40 76 31 96 15 57
            Card  67: 15 60  6 79 89 46 67 26 70 29 | 29 79 36 20  1 14 39  2 89 75 67 33 19 12 70 26  9 58  6 35 60 17 15 10 68
            Card  68: 76 32 99 77 16  6 17 46 91 42 | 81 42 82 80 91 52 70 90 26 15 45 35 22 27 84 67 16 24 32 76 54 77 75 51 31
            Card  69: 34 68 42 21  5 91 47 95 59 63 | 35 59  4 20 47 37 24 95 79 91 49 73 68  6  5 46 63 99 58 21 88 67 94 44 80
            Card  70: 69 11 94 64 21 65 31 89 55 28 |  5 64 99 53 34  4 51 88 10 68 74 81  1 23 29 58 93 94 43 32 70 76  7 77 21
            Card  71: 29 52 43 97 41 83 86 99 49 45 | 86 29 43 89 42 53 82 17 94 62 52 99 15 27 40 49 78 74 77 28  9 10 88 97 68
            Card  72: 48 11 62 26  1 89 61  3 23  6 | 11 54 93 60 43  3 24 77 48 61 32  5 79 91 21 73 62 96 87  6 89 59 26  1 15
            Card  73: 65 45 90 13 71 50 48 97 54 77 | 97 16 45 87 68 32 12 15 21  7 33 64 37 88 67 10 60 26 49 65 20 11 29 44 52
            Card  74: 83 54 74 80  4 60  8 68 35 18 | 71 81 31 11  5 60 77 90 76 58 64 32 82 21 97  2 53  9 10 85 75 18 55 86 25
            Card  75: 13 19 42 14 17 74 28 34 56 93 | 76 83 33 82 28 86 93 29 39 88 98 14 49 89 74 62 13 23 43 17 42 56 45 41 35
            Card  76: 51 79 58 45 42 65 29 96 36 13 | 82 99 23 66 29 69 92 95 64 13 40 45 76 86 74 44 81 73 10  4 30 79 68  1 12
            Card  77: 98 12 89 28 29 87 92  4 13 85 | 24 47  4 76  9 95 29 80 87 16 53 85  7 42 86 82 59 37 28 35 55 40 50 10 58
            Card  78: 98 59 89 56 30 15 51 64  4 95 | 74 92 70 91 45 94 95 58 79 88 87 63 40 54 50 77 31 30 41 17 43 82  4 44 13
            Card  79: 37 35 51 39 10 79 32 56 40 42 | 53 28 38 36 76 83  2 42 35 90  6 84 49 27 47 64 75 39 89 52  3 17 40 87 93
            Card  80: 80 42 36 87  9 50 40 96 27 16 |  6 65  3 69 85 63 30 18  8 77 11 94 53 55  5 20 66 33 32 13 49 89 19 83 70
            Card  81: 20 51 80 10 71 43 94 87  7 55 | 44  1 59 33 27 57 28 17 77 46 11 72 73 54 65 37 25 74 60 41 98 84 22 58 79
            Card  82: 13 87 41 55 28 79  7  2 22 37 |  8 20 91 72 53 65 96 54 44 42 90 79 97 31 75 67 21 93 62 81 82  3 11 58 73
            Card  83: 99  2 35 20 25 41 46 88 23 37 |  6 89 28 32 36 93 62 78 61 53 58 16  5 98 67 15  1 24 80 83 31 76 33 82 73
            Card  84: 13  5 98 70 63 92 79 10 53 84 | 32 10 41 63 92 40 85 53 88 57 13  5 70 93 96 54 29 30 44 22 98 84 79 21 78
            Card  85: 20 32 69 15  1 29  2 87 45 10 | 43 15 36  7 32 95 91 90 75 45  6 65 10 62 20  1  2 73 72 51 85 25 29 33 49
            Card  86: 74 56 19  8  4 35 78 43 75 26 | 74 30 77 56 42 14 19  1 28 18 16 99 63 72  8 90 43 15 20  4 78 23 35 75 26
            Card  87: 40 16 56 70 55 65 23 24 78 47 | 59 56 55 29 64 91 42 93 96 54 13 67 14 20 23 76  6 60 63 21 27 61 85 10 15
            Card  88: 50 48 45 15 51 65 97 22 12 39 | 59 93 75 12 30 39  5 85 32 56 29 50 80 82 21 38 96 65 14 53 62 51 15 23 54
            Card  89: 89 77 17 59 37 15 50 63 40 60 | 44 63 77 40 74 15 37 62 43 24 32 81 55 98 86 16 71 75 84 54 76 67 42 90 10
            Card  90: 63 97 57 10 38 19 42 69 73 56 |  3 89 60 13 33 24 70 97 27 56 15 21 19 83 63 42 36 68 57 38 10 73 77 17  5
            Card  91: 13 62 75 45 36 96 69 50 73 74 | 86  7 13 52 60 22 91 14 67 62 85 54 21 32 75  2 55 69 40 42 77 57  9 84 41
            Card  92: 54 97 73 17 67 58 69 27 21  7 | 38 66 59 27 69  7 71 68 83  2 58 44 11 89 10 75 70 63 61 73 94  5 41 45 17
            Card  93: 52 51 28 93 45 31 90 25 53 60 | 31 65 24 87 74 54 66 69 30  7 50 52  9 93 16 51 53 95 18 22  8 90 99 97 43
            Card  94: 73 17 53 21 91  3 85 89 36 92 | 53 57  5 44 55 90 74 20 51 77 41 81 75 31 63 23 76 18 48 82 45 40  9 37 54
            Card  95:  5 88 23 61 63 38 78 82 42 45 | 45 26  5 91 71 40  4  1 94 47 14 55 10 95 81 44 25 24 78 20  3 43 61 50 66
            Card  96: 70 85 30 35 56 69 83 47 18 33 |  6 93 99 79 52  5 69 48 57 85  4 11  9 22 43 53 87 21 33 56 39 31 83 32 67
            Card  97: 52 27  4 63 32 54 89 45 19 40 | 86 50  2 75 72 79 23 85 12 60 34 76 15 55 17 68 90 78 14 95 41 26 33 38 24
            Card  98: 89 50 13 23 47 26 27 90 92 24 | 45 74 96 24 40 46 23 17 76 15 53 25 29 27 92 58 20 12 37 35 72 10 32 47 26
            Card  99: 12 47 10 35 30  8 57 83 84 39 | 73 98 81 88 34 52 33  8 86 55 47 83 84 66 23 29 62 21 70  1 64 49 53 15 94
            Card 100: 45 99 97 10 17  5 44 54 96 88 | 95 91 34 50 61 15 81 20 99 14 69 33 48 75 16  9 29 98 41 80 53 77 89 56 72
            Card 101: 20 85 57 67 47 54 58 65 95 32 | 49 88 10 23 48 93 85 95 69 75 38 25 78 45 12 80 26 14 32 24  8 21 99 77  1
            Card 102: 52 32 10 58  7 99 74  1 59 50 | 29 97 91 78 53 11 96 25 79 43 77 72 60 66 81 55 69 93 50 48 98 54 39 87 10
            Card 103: 28 95 73 79 26  5 60 56 40 59 | 49 14 29 94 69 86 82 85  1 83 81 54  8 43 71 99 32 35 78  5 87 57 15 44 48
            Card 104: 68 70 17 29 85 16 48 21  2 34 | 14 45 36 72 99 94 62 37 74 84 97 75 31 35 83 19  1 15 91 57 61 40 79 77  5
            Card 105: 18 33 10 82 88  2 61 81 41 15 | 33 34 15  1 82 63 77  3 57 67 10 88 93 13 84 39 68  5 54 96 64 41  7 18  4
            Card 106: 67 56 16 10 80 60 62 61 64 51 | 51 10 21 64 71 80  9 98 67  4 60 45 16 85 92 27 56 91 61 43 62 68  1 77 35
            Card 107: 58 50 49 28 39 22  8 63 72  1 | 25 45  9  7 88 71 94  3 54 66 27 85 32  1  8 10 22 84 69 92 86 39 61 50 49
            Card 108: 41 38 72  4 70 66 61 87 42 83 | 69 99 89 43 50 15  5 29 68 28 86 37 66 38 27 19 95 41 62 82  2 87 33 20 42
            Card 109: 85 35 69 74 73 23 29 31 11 92 | 33 36 29 28 92 87 70 62  5 24 95 38 11 48 47 75 81 35 51 15 94 90 32 78 13
            Card 110: 86  6 55 64 15 23 66 51 77 67 | 51 44 89 73 22 29 58 17 77 48 18 53 75 91 57 60 30 63 96 93 33 79 68 86 12
            Card 111: 34 40 48 88 11  4  2 41 39 66 | 15 47 42 81  4 24 52 10 13 34 90  2 79 45 99 96 31 92 23 63 11 76 44 73 70
            Card 112: 34 74 86 79 37 30 31 51 41 13 | 75 21 14 78  1 93 60 96 56 77 65 29 34 58 22 90 79 59  8 19 42 46 33  2  4
            Card 113: 50 16 13 39 17 98 73 48 30 66 | 37 41 97 57  6 91 99 86 78  5 94 85 15 88  3 83 21 26 16 98 51 27 60 46 50
            Card 114: 36 74 72 70 13 27 42 21  8 65 | 39 27 16 94 75  5 87 66 51 68 72 67 85 19 21 91 65 10 56 86 84 40 83 36 43
            Card 115: 55 68 47 93 44 19 40 33 69 51 | 20 89 33 84 77 79 10 14 26 43 16 78 37 41 62 49 95  3 30 82  5 18 50 86 69
            Card 116: 40  7 60 18 17 94 24 12 79 59 | 91 22 74 80 35 10 20 97 25 78 24 95 18 70 26 33  8 41 71 32 77 15 72 86 61
            Card 117: 53  5 43 22 37 55 42 62 87 50 | 24 57 27 11 17 20 83 86 81 18 21 42 79 62 69 25 88 36 13 65 78 80 28 93  7
            Card 118: 49 76 39 26 97 89  9  2 79 40 | 77 99 93 86 89  3 24 47 52 13 19  6 44 25 73 71 70 34 46 28 97 61 92 35 20
            Card 119: 35 97 25 47 40 18 86 45 64 38 | 83 33 66 73 58 22 14 77 45 16 98 61 63 99  4 40 72 30 50 46 51 44 49 84 70
            Card 120: 86 99 32 14 39 90 15  6 89 76 | 56 18 35 31  5 83 68 42 47 36 33 81 69 43 46 20 97 53 94 60 23 71 26 24 52
            Card 121: 53 39 83  9  7 59 20 14 17 93 | 92 78 49 16 62  3 60 29 48 88 56 33 68 64 15 35 99 94 55 41 77 74 90 11 86
            Card 122: 39 62 24 68 36 29 38 15 96 76 | 79 99 71 75 70 43 20 21 41 82  7 17  9 57 85  8 33 30 80 26 93 63 12  5 54
            Card 123: 34 53 21 57 84 19 45 62 50  4 | 27 59 88 78 36  3 22 28 29 10 54  9 48 47 76 91 25 13 96  8 11 42 46 74 40
            Card 124: 51 30 87 45 39 11 33 94 22 65 | 11 30 18 39  2 33 90 87 37 72 51 81 67 45 78 48 14 94 56 27 65 22 20 10 76
            Card 125: 94 87 72 21 28  6 59 60 61 35 | 46 92 59  4 60 76 71  3 53 35  8 86 51 72 47 73 25 28 87 55 43 63 21 66 50
            Card 126: 77 83 56 19 75 12 58 54 64 90 | 18 53 50 77 44 57 74 64 61 65 62  5 91 85 26  1 66 71 41 68 92 69 27  9 93
            Card 127:  2 14 39 77 97 63 33 24 46 35 |  8 79 91 46 28 66 58 75 80 73 85 34  2 95 33 38 35 69 67 26 23 63 78 87 97
            Card 128: 23 78 35 17 80 32 90 34 30 63 | 42 80  4 32 37 27  8 23 40 65 17 41 43 90 34 81 35 63 83 46 30 78 72 55 82
            Card 129: 77 84 23 99 78 95 40  4 55 56 | 46 49 44 65 36 66 94 64 34 16 30 81 62 77 82 28 31 18 86 32 74 75 88 99 71
            Card 130: 50 93 25 12 24 66 38 29 46 76 | 99 27 29 84 66  7 47 34 93  4 60 24 25 46 12  9 33 86 38 69 76 48 81 85 21
            Card 131: 89 68 69 30 59 72 26 70 42 51 | 78 74 18 68 89 22 72 40 67 34 84 39 43 76 81 63 30 54 36 24 51 98  1 19 59
            Card 132: 70 73  4 37 15 86 65  7 77 85 | 79 21  5 55 81 99 25 27 94 33 40 34 20 29 68 93 98 16 90 54 71 13 78 60 74
            Card 133: 11 89 21 87 56  1 70 79 36 63 | 18 59 80 62 76 39 25 27 53 48 45 47 89  2 85 55 91 15 84 31 33 64 28 30  8
            Card 134: 69 96 77 54 67 24 46 22 81 42 | 79 73 45 14 97 39 61 74 95 58 82 43 89  2  4 36 21 78 37 51 65 47 53 99 20
            Card 135: 33 92  5 46 16 18 10 32 77 67 | 72 30 76  7 33 64 18 54  6 85 73  3 68 96 92 45 61 32 88 56 87 42 39 84 86
            Card 136: 27 53 79 82 18  4 75 63 11 39 | 66 55 93 24 58 83 33 92 61 68 26 72 80 49  8 36 22 43 63 20 84 69 21 97 88
            Card 137: 22 50 61  6 94 62 90 73 25 46 | 66 99 72 17 77 57  6 73 48 34  5 58 13 16  2 36 35 23 78 42 12 27 24 29 37
            Card 138: 18 19 59 57 23 68 61 78 91 65 |  2  4 83 10 20 85 51 73 30  1 31  9 80 27 24 92  3 42 71 67 84 54 82 45 76
            Card 139: 23 41 36 48 93 74 10 73  6 18 | 47 84 14 89 62 39 37 94 15 46 97 98 29 17 72 32 64 31 99 76 86 45 75 61  5
            Card 140: 99 71 15 31 61 53  4 62 69 77 | 58  8 86 44 59 47 91 38 30 25 42 82 11 19 49 75  9 83 20 90  1 94 12 22 52
            Card 141: 55 87 74 50 52  9 76 90 73 25 | 39 50 27 56 57 70 66 92 24 23 32 72 45 75 31 78 74 99  8 79  1 43  5  2 53
            Card 142: 28 66 34 64 49 48 91 73 81 41 | 54 78 66 59 32 43 41 85 15 64 68 56  4 46 90 67 63 45 24 16 53 74 52 75 81
            Card 143: 30 38 11 31 66 26 40 93 88 14 | 30 88 31 15 93 27 55 96 11 17 20 35 14 10 66 51 40 18 57 53 24  9 26 58 38
            Card 144: 70  6 21 59 69 99 30 43 60 58 | 60  1 11 69 70 59 66 79 58 73 54 30 33 55 21 89 14 43 99  4 52 82  6  2 96
            Card 145: 25 44 26 89 81  7 57 40 59  9 | 95 20 38 12 57 32 80 34 25 94 40 59  9 65 93 68 42 86 44 89 81  7 70 26 62
            Card 146: 90 61 87 20 26 51 33 19 64 10 |  1 84 80 86 22 50 73 69 78 70 12 34 65 54  5 83 44 98 82 57 31  3 37 59 79
            Card 147: 73 32 37 22 42 83 17 20 61 76 | 76 87 22 61 45 48 83 32 60  3 17  8 70 95 12  1 73 40 82 85 65 20 37 42 88
            Card 148: 16 40 86 29 68 67 22 42 96  9 | 34 83 95 91 40 11 74 99 25 16 15 76 52 12 75 43 92 59 17 98 88 36  4  5 28
            Card 149: 17 85 45 10 60 72 15 50 61 63 | 85 11 92 50 10  8 37 23 96 20 69 15 40 72 89 60 61 17 54 63 27 28 31 24 45
            Card 150: 64 87 43 20 97 83 69 61 95 56 | 29 76 26 52 65 84 62 11 49 92 90 77 47 46 42 50 33 37 98  2  5  9 71 22 85
            Card 151: 12 23 35 44 39 73  1 30 95 33 | 36 10 58 60 72  6 43 64 75  1 16 93 15 89 68 56 99 88 32 52 18 22 98 67 66
            Card 152: 69 85 51  1 46 21 31 19 44 49 | 45 92  8 10 80 94 32 33 88 28 16 86 23 93  6 96 25 81  3 52 64 90 76 95 59
            Card 153: 42 74 39 36 45 26 25  1 68 12 | 70 33 74 79 96  1  4 88 34 76 59 61 75 27 92 72 57 98 51 10 29 52 64 26 35
            Card 154: 85 95 68 79 28 93 46 65 38 14 | 52 18 50  9 79  2 88 43 85 31 83 74 21 38 87 73 56 63 71 15 59 72 13 64 46
            Card 155: 11 43 73 53 49 65 27 37 30 51 | 27 28 23 51 61 54 71 57 88  1 40 65 34 53 73 83 24 82 15 32 46 11 79 77 80
            Card 156: 64 26  5 45 59 81 23 43 27 44 | 11 16 77 37 62 98 10 21 71 35 28 13 74 65 89 78 45 61 96  7 12 53 51 52 44
            Card 157: 27 87 50 60 44 48 85  1 33  8 | 84 41  4 36  8 49 26 43 52 95 77  3 75 72 54 66 98 21 19 61 18 78 30 37  1
            Card 158: 31 88 37 91 12 56 65 79 95 17 | 64 32 43  1 45  8 99 94 39 16 77 18 84 70 24 51 27 93 59 25 41 73 78 34 44
            Card 159: 88 86 84 17 93 64  3 14 59 32 | 37 21 60 95 23 62 45 98 56 78 20 81 99 22 12 57  5 36 51 10 93 72 58  8 13
            Card 160: 49 37 23 63 73 64 16 70 19 32 | 75 74 21 77 37 56 30 76 66  1 51  9 97 95 38 40 29 69 67  6 47 50 45 71 87
            Card 161: 10 67 21 27 39 79 22 92 47 42 |  9 80 82 75 20  2 53 64 76 96 31 61 50 12 16 15 38 18 90 59 65 70 55 99  6
            Card 162: 89 99 28 87  4 43 30 48 46 82 | 46 51 59 81 14 40 82  5 85 48 97 28 11  7 87 64 58 38 92 30 89 99 27 36 34
            Card 163:  2 91 88 95 38 31 92  3 27 60 | 76 31 38 29 60 68 22 87 71 11 95 48 50 21 88 26 44 32  2 69 27 45 94 79 81
            Card 164: 87 91 47 40  2 71 95 99 49 74 | 34  8 65 49 45 43 51 81  9 13 80 38 33 88 82 74 28 60 91 48  1 55 42 95 56
            Card 165: 57 74 54 91 94 96 55 64 39 75 | 67  9 10 80 75 77 12 87 95 50 25 19 82 65 76 66 23 30 45 61 79 17 72 37  7
            Card 166: 16  4 73 97 28 19 17 10 57 43 | 73 97 24 57 43 58 72 10  2 34  9 63 70 64 68 28 44 16 99 61  4 78 46 83 19
            Card 167: 53 51 67 68 18 26 55 10 69  9 | 46 64 73 12 97 89 67 22 30 82 14 27 36  4 72 19 61 13 93 24 91 63 48 21 87
            Card 168: 69 32 33 90 72 88 55 80 15 27 | 10 80 14 55 64 39 97 82 24 96 74 46 73 68 65 87 43 59 16 53 84 38 79 11 47
            Card 169: 74 85 11 55 15 59 36  5 54 18 | 31 27 42 86 93 33 68 45 40 32 44 88 48 47 35 25  9 89 49 66 39  1 79 69  7
            Card 170:  9 49  3 88  4 31 13 64 38 41 | 71 38 32 13 91 61  3 48 88 63 17 19 59 75 98 36  6 49 50 44 20  2 45 67 53
            Card 171: 67 87 77 34 32 64 56 59 58 36 | 50 74 68 64 22 63 38 12 73 32 10 14 29 55 31 92 91  9 76 24 17 49 33 95  2
            Card 172:  4 80 72 36 37 99 64 16 22 31 |  8 78 95 47 18 36 55 59 23 86 70 54 60 98 13 25 80 75 74 48 16  7 81  5 11
            Card 173:  4 77 32 71 39 67 58 11 57 29 | 87 31 69 92 71 62 20 77 14 47 60 57 75 94 37 86 95 97 41 85 30 54 48 44 10
            Card 174: 21 12  8 71 74 77 31 82  1 16 |  4 89 75 19 91 51 10 71 88 78 30 80 47 96 28 65 99 35 13 48 73 29 53 49 42
            Card 175: 58 38 44 98 56 18 21 63 74  3 | 92 55 64 94 49 41 10 78  6 53 76 70 45 46 83 63 82 73 95 81 33 30 77 50 34
            Card 176: 94 39 50 86 44 81 88 55 54  9 | 16 60 84 75 52 81  4 10 37  6 56 14 22  8 90 41 45 70 12 23 89 31 28 80 87
            Card 177: 96 26 47 79 99  4 66 50 53 45 |  9 94 70 72 81 75 76 77 64 78 12 32 85 40 20 90 28  3 35 57 93 87 63 17 54
            Card 178: 33 65 93 44 97 37 12 41 83 24 |  2  1 46 10 55 82 58 57 52 64 81 76 74 99  5 89  9 14 42 27  6 19 66  3 15
            Card 179: 12 28 65 24  9 30 38 11 18 55 | 85 10 70 62 71 19 55 67 68 25 44 95 51 91 29 30 15 13 90 52 47 23 21 66 92
            Card 180: 65  7 39 97 64  3 50 27 46  4 | 12 59 88 91 43 61 36 18 58 85 26  5 23  1 62 56  8 49 66 72  9 67 90 52 95
            Card 181: 13 97 85 12 18 20 70  6 93 44 | 25 12 54 97 68 65 23 26 13  8 50 62 10 32 73 88 51 76 98 16 37  4 87 18 35
            Card 182: 65 35 63  2 34 77 15 89 16 54 | 36  2  6 62 81 58 34 99 54 48 57 15 52 16 30 65 83 53 38 10 41 77 63 35 21
            Card 183: 16 98 38  2 86 34 85 48 50 95 | 34 50  2 14 89  6 47 44 67 25 86 98 17 48 85 99 51 16 55 35 38 66 95 32 90
            Card 184:  4  1 48 73 89 69 47 81 94 59 | 20 18 50 30 87 17 99 76 74 61  9 84 40 46 51 97 65 98 56 53  3 77 11  8 34
            Card 185: 79 72 33 15 36 89 13 90 94 28 | 58 96 94 83 84 90 52 53 31  9 17 97  2 54 56 26 20 76 98 91  1 14 74 23 64
            Card 186: 36 34 86 79 32 64 15 94 72 35 | 12 94 19 11 35 78 59 91 86 47 55 36 53 21  9 45 77 18 17 72 10 79 49 34 74
            Card 187: 43 16 88 54 62 17 93 91 21 59 | 51  7 46 16 26  6 96 30 42  2 10 55 37 48 74 89 98 73 84 19  9 11 24 44 69
            Card 188: 50 59 97 84 11 57  7 94 29 92 | 19 79 94 25 18 55 24 89 65 11 53 93 83 92 27 85 48  4 68 97 46 64 40  1 74
            Card 189: 17 19 78 66 96 13 84 16 59 53 | 15 30 24 55 62 12 51 87 35 73 33  8  2 94 49 52 58 75 32 34 69 16 74 89 46
            Card 190: 61 96  6 49 37  3 26 12 99 11 | 87 95 57 43 23 53 36 25 84 69 38 67 46 92 74 16 18 27 80 19 13 54 12 39 61
            Card 191: 85 69  8 21 84 42 68 22 55 45 | 10 51 26  7 27  9 18 76 46 57 20 59 77 38 92 49 90 52 98 78 34 37 74 91 12
            Card 192: 58 46 36 50 72 87 96 63 83  5 | 69 11 88 81 52 97 43 42 26 41 47 54 78 67 51 95 18 62 13 99 30  3 34 89 53
            Card 193: 20 78 41  3 26  1 29 63  4 88 | 64 59 80 93 66 18 62 94 23 25 79 53 52 31 44 46 67 75 24  8 97 10 51 99 84
            Card 194: 40  5 28 23 65 78 63 94 60 83 | 45 54  2 74 75 11 55  8 68 41 30 51 47 33 88 58 37 36 64  1 21 72 77 25 99
        "#;
        assert_eq!(scratch_cards(input), 5554894);
    }
}
