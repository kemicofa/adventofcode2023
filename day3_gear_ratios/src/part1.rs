/*
    --- Day 3: Gear Ratios ---
    You and the Elf eventually reach a gondola lift station; he says the gondola lift will take you up to the water source, but this is as far as he can bring you. You go inside.
    It doesn't take long to find the gondolas, but there seems to be a problem: they're not moving.
    "Aaah!"
    You turn around to see a slightly-greasy Elf with a wrench and a look of surprise. "Sorry, I wasn't expecting anyone! The gondola lift isn't working right now; it'll still be a while before I can fix it." You offer to help.
    The engineer explains that an engine part seems to be missing from the engine, but nobody can figure out which one. If you can add up all the part numbers in the engine schematic, it should be easy to work out which part is missing.
    The engine schematic (your puzzle input) consists of a visual representation of the engine. There are lots of numbers and symbols you don't really understand, but apparently any number adjacent to a symbol, even diagonally, is a "part number" and should be included in your sum. (Periods (.) do not count as a symbol.)

    Here is an example engine schematic:

    467..114..
    ...*......
    ..35..633.
    ......#...
    617*......
    .....+.58.
    ..592.....
    ......755.
    ...$.*....
    .664.598..

    In this schematic, two numbers are not part numbers because they are not adjacent to a symbol: 114 (top right) and 58 (middle right). Every other number is adjacent to a symbol and so is a part number; their sum is 4361.
    Of course, the actual engine schematic is much larger. What is the sum of all of the part numbers in the engine schematic?
*/

use utils::split_and_clean_input_into_lines;

fn parse_input(input: &str) -> Vec<Vec<char>> {
    split_and_clean_input_into_lines(input)
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

#[derive(Debug)]
struct PartNumber {
    value: u32,
    x_start: usize,
    x_end: usize,
    y: usize,
}

impl PartNumber {
    pub fn is_adjacent(&self, x: usize, y: usize) -> bool {
        let x_offset = x + 1;
        let y_offset = y + 1;
        x_offset >= self.x_start
            && x_offset <= self.x_end + 2
            && y_offset >= self.y
            && y_offset <= self.y + 2
    }
}

pub fn gear_ratios(input: &str) -> u32 {
    let gears = parse_input(input);

    let mut part_numbers: Vec<PartNumber> = Vec::new();
    let mut symbols_positions: Vec<(usize, usize)> = Vec::new();

    for i in 0..gears.len() {
        let row = gears.get(i).unwrap();
        let mut start_index = Option::None;
        let mut current_gear_number = 0;

        for j in 0..row.len() {
            let cell = row.get(j).unwrap();
            if cell.is_digit(10) {
                if start_index.is_none() {
                    start_index = Some(j);
                }
                let digit = cell.to_digit(10).unwrap();
                current_gear_number = current_gear_number * 10 + digit;
                continue;
            }

            if *cell != '.' {
                symbols_positions.push((j, i));
            }

            if start_index.is_some() {
                part_numbers.push(PartNumber {
                    value: current_gear_number,
                    x_start: start_index.unwrap(),
                    x_end: j - 1,
                    y: i,
                });
                start_index = Option::None;
                current_gear_number = 0;
            }
        }

        if start_index.is_some() {
            part_numbers.push(PartNumber {
                value: current_gear_number,
                x_start: start_index.unwrap(),
                x_end: row.len() - 1,
                y: i,
            });
        }
    }

    let mut sum = 0;

    for part_number in part_numbers {
        for (x, y) in &symbols_positions {
            if part_number.is_adjacent(*x, *y) {
                sum += part_number.value;
                break;
            }
        }
    }

    return sum;
}

#[cfg(test)]
mod tests {
    use crate::consts::INPUT;

    use super::*;

    #[test]
    fn it_works() {
        let input = r#"
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
        "#;
        assert_eq!(gear_ratios(input), 4361);
    }

    #[test]
    fn it_works_with_puzzle() {
        assert_eq!(gear_ratios(INPUT), 532428);
    }
}
