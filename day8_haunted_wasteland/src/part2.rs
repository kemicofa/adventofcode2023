/*
    --- Day 8: Haunted Wasteland ---
    --- Part Two ---
    The sandstorm is upon you and you aren't any closer to escaping the wasteland. You had the camel follow the instructions, but you've barely left your starting position. It's going to take significantly more steps to escape!

    What if the map isn't for people - what if the map is for ghosts? Are ghosts even bound by the laws of spacetime? Only one way to find out.

    After examining the maps a bit longer, your attention is drawn to a curious fact: the number of nodes with names ending in A is equal to the number ending in Z! If you were a ghost, you'd probably just start at every node that ends with A and follow all of the paths at the same time until they all simultaneously end up at nodes that end with Z.

    For example:

    LR

    11A = (11B, XXX)
    11B = (XXX, 11Z)
    11Z = (11B, XXX)
    22A = (22B, XXX)
    22B = (22C, 22C)
    22C = (22Z, 22Z)
    22Z = (22B, 22B)
    XXX = (XXX, XXX)
    Here, there are two starting nodes, 11A and 22A (because they both end with A). As you follow each left/right instruction, use that instruction to simultaneously navigate away from both nodes you're currently on. Repeat this process until all of the nodes you're currently on end with Z. (If only some of the nodes you're on end with Z, they act like any other node and you continue as normal.) In this example, you would proceed as follows:

    Step 0: You are at 11A and 22A.
    Step 1: You choose all of the left paths, leading you to 11B and 22B.
    Step 2: You choose all of the right paths, leading you to 11Z and 22C.
    Step 3: You choose all of the left paths, leading you to 11B and 22Z.
    Step 4: You choose all of the right paths, leading you to 11Z and 22B.
    Step 5: You choose all of the left paths, leading you to 11B and 22C.
    Step 6: You choose all of the right paths, leading you to 11Z and 22Z.
    So, in this example, you end up entirely on nodes that end in Z after 6 steps.

    Simultaneously start on every node that ends with A. How many steps does it take before you're only on nodes that end with Z?
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

    fn get_instruction(&mut self) -> &Instruction {
        self.values.get(self.index).unwrap()
    }

    // indicate if moving forward
    // reset the index
    fn go_forward(&mut self) -> bool {
        self.index = (self.index + 1) % self.values.len();

        self.index == 0
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

// Function to calculate the greatest common divisor (GCD) of two numbers
fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        0
    } else {
        (a * b) / gcd(a, b)
    }
}

fn lcm_of_numbers(numbers: Vec<u32>) -> u64 {
    numbers.iter().fold(1, |acc, &x| lcm(acc, x as u64))
}

pub fn solve(input: &str) -> u64 {
    let (network, mut instructions) = parse_input(input);

    let starting_node_ids: Vec<&str> = network
        .keys()
        .map(|c| *c)
        .filter(|key| key.ends_with('A'))
        .collect::<Vec<&str>>();

    let mut node_keys = starting_node_ids
        .iter()
        .map(|v| Some(*v))
        .collect::<Vec<Option<&str>>>();

    let mut steps = 0;
    let mut steps_completed_list: Vec<u32> = vec![];
    loop {
        let mut next_node_keys: Vec<Option<&str>> = vec![];
        steps += 1;
        for i in 0..node_keys.len() {
            let node_key = node_keys.get(i).unwrap();

            if node_key.is_none() {
                next_node_keys.push(None);
                continue;
            }

            let node_key = node_keys.get(i).unwrap();
            let instruction = instructions.get_instruction();
            let current_node = network.get(node_key.unwrap()).unwrap();
            let next_node_id = current_node.get_destination(instruction);

            if next_node_id.ends_with('Z') {
                steps_completed_list.push(steps);
                next_node_keys.push(None);
            } else {
                next_node_keys.push(Some(&next_node_id));
            }
        }

        instructions.go_forward();

        if steps_completed_list.len() == starting_node_ids.len() {
            break;
        }

        node_keys = next_node_keys;
    }

    lcm_of_numbers(steps_completed_list)
}

#[cfg(test)]
mod tests {
    use crate::consts::INPUT;

    use super::*;

    #[test]
    fn it_works() {
        let input = r#"
            LR

            11A = (11B, XXX)
            11B = (XXX, 11Z)
            11Z = (11B, XXX)
            22A = (22B, XXX)
            22B = (22C, 22C)
            22C = (22Z, 22Z)
            22Z = (22B, 22B)
            XXX = (XXX, XXX)
        "#;
        assert_eq!(solve(input), 6);
    }

    #[test]
    fn it_works_with_puzzle() {
        assert_eq!(solve(INPUT), 9064949303801);
    }
}
