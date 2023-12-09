use utils::split_and_clean_input_into_lines;

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    split_and_clean_input_into_lines(input)
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|val| val.trim().parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>()
}

fn solve_history(initial_values: &Vec<i32>) -> i32 {
    let mut levels: Vec<Vec<i32>> = vec![initial_values.clone()];
    loop {
        let last_level = levels.last().unwrap();
        let mut new_level: Vec<i32> = vec![];

        for i in 1..last_level.len() {
            let previous = last_level.get(i - 1).unwrap();
            let current = last_level.get(i).unwrap();

            new_level.push(current - previous);
        }

        let is_only_zeros = new_level.iter().all(|&v| v == 0);

        levels.push(new_level);

        if is_only_zeros {
            break;
        }
    }

    levels
        .iter()
        .fold(0, |acc, level| acc + level.last().unwrap())
}

pub fn solve(input: &str) -> i32 {
    parse_input(input)
        .iter()
        .map(|history| solve_history(history))
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::consts::INPUT;

    use super::*;

    #[test]
    fn it_works() {
        let input = r#"
            0 3 6 9 12 15
            1 3 6 10 15 21
            10 13 16 21 30 45
        "#;

        assert_eq!(solve(input), 114);
    }

    #[test]
    fn it_works_too() {
        let input = r#"
        -3 10 36 70 97 86 -21 -325 -990 -2262 -4472 -7980 -12923 -18336 -19424 87 87451 366536 1136900 3075432 7624577
        "#;

        assert_eq!(solve(input), 17704920);
    }

    #[test]
    fn it_works_with_puzzle() {
        assert_eq!(solve(INPUT), 1731106378);
    }
}
