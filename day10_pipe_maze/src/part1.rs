use utils::split_and_clean_input_into_lines;

type Direction = (i8, i8);

const NORTH: Direction = (0, -1);
const SOUTH: Direction = (0, 1);
const WEST: Direction = (-1, 0);
const EAST: Direction = (1, 0);
const START: Direction = (0, 0);

#[derive(Debug)]
struct Pipe {
    pub val: char,
    pub directions: [Direction; 2],
}

impl Pipe {
    pub fn direction_match(&self, direction: Direction) -> bool {
        self.directions
            .iter()
            .find(|dir| dir.0 == direction.0 && dir.1 == direction.1)
            .is_some()
    }

    pub fn get_next_direction(&self, direction: Direction) -> Direction {
        let dir_a = self.directions[0];
        let dir_b = self.directions[1];

        // if we can continue in the same direction we'll take it
        if direction == dir_a || direction == dir_b {
            return direction;
        }

        // if we can't continue in the right direction then
        // we'll need to take a left or right
        if direction.0 == dir_a.0 || direction.1 == dir_a.1 {
            return dir_b;
        }
        dir_a
    }
}

const PIPES: [Pipe; 7] = [
    Pipe {
        val: '|',
        directions: [NORTH, SOUTH],
    },
    Pipe {
        val: '-',
        directions: [EAST, WEST],
    },
    Pipe {
        val: 'L',
        directions: [NORTH, EAST],
    },
    Pipe {
        val: 'J',
        directions: [NORTH, WEST],
    },
    Pipe {
        val: '7',
        directions: [WEST, SOUTH],
    },
    Pipe {
        val: 'F',
        directions: [EAST, SOUTH],
    },
    Pipe {
        val: 'S',
        directions: [START, START],
    },
];

const G: char = '.';
const S: char = 'S';

type Maze<'a> = Vec<Vec<Option<&'a Pipe>>>;

fn parse_input(input: &str) -> ((usize, usize), Maze) {
    let mut start_x = 0;
    let mut start_y = 0;
    let lines = split_and_clean_input_into_lines(input);

    let mut maze: Maze = vec![];

    for y in 0..lines.len() {
        let line = lines.get(y).unwrap();
        let mut maze_row = vec![];
        for x in 0..line.len() {
            let c = line.get(x..x + 1).unwrap().parse::<char>().unwrap();
            if c == G {
                maze_row.push(None);
                continue;
            }
            if c == S {
                start_x = x;
                start_y = y;
            }
            maze_row.push(PIPES.iter().find(|pipe| pipe.val == c));
        }
        maze.push(maze_row);
    }
    ((start_x, start_y), maze)
}

fn get_initial_direction(start_x: usize, start_y: usize, maze: &Maze) -> Direction {
    for (xo, yo) in [NORTH, SOUTH, WEST, EAST] {
        let yc = (start_y as i64 + yo as i64) as usize;
        let xc = (start_x as i64 + xo as i64) as usize;
        let row = maze.get(yc);
        if row.is_none() {
            continue;
        }
        let cell = row.unwrap().get(xc);
        if cell.is_none() {
            continue;
        }
        let pipe = match cell.unwrap() {
            Some(pipe) => *pipe,
            None => continue,
        };

        if pipe.direction_match((xo, yo)) {
            return (xo, yo);
        }
    }
    panic!("Should never arrive here");
}

pub fn solve(input: &str) -> u32 {
    let ((start_x, start_y), maze) = parse_input(input);

    let mut current_direction: Direction = get_initial_direction(start_x, start_y, &maze);

    let mut steps: u32 = 0;
    let mut x = start_x;
    let mut y = start_y;

    loop {
        steps += 1;
        x = (x as i64 + current_direction.0 as i64) as usize;
        y = (y as i64 + current_direction.1 as i64) as usize;

        let current_pipe = maze.get(y).unwrap().get(x).unwrap().unwrap();

        if current_pipe.val == S {
            break;
        }

        current_direction = current_pipe.get_next_direction(current_direction);
    }

    let half_steps = steps / 2;
    return half_steps + (half_steps % 2);
}

#[cfg(test)]
mod tests {
    use crate::consts::INPUT;

    use super::*;

    #[test]
    fn it_works() {
        let input = r#"
            .....
            .S-7.
            .|.|.
            .L-J.
            .....
        "#;
        assert_eq!(solve(input), 4);
    }

    #[test]
    fn it_works_too() {
        let input = r#"
            ..F7.
            .FJ|.
            SJ.L7
            |F--J
            LJ...
        "#;
        assert_eq!(solve(input), 8);
    }

    #[test]
    fn it_works_with_puzzle() {
        assert_eq!(solve(INPUT), 6956);
    }
}
