/*
    --- Day 5: If You Give A Seed A Fertilizer ---
    You take the boat and find the gardener right where you were told he would be: managing a giant "garden" that looks more to you like a farm.

    "A water source? Island Island is the water source!" You point out that Snow Island isn't receiving any water.

    "Oh, we had to stop the water because we ran out of sand to filter it with! Can't make snow with dirty water. Don't worry, I'm sure we'll get more sand soon; we only turned off the water a few days... weeks... oh no." His face sinks into a look of horrified realization.

    "I've been so busy making sure everyone here has food that I completely forgot to check why we stopped getting more sand! There's a ferry leaving soon that is headed over in that direction - it's much faster than your boat. Could you please go check it out?"

    You barely have time to agree to this request when he brings up another. "While you wait for the ferry, maybe you can help us with our food production problem. The latest Island Island Almanac just arrived and we're having trouble making sense of it."

    The almanac (your puzzle input) lists all of the seeds that need to be planted. It also lists what type of soil to use with each kind of seed, what type of fertilizer to use with each kind of soil, what type of water to use with each kind of fertilizer, and so on. Every type of seed, soil, fertilizer and so on is identified with a number, but numbers are reused by each category - that is, soil 123 and fertilizer 123 aren't necessarily related to each other.

    For example:

    seeds: 79 14 55 13

    seed-to-soil map:
    50 98 2
    52 50 48

    soil-to-fertilizer map:
    0 15 37
    37 52 2
    39 0 15

    fertilizer-to-water map:
    49 53 8
    0 11 42
    42 0 7
    57 7 4

    water-to-light map:
    88 18 7
    18 25 70

    light-to-temperature map:
    45 77 23
    81 45 19
    68 64 13

    temperature-to-humidity map:
    0 69 1
    1 0 69

    humidity-to-location map:
    60 56 37
    56 93 4
    The almanac starts by listing which seeds need to be planted: seeds 79, 14, 55, and 13.

    The rest of the almanac contains a list of maps which describe how to convert numbers from a source category into numbers in a destination category. That is, the section that starts with seed-to-soil map: describes how to convert a seed number (the source) to a soil number (the destination). This lets the gardener and his team know which soil to use with which seeds, which water to use with which fertilizer, and so on.

    Rather than list every source number and its corresponding destination number one by one, the maps describe entire ranges of numbers that can be converted. Each line within a map contains three numbers: the destination range start, the source range start, and the range length.

    Consider again the example seed-to-soil map:

    50 98 2
    52 50 48
    The first line has a destination range start of 50, a source range start of 98, and a range length of 2. This line means that the source range starts at 98 and contains two values: 98 and 99. The destination range is the same length, but it starts at 50, so its two values are 50 and 51. With this information, you know that seed number 98 corresponds to soil number 50 and that seed number 99 corresponds to soil number 51.

    The second line means that the source range starts at 50 and contains 48 values: 50, 51, ..., 96, 97. This corresponds to a destination range starting at 52 and also containing 48 values: 52, 53, ..., 98, 99. So, seed number 53 corresponds to soil number 55.

    Any source numbers that aren't mapped correspond to the same destination number. So, seed number 10 corresponds to soil number 10.

    So, the entire list of seed numbers and their corresponding soil numbers looks like this:

    seed  soil
    0     0
    1     1
    ...   ...
    48    48
    49    49
    50    52
    51    53
    ...   ...
    96    98
    97    99
    98    50
    99    51
    With this map, you can look up the soil number required for each initial seed number:

    Seed number 79 corresponds to soil number 81.
    Seed number 14 corresponds to soil number 14.
    Seed number 55 corresponds to soil number 57.
    Seed number 13 corresponds to soil number 13.
    The gardener and his team want to get started as soon as possible, so they'd like to know the closest location that needs a seed. Using these maps, find the lowest location number that corresponds to any of the initial seeds. To do this, you'll need to convert each seed number through other categories until you can find its corresponding location number. In this example, the corresponding types are:

    Seed 79, soil 81, fertilizer 81, water 81, light 74, temperature 78, humidity 78, location 82.
    Seed 14, soil 14, fertilizer 53, water 49, light 42, temperature 42, humidity 43, location 43.
    Seed 55, soil 57, fertilizer 57, water 53, light 46, temperature 82, humidity 82, location 86.
    Seed 13, soil 13, fertilizer 52, water 41, light 34, temperature 34, humidity 35, location 35.
    So, the lowest location number in this example is 35.

    What is the lowest location number that corresponds to any of the initial seed numbers?
*/

use std::slice::Iter;

use utils::split_and_clean_input_into_lines;

#[derive(Debug)]
struct SourceToDestination {
    destination_range_start: u32,
    source_range_start: u32,
    range_length: u32,
}

impl SourceToDestination {
    pub fn new(destination_range_start: u32, source_range_start: u32, range_length: u32) -> Self {
        Self {
            destination_range_start,
            source_range_start,
            range_length,
        }
    }

    pub fn get_destination(&self, source_id: u32) -> Option<u32> {
        let diff = source_id as i64 - self.source_range_start as i64;
        if diff >= 0 && diff < self.range_length as i64 {
            return Some(self.destination_range_start + diff as u32);
        }

        None
    }
}

fn get_seeds(line: &str) -> Vec<u32> {
    line.split_once(':')
        .unwrap()
        .1
        .trim()
        .split_whitespace()
        .map(|val| val.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
}

fn extract_source_to_destinations(
    lines_iter: &mut Iter<'_, &str>,
    key: &str,
) -> Vec<SourceToDestination> {
    let mut source_to_destinations: Vec<SourceToDestination> = Vec::new();

    if !lines_iter.next().unwrap().contains(key) {
        panic!("Expected {key} section");
    }

    loop {
        let line = match lines_iter.next() {
            Some(line) => *line,
            None => break,
        };

        if line.is_empty() {
            break;
        }

        let raw_source_to_destination = line
            .trim()
            .split_whitespace()
            .map(|val| val.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        if raw_source_to_destination.len() != 3 {
            panic!("Expected map line to have 3 elements");
        }

        source_to_destinations.push(SourceToDestination::new(
            raw_source_to_destination[0],
            raw_source_to_destination[1],
            raw_source_to_destination[2],
        ));
    }

    source_to_destinations
}

fn parse_input(input: &str) -> (Vec<u32>, [Vec<SourceToDestination>; 7]) {
    let lines = split_and_clean_input_into_lines(input);

    let mut lines_iter = lines.iter();

    let seeds = get_seeds(*lines_iter.next().unwrap());

    // next line is empty so we can skip it
    lines_iter.next().unwrap();

    let maps: [Vec<SourceToDestination>; 7] = [
        extract_source_to_destinations(&mut lines_iter, "seed-to-soil map:"),
        extract_source_to_destinations(&mut lines_iter, "soil-to-fertilizer map:"),
        extract_source_to_destinations(&mut lines_iter, "fertilizer-to-water map:"),
        extract_source_to_destinations(&mut lines_iter, "water-to-light map:"),
        extract_source_to_destinations(&mut lines_iter, "light-to-temperature map:"),
        extract_source_to_destinations(&mut lines_iter, "temperature-to-humidity map:"),
        extract_source_to_destinations(&mut lines_iter, "humidity-to-location map:"),
    ];

    (seeds, maps)
}

fn get_location_for_seed(seed: &u32, maps: &[Vec<SourceToDestination>; 7]) -> u32 {
    let mut source_value = *seed;
    for map in maps.iter() {
        for source_destination in map {
            match source_destination.get_destination(source_value) {
                Some(new_source_value) => {
                    source_value = new_source_value;
                    break;
                }
                None => continue,
            }
        }
        // if we don't match anything
        // then the destination value is the source value
    }
    source_value
}

pub fn solve(input: &str) -> u32 {
    let (seeds, maps) = parse_input(input);

    seeds
        .iter()
        .map(|seed| get_location_for_seed(seed, &maps))
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::consts::INPUT;

    use super::*;

    #[test]
    fn it_works() {
        let input = r#"
            seeds: 79 14 55 13

            seed-to-soil map:
            50 98 2
            52 50 48

            soil-to-fertilizer map:
            0 15 37
            37 52 2
            39 0 15

            fertilizer-to-water map:
            49 53 8
            0 11 42
            42 0 7
            57 7 4

            water-to-light map:
            88 18 7
            18 25 70

            light-to-temperature map:
            45 77 23
            81 45 19
            68 64 13

            temperature-to-humidity map:
            0 69 1
            1 0 69

            humidity-to-location map:
            60 56 37
            56 93 4
        "#;

        assert_eq!(solve(input), 35);
    }

    #[test]
    fn it_works_with_puzzle() {
        assert_eq!(solve(INPUT), 486613012);
    }
}
