/*
    --- Day 2: Cube Conundrum ---
    You're launched high into the atmosphere! The apex of your trajectory just barely reaches the surface of a large island floating in the sky. You gently land in a fluffy pile of leaves. It's quite cold, but you don't see much snow. An Elf runs over to greet you.
    The Elf explains that you've arrived at Snow Island and apologizes for the lack of snow. He'll be happy to explain the situation, but it's a bit of a walk, so you have some time. They don't get many visitors up here; would you like to play a game in the meantime?
    As you walk, the Elf shows you a small bag and some cubes which are either red, green, or blue. Each time you play this game, he will hide a secret number of cubes of each color in the bag, and your goal is to figure out information about the number of cubes.
    To get information, once a bag has been loaded with cubes, the Elf will reach into the bag, grab a handful of random cubes, show them to you, and then put them back in the bag. He'll do this a few times per game.
    You play several games and record the information from each game (your puzzle input). Each game is listed with its ID number (like the 11 in Game 11: ...) followed by a semicolon-separated list of subsets of cubes that were revealed from the bag (like 3 red, 5 green, 4 blue).

    For example, the record of a few games might look like this:

    Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    In game 1, three sets of cubes are revealed from the bag (and then put back again). The first set is 3 blue cubes and 4 red cubes; the second set is 1 red cube, 2 green cubes, and 6 blue cubes; the third set is only 2 green cubes.

    The Elf would first like to know which games would have been possible if the bag contained only 12 red cubes, 13 green cubes, and 14 blue cubes?

    In the example above, games 1, 2, and 5 would have been possible if the bag had been loaded with that configuration. However, game 3 would have been impossible because at one point the Elf showed you 20 red cubes at once; similarly, game 4 would also have been impossible because the Elf showed you 15 blue cubes at once. If you add up the IDs of the games that would have been possible, you get 8.

    Determine which games would have been possible if the bag had been loaded with only 12 red cubes, 13 green cubes, and 14 blue cubes. What is the sum of the IDs of those games?
*/

use std::collections::HashMap;

use utils::split_and_clean_input_into_lines;

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<GameSet>,
}

#[derive(Debug)]
struct GameSet {
    cube_results: Vec<CubeResult>,
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum CubeColor {
    RED,
    BLUE,
    GREEN,
}

#[derive(Debug)]
pub struct CubeResult {
    color: CubeColor,
    count: u32,
}

impl CubeResult {
    pub fn new(count: u32, color: CubeColor) -> Self {
        Self { color, count }
    }
}

fn map_color_str_to_cube_color(color: &str) -> CubeColor {
    match color {
        "red" => CubeColor::RED,
        "blue" => CubeColor::BLUE,
        "green" => CubeColor::GREEN,
        _ => panic!("Should never happen"),
    }
}

fn parse_input(input: &str) -> Vec<Game> {
    split_and_clean_input_into_lines(input)
        .iter()
        .map(|raw_game| {
            let (game_meta, sets_data) = raw_game.trim().split_once(':').unwrap();

            let game_id = game_meta.split_once(' ').unwrap().1.parse::<u32>().unwrap();

            let sets = sets_data
                .split(';')
                .map(|raw_set| {
                    let cube_results = raw_set
                        .split(',')
                        .map(|raw_balls_data| {
                            let (ball_count_str, color) =
                                raw_balls_data.trim().split_once(' ').unwrap();
                            let cube_color = map_color_str_to_cube_color(color);
                            let ball_count = ball_count_str.parse::<u32>().unwrap();

                            CubeResult {
                                count: ball_count,
                                color: cube_color,
                            }
                        })
                        .collect::<Vec<CubeResult>>();

                    GameSet { cube_results }
                })
                .collect::<Vec<GameSet>>();

            Game { id: game_id, sets }
        })
        .collect::<Vec<Game>>()
}

type ExpectedCubeResults = HashMap<CubeColor, u32>;

pub fn cube_conundrum(input: &str, expected_cube_results: ExpectedCubeResults) -> u32 {
    let games = parse_input(input);

    let mut game_id_sum: u32 = 0;
    for game in games {
        let mut game_is_possible = true;
        for set in game.sets {
            for cube_result in set.cube_results {
                let expected_cube_result = expected_cube_results.get(&cube_result.color).unwrap();
                if cube_result.count > *expected_cube_result {
                    game_is_possible = false;
                    break;
                }
            }
            if !game_is_possible {
                break;
            }
        }
        if game_is_possible {
            game_id_sum += game.id;
        }
    }

    game_id_sum
}

#[cfg(test)]
mod tests {
    use crate::consts::INPUT;

    use super::*;

    #[test]
    fn it_works() {
        let input = r#"
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        "#;

        let mut expected_cube_result = HashMap::new();
        expected_cube_result.insert(CubeColor::BLUE, 14);
        expected_cube_result.insert(CubeColor::RED, 12);
        expected_cube_result.insert(CubeColor::GREEN, 13);
        assert_eq!(cube_conundrum(input, expected_cube_result), 8);
    }

    #[test]
    fn it_works_for_puzzle() {
        let mut expected_cube_result = HashMap::new();
        expected_cube_result.insert(CubeColor::BLUE, 14);
        expected_cube_result.insert(CubeColor::RED, 12);
        expected_cube_result.insert(CubeColor::GREEN, 13);
        assert_eq!(cube_conundrum(INPUT, expected_cube_result), 2256);
    }
}
