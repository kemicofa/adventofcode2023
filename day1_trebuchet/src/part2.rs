use utils::split_and_clean_input_into_lines;

/**
    Day 1: Trebuchet?!
    https://adventofcode.com/2023/day/1

    Something is wrong with global snow production, and you've been selected to take a look. The Elves have even given you a map; on it, they've used stars to mark the top fifty locations that are likely to be having problems.
    You've been doing this long enough to know that to restore snow operations, you need to check all fifty stars by December 25th.
    Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!
    You try to ask why they can't just use a weather machine ("not powerful enough") and where they're even sending you ("the sky") and why your map looks mostly blank ("you sure ask a lot of questions") and hang on did you just say the sky ("of course, where do you think snow comes from") when you realize that the Elves are already loading you into a trebuchet ("please hold still, we need to strap you in").
    As they're making the final adjustments, they discover that their calibration document (your puzzle input) has been amended by a very young Elf who was apparently just excited to show off her art skills. Consequently, the Elves are having trouble reading the values on the document.
    The newly-improved calibration document consists of lines of text; each line originally contained a specific calibration value that the Elves now need to recover. On each line, the calibration value can be found by combining the first digit and the last digit (in that order) to form a single two-digit number.

    For example:

    1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet

    In this example, the calibration values of these four lines are 12, 38, 15, and 77. Adding these together produces 142.

    Consider your entire calibration document. What is the sum of all of the calibration values?

    --- Part Two ---
    Your calculation isn't quite right. It looks like some of the digits are actually spelled out with letters: one, two, three, four, five, six, seven, eight, and nine also count as valid "digits".
    Equipped with this new information, you now need to find the real first and last digit on each line. For example:

    two1nine
    eightwothree
    abcone2threexyz
    xtwone3four
    4nineeightseven2
    zoneight234
    7pqrstsixteen

    In this example, the calibration values are 29, 83, 13, 24, 42, 14, and 76. Adding these together produces 281.
*/

fn byte_to_digit(byte: &u8) -> Option<u32> {
    (*byte as char).to_digit(10)
}

fn find_first_and_last_digits(line: &str) -> [(u32, Option<usize>); 2] {
    let bytes = line.as_bytes();

    let mut first: u32 = 0;
    let mut first_index: Option<usize> = Option::None;
    let mut last: u32 = 0;
    let mut last_index: Option<usize> = Option::None;

    for i in 0..bytes.len() {
        let left_byte = bytes.get(i).unwrap();
        let right_index = bytes.len() - 1 - i;
        let right_byte = bytes.get(right_index).unwrap();

        if first_index.is_none() {
            match byte_to_digit(left_byte) {
                Some(val) => {
                    first = val;
                    first_index = Some(i);
                }
                None => {}
            }
        };

        if last_index.is_none() {
            match byte_to_digit(right_byte) {
                Some(val) => {
                    last = val;
                    last_index = Some(right_index);
                }
                None => {}
            }
        };

        if first_index.is_some() && last_index.is_some() {
            break;
        }
    }

    [(first, first_index), (last, last_index)]
}

fn combine_two_digits(first: u32, second: u32) -> u32 {
    first * 10 + second
}

const DIGITS_STR_LIST: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn map_word_to_digit(word: &str) -> u32 {
    for i in 0..DIGITS_STR_LIST.len() {
        if word != DIGITS_STR_LIST[i] {
            continue;
        }
        return (i as u32) + 1;
    }
    panic!("Should never arrive here");
}

pub fn trebuchet(input: &str) -> u32 {
    let mut numbers: Vec<u32> = Vec::new();

    let lines: Vec<&str> = split_and_clean_input_into_lines(input);

    for line in lines {
        // if by coincidence the first or last character
        // are digits then we can save precious time
        let [(first_digit, first_digit_index), (last_digit, last_digit_index)] =
            find_first_and_last_digits(line);

        // since no number string length is longer than 3 characters
        // we can assume that if the number we found exists within the first 3 characters
        // it's really the first number (and same logic in reverse for the last)
        if first_digit_index.is_some()
            && first_digit_index.unwrap() <= 2
            && last_digit_index.is_some()
            && last_digit_index.unwrap() >= line.len() - 2
        {
            numbers.push(combine_two_digits(first_digit, last_digit));
            continue;
        }

        let mut first = first_digit;
        let mut first_index = match first_digit_index {
            Some(index) => index,
            None => line.len(),
        };

        let mut last = last_digit;
        let mut last_index = match last_digit_index {
            Some(index) => index,
            None => 0,
        };

        for digits_str in DIGITS_STR_LIST {
            let matches: Vec<_> = line.match_indices(digits_str).collect();
            for (index, value) in matches {
                let digit = map_word_to_digit(value);
                if index <= first_index {
                    first_index = index;
                    first = digit;
                }

                if index >= last_index {
                    last_index = index;
                    last = digit;
                }
            }
        }

        numbers.push(combine_two_digits(first, last));
    }
    return numbers.iter().sum();
}

#[cfg(test)]
mod tests {
    use crate::consts::INPUT;

    use super::*;

    #[test]
    fn it_works() {
        let input: &str = r#"
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet 
        "#;
        let result = trebuchet(input);
        assert_eq!(result, 142);
    }

    #[test]
    fn it_works_with_words_and_digits() {
        let input = "7z";
        let result = trebuchet(input);
        assert_eq!(result, 77);
    }

    #[test]
    fn it_works_with_puzzle_input() {
        let result = trebuchet(INPUT);
        assert_eq!(result, 53268);
    }
}
