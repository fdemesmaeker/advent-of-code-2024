use std::fmt;
use std::collections::HashSet;

use aoc::utils::{Challenge, get_input_path, read_contents};

struct Day6 {
    input_path: String
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Direction::North => write!(f, "North"),
            Direction::East => write!(f, "East"),
            Direction::South => write!(f, "South"),
            Direction::West => write!(f, "West")
        }
    }
}
#[derive(PartialEq, Eq, Clone, Copy)]
struct Guard {
    row: i32,
    col: i32,
    direction: Direction
}

impl fmt::Display for Guard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{},{}) ", self.row, self.col, self.direction)
    }
}

struct Game {
    rocks: HashSet<(i32, i32)>,
    guard: Guard,
    n_rows: i32,
    n_cols: i32
}

fn parse_lines(input_path: &str) -> Game {
    let contents: String = read_contents(input_path);
    let mut rocks: HashSet<(i32, i32)> = HashSet::new();
    let mut row: i32 = 0;
    let mut guard: Option<Guard> = None;
    let mut n_cols: i32 = 0;
    for line in contents.split("\n") {
        let mut column: i32 = 0;
        for c in line.chars() {
            if c == '#' {
                rocks.insert((row, column));
            } else if c == '^' {
                let _ = guard.insert(Guard { row, col:column, direction: Direction::North });
            }
            column += 1;
        }
        n_cols = column;
        row += 1;
    }
    Game {rocks, guard: guard.unwrap(), n_rows: row, n_cols }
}

fn next_straight_position(guard: Guard) -> (i32, i32) {
    match guard.direction {
        Direction::North => {
            (guard.row - 1, guard.col)
        }
        Direction::East => {
            (guard.row, guard.col + 1)
        }
        Direction::South => {
            (guard.row + 1, guard.col)
        }
        Direction::West => {
            (guard.row, guard.col - 1)
        }
    }
}

fn next_direction(direction: Direction) -> Direction {
    match direction {
        Direction::North => {
            Direction::East
        }
        Direction::East => {
            Direction::South
        }
        Direction::South => {
            Direction::West
        }
        Direction::West => {
            Direction::North
        }
    }
}

fn next_position(rocks: &HashSet<(i32, i32)>, mut guard: Guard) -> Guard {
    let mut next_position: (i32, i32) = next_straight_position(guard);
    while rocks.contains(&next_position) {
        guard.direction = next_direction(guard.direction);
        next_position = next_straight_position(guard);
    }
    Guard { row: next_position.0, col: next_position.1, direction: guard.direction }
}

fn contains_loop(game: Game) -> bool {
    let mut guard: Guard = game.guard;
    let mut n_visited_positions = 0;
    
    while guard.row < game.n_rows && guard.row >= 0 && guard.col < game.n_cols && guard.col >= 0 {
        if n_visited_positions > 25000 {
            return true;
        }
        guard = next_position(&game.rocks, guard);
        n_visited_positions += 1;
    }
    false
}

impl Challenge for Day6 {
    fn part_1(&self) -> i32 {
        let game = parse_lines(&self.input_path);
        let mut visited_positions: HashSet<(i32, i32)> = HashSet::new();
        let mut guard: Guard = game.guard;

        while guard.row < game.n_rows && guard.row >= 0 && guard.col < game.n_cols && guard.col >= 0 {
            //println!("Guard: {}", guard);
            let pos: (i32, i32) = (guard.row, guard.col);
            visited_positions.insert(pos);
            guard = next_position(&game.rocks, guard);
        }
        visited_positions.len().try_into().unwrap()
    }

    /* Test input solutions:
        - (6,3)
        - (7,6)
        - (7,7)
        - (8,1)
        - (8,3)
        - (9,7)
    */
    fn part_2(&self) -> i32 {
        let game = parse_lines(&self.input_path);
        let mut n_loops = 0;
        let mut loops: Vec<(i32, i32)> = vec![];
        for i in 0..game.n_rows {
            for j in 0..game.n_cols {
                if !game.rocks.contains(&(i, j)) && !(i == game.guard.row && j == game.guard.col ) {
                   println!("({},{}) - {}", i, j, n_loops);
                    let mut modified_rocks = game.rocks.clone();
                    modified_rocks.insert((i,j));
                    if contains_loop(Game {rocks: modified_rocks, guard: game.guard, n_rows: game.n_rows, n_cols: game.n_cols}) {
                        loops.push((i,j));
                        n_loops += 1;
                    }
                }
            }
        }
        //loops.iter().for_each(|x|println!("({},{})", x.0, x.1));
        n_loops
    }
}

fn main() {
    let input_path: String = get_input_path(6);
    let day = Day6 {input_path};
    day.run();
}
