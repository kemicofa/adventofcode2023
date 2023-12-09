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
*/

pub fn trebuchet(input: &str) -> u32 {
    let mut numbers: Vec<u32> = Vec::new();
    let mut first: Option<u32> = Option::None;
    let mut last: Option<u32> = Option::None;

    let bytes = input.as_bytes();
    for i in 0..bytes.len() {
        let c = match bytes.get(i) {
            Some(val) => *val as char,
            None => '\n',
        };

        if c == '\n' {
            if first.is_none() {
                continue;
            }
            let unwrapped_first = first.unwrap();
            let unwrapped_last = match last {
                Some(val) => val,
                None => unwrapped_first,
            };
            numbers.push(unwrapped_first * 10 + unwrapped_last);
            first = Option::None;
            last = Option::None;
            continue;
        }

        match c.to_digit(10) {
            Some(digit) => match first {
                Some(_) => last = Some(digit),
                None => first = Some(digit),
            },
            None => continue,
        }
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
    fn it_works_with_puzzle_input() {
        let result = trebuchet(INPUT);
        assert_eq!(result, 53080);
    }
}
