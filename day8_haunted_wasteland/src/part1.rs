/*
    --- Day 8: Haunted Wasteland ---
    You're still riding a camel across Desert Island when you spot a sandstorm quickly approaching. When you turn to warn the Elf, she disappears before your eyes! To be fair, she had just finished warning you about ghosts a few minutes ago.

    One of the camel's pouches is labeled "maps" - sure enough, it's full of documents (your puzzle input) about how to navigate the desert. At least, you're pretty sure that's what they are; one of the documents contains a list of left/right instructions, and the rest of the documents seem to describe some kind of network of labeled nodes.

    It seems like you're meant to use the left/right instructions to navigate the network. Perhaps if you have the camel follow the same instructions, you can escape the haunted wasteland!

    After examining the maps for a bit, two nodes stick out: AAA and ZZZ. You feel like AAA is where you are now, and you have to follow the left/right instructions until you reach ZZZ.

    This format defines each node of the network individually. For example:

    RL

    AAA = (BBB, CCC)
    BBB = (DDD, EEE)
    CCC = (ZZZ, GGG)
    DDD = (DDD, DDD)
    EEE = (EEE, EEE)
    GGG = (GGG, GGG)
    ZZZ = (ZZZ, ZZZ)
    Starting with AAA, you need to look up the next element based on the next left/right instruction in your input. In this example, start with AAA and go right (R) by choosing the right element of AAA, CCC. Then, L means to choose the left element of CCC, ZZZ. By following the left/right instructions, you reach ZZZ in 2 steps.

    Of course, you might not find ZZZ right away. If you run out of left/right instructions, repeat the whole sequence of instructions as necessary: RL really means RLRLRLRLRLRLRLRL... and so on. For example, here is a situation that takes 6 steps to reach ZZZ:

    LLR

    AAA = (BBB, BBB)
    BBB = (AAA, ZZZ)
    ZZZ = (ZZZ, ZZZ)
    Starting at AAA, follow the left/right instructions. How many steps are required to reach ZZZ?
*/

use std::collections::HashMap;

use utils::split_and_clean_input_into_lines;

#[derive(Debug)]
enum Instruction {
    LEFT,
    RIGHT,
}

#[derive(Debug)]
struct Instructions {
    values: Vec<Instruction>,
    index: usize,
}

impl Instructions {
    pub fn new(values: Vec<Instruction>) -> Self {
        Self { values, index: 0 }
    }

    fn get_next_instruction(&mut self) -> &Instruction {
        let instruction = self.values.get(self.index).unwrap();
        self.index = (self.index + 1) % self.values.len();
        instruction
    }
}

#[derive(Debug)]
struct Node {
    left: String,
    right: String,
}

impl Node {
    pub fn new(left: String, right: String) -> Self {
        Self { left, right }
    }

    pub fn get_destination(&self, instruction: &Instruction) -> &String {
        match instruction {
            Instruction::LEFT => &self.left,
            Instruction::RIGHT => &self.right,
        }
    }
}

type Network<'a> = HashMap<&'a str, Node>;

fn parse_input(input: &str) -> (Network, Instructions) {
    let lines = split_and_clean_input_into_lines(input);
    let mut lines_iter = lines.iter();

    let instructions = lines_iter
        .next()
        .unwrap()
        .chars()
        .map(|val| match val {
            'R' => Instruction::RIGHT,
            'L' => Instruction::LEFT,
            _ => panic!("Should either be L or R instructions"),
        })
        .collect::<Vec<Instruction>>();

    // ignore empty line
    lines_iter.next();

    let mut network: Network = HashMap::new();

    for line in lines_iter {
        let (key, raw_destinations) = line.split_once(" = ").unwrap();
        let (destination_left, destination_right) = raw_destinations
            .trim_matches(|c: char| c.is_whitespace() || c.is_ascii_punctuation())
            .split_once(",")
            .unwrap();

        network.insert(
            key.trim(),
            Node::new(
                destination_left.trim().to_string(),
                destination_right.trim().to_string(),
            ),
        );
    }

    (network, Instructions::new(instructions))
}

const START_DESTINATION_ID: &str = "AAA";
const END_DESTINATION_ID: &str = "ZZZ";

pub fn solve(input: &str) -> u32 {
    let (network, mut instructions) = parse_input(input);

    let mut steps: u32 = 0;
    let mut current_node = network.get(START_DESTINATION_ID).unwrap();

    loop {
        let instruction = instructions.get_next_instruction();
        let next_node_id = current_node.get_destination(instruction);
        steps += 1;
        if next_node_id == END_DESTINATION_ID {
            break;
        }
        current_node = network.get(next_node_id.as_str()).unwrap();
    }

    steps
}

#[cfg(test)]
mod tests {
    use crate::consts::INPUT;

    use super::*;

    #[test]
    fn it_works() {
        let input = r#"
            RL

            AAA = (BBB, CCC)
            BBB = (DDD, EEE)
            CCC = (ZZZ, GGG)
            DDD = (DDD, DDD)
            EEE = (EEE, EEE)
            GGG = (GGG, GGG)
            ZZZ = (ZZZ, ZZZ)
        "#;
        assert_eq!(solve(input), 2);
    }

    #[test]
    fn it_works_too() {
        let input = r#"
            LLR

            AAA = (BBB, BBB)
            BBB = (AAA, ZZZ)
            ZZZ = (ZZZ, ZZZ)
        "#;
        assert_eq!(solve(input), 6);
    }

    #[test]
    fn it_works_with_puzzle() {
        assert_eq!(solve(INPUT), 12737);
    }
}
