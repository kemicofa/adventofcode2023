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
        let line = lines[y];
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

        for (px, py) in pipe.directions {
            if xc as i32 + (px as i32) == start_x as i32
                && yc as i32 + (py as i32) == start_y as i32
            {
                return (xo, yo);
            }
        }
    }
    panic!("Should never arrive here");
}

fn is_point_inside_polygon(point: (f64, f64), polygon: &Vec<(f64, f64)>) -> bool {
    let mut inside = false;

    for i in 0..polygon.len() {
        let j = (i + 1) % polygon.len();

        let xi = polygon[i].0;
        let yi = polygon[i].1;
        let xj = polygon[j].0;
        let yj = polygon[j].1;

        let intersect = ((yi > point.1) != (yj > point.1))
            && (point.0 < (xj - xi) * (point.1 - yi) / (yj - yi) + xi);

        if intersect {
            inside = !inside;
        }
    }
    inside
}

fn get_ground_vertices(maze: &Maze, polygon: &Vec<(f64, f64)>) -> Vec<(usize, usize)> {
    let mut vertices = vec![];
    for y in 0..maze.len() {
        let row = &maze[y];
        for x in 0..row.len() {
            let cell = row[x];
            if cell.is_none() || !polygon.contains(&(x as f64, y as f64)) {
                vertices.push((x, y));
            }
        }
    }
    vertices
}

pub fn solve(input: &str) -> u32 {
    let ((start_x, start_y), maze) = parse_input(input);

    let mut current_direction: Direction = get_initial_direction(start_x, start_y, &maze);

    let mut x = start_x;
    let mut y = start_y;

    let mut polygon: Vec<(f64, f64)> = vec![];

    loop {
        x = (x as i64 + current_direction.0 as i64) as usize;
        y = (y as i64 + current_direction.1 as i64) as usize;

        let current_pipe = maze[y][x].unwrap();

        polygon.push((x as f64, y as f64));
        if current_pipe.val == S {
            break;
        }

        current_direction = current_pipe.get_next_direction(current_direction);
    }

    get_ground_vertices(&maze, &polygon)
        .iter()
        .filter(|&&(x, y)| is_point_inside_polygon((x as f64, y as f64), &polygon))
        .count() as u32
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
        assert_eq!(solve(input), 1);
    }

    #[test]
    fn it_works_too() {
        let input = r#"
            ...........
            .S-------7.
            .|F-----7|.
            .||.....||.
            .||.....||.
            .|L-7.F-J|.
            .|..|.|..|.
            .L--J.L--J.
            ...........
        "#;
        assert_eq!(solve(input), 4);
    }

    #[test]
    fn it_works_as_well() {
        let input = r#"
        .F----7F7F7F7F-7....
        .|F--7||||||||FJ....
        .||.FJ||||||||L7....
        FJL7L7LJLJ||LJ.L-7..
        L--J.L7...LJS7F-7L7.
        ....F-J..F7FJ|L7L7L7
        ....L7.F7||L7|.L7L7|
        .....|FJLJ|FJ|F7|.LJ
        ....FJL-7.||.||||...
        ....L---J.LJ.LJLJ...
        "#;
        assert_eq!(solve(input), 8);
    }

    #[test]
    fn it_should_work_as_well_right() {
        let input = r#"
        FF7FSF7F7F7F7F7F---7
        L|LJ||||||||||||F--J
        FL-7LJLJ||||||LJL-77
        F--JF--7||LJLJIF7FJ-
        L---JF-JLJIIIIFJLJJ7
        |F|F-JF---7IIIL7L|7|
        |FFJF7L7F-JF7IIL---7
        7-L-JL7||F7|L7F-7F7|
        L.L7LFJ|||||FJL7||LJ
        L7JLJL-JLJLJL--JLJ.L
        "#;
        assert_eq!(solve(input), 10);
    }

    #[test]
    fn it_works_with_puzzle() {
        assert_eq!(solve(INPUT), 455);
    }
}
