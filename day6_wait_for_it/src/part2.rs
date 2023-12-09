/*
    --- Day 6: Wait For It ---
    --- Part Two ---
    As the race is about to start, you realize the piece of paper with race times and record distances you got earlier actually just has very bad kerning. There's really only one race - ignore the spaces between the numbers on each line.

    So, the example from before:

    Time:      7  15   30
    Distance:  9  40  200
    ...now instead means this:

    Time:      71530
    Distance:  940200
    Now, you have to figure out how many ways there are to win this single race. In this example, the race lasts for 71530 milliseconds and the record distance you need to beat is 940200 millimeters. You could hold the button anywhere from 14 to 71516 milliseconds and beat the record, a total of 71503 ways!

    How many ways can you beat the record in this one much longer race?
*/

use utils::split_and_clean_input_into_lines;

pub struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    pub fn new(time: u64, distance: u64) -> Self {
        Self { time, distance }
    }

    pub fn get_number_of_ways_to_win(&self) -> u32 {
        let mut count: u32 = 0;
        for w in 0..self.time {
            let s = (self.time - w) * w;
            if s > self.distance {
                count += 1;
            }
        }
        count
    }
}

pub fn parse_input(input: &str) -> Race {
    let data = split_and_clean_input_into_lines(input)
        .iter()
        .map(|line| {
            line.split_once(':')
                .unwrap()
                .1
                .trim()
                .split_whitespace()
                .collect::<Vec<&str>>()
                .join("")
                .parse::<u64>()
                .unwrap()
        })
        .collect::<Vec<u64>>();

    if data.len() != 2 {
        panic!("Expected to have a time and distance in list");
    }

    let time = data.get(0).unwrap();
    let distance = data.get(1).unwrap();

    Race::new(*time, *distance)
}

pub fn solve(input: &str) -> u32 {
    parse_input(input).get_number_of_ways_to_win()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = r#"
            Time:      7  15   30
            Distance:  9  40  200
        "#;
        assert_eq!(solve(input), 71503);
    }

    #[test]
    fn it_works_with_puzzle() {
        let input = r#"
            Time:        42     89     91     89
            Distance:   308   1170   1291   1467
        "#;
        assert_eq!(solve(input), 24655068);
    }
}
