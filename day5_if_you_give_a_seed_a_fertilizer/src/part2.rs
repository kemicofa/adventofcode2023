/*
    --- Day 5: If You Give A Seed A Fertilizer ---
    --- Part Two ---
    Everyone will starve if you only plant such a small number of seeds. Re-reading the almanac, it looks like the seeds: line actually describes ranges of seed numbers.

    The values on the initial seeds: line come in pairs. Within each pair, the first value is the start of the range and the second value is the length of the range. So, in the first line of the example above:

    seeds: 79 14 55 13
    This line describes two ranges of seed numbers to be planted in the garden. The first range starts with seed number 79 and contains 14 values: 79, 80, ..., 91, 92. The second range starts with seed number 55 and contains 13 values: 55, 56, ..., 66, 67.

    Now, rather than considering four seed numbers, you need to consider a total of 27 seed numbers.

    In the above example, the lowest location number can be obtained from seed number 82, which corresponds to soil 84, fertilizer 84, water 84, light 77, temperature 45, humidity 46, and location 46. So, the lowest location number is 46.

    Consider all of the initial seed numbers listed in the ranges on the first line of the almanac. What is the lowest location number that corresponds to any of the initial seed numbers?

    Your puzzle answer was 56931769.
*/

use std::{
    ops::Range,
    slice::Iter,
    sync::{Arc, Mutex},
    thread,
};

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

fn get_seed_ranges(line: &str) -> Vec<Range<u32>> {
    let seed_and_ranges = line
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .split_whitespace()
        .map(|val| val.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let mut ranges = Vec::new();
    let mut i = 0;
    loop {
        if i >= seed_and_ranges.len() {
            break;
        }
        let start = seed_and_ranges[i];
        let end = start + seed_and_ranges[i + 1] - 1;
        ranges.push(start..end);
        i += 2;
    }
    ranges
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

fn parse_input(input: &str) -> (Vec<Range<u32>>, [Vec<SourceToDestination>; 7]) {
    let lines = split_and_clean_input_into_lines(input);

    let mut lines_iter = lines.iter();

    let seed_ranges = get_seed_ranges(*lines_iter.next().unwrap());

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

    (seed_ranges, maps)
}

fn get_location_for_seed(seed: &u32, maps: &Arc<[Vec<SourceToDestination>; 7]>) -> u32 {
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
    let (seed_ranges, maps) = parse_input(input);

    let shared_value: Arc<Mutex<u32>> = Arc::new(Mutex::new(0xFFFFFFFF));
    let mut handles = vec![];

    let maps_arc = Arc::new(maps);

    // this will spawn 10 threads in the end
    // kekw if your pc can't support that
    for seed_range in seed_ranges {
        let maps_clone = Arc::clone(&maps_arc);
        let shared_value_clone = Arc::clone(&shared_value);
        handles.push(thread::spawn(move || {
            let mut min: u32 = 0xFFFFFFFF;
            for seed in seed_range.step_by(1) {
                let location = get_location_for_seed(&seed, &maps_clone);
                if location < min {
                    min = location;
                }
            }

            let mut min_mut = shared_value_clone.lock().unwrap();
            if min < *min_mut {
                *min_mut = min;
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let min_location = *shared_value.lock().unwrap();
    min_location
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

        assert_eq!(solve(input), 46);
    }

    #[test]
    fn it_works_with_puzzle() {
        assert_eq!(solve(INPUT), 56931769);
    }
}
