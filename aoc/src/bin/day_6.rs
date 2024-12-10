use std::fmt;
use std::collections::HashSet;

use aoc::utils::{Challenge, get_input_path, read_contents};

struct Day6 {
    input_path: String
}

#[derive(PartialEq, Eq)]
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

fn parse_lines(input_path: &str) -> (HashSet<(i32, i32)>, Guard, i32, i32) {
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
    (rocks, guard.unwrap(), row, n_cols)
}
/*
fn move_guard(rocks: &HashSet<(i32, i32)>, guard: Guard, next_position: (i32, i32)) -> Guard {
    if rocks.contains(&next_position) {
        if guard.direction == Direction::North {
            Guard { row: guard.row, col: guard.col + 1, direction: Direction::East }
        }
        else if guard.direction == Direction::East {
            Guard { row: guard.row + 1, col: guard.col, direction: Direction::South }
        }
        else if guard.direction == Direction::South {
            Guard { row: guard.row, col: guard.col - 1, direction: Direction::West }
        }
        else {
            Guard { row: guard.row - 1, col: guard.col, direction: Direction::North }
        }
    }
    else {
        if guard.direction == Direction::North {
            Guard { row: guard.row - 1, col: guard.col, direction: guard.direction }
        }
        else if guard.direction == Direction::East {
            Guard { row: guard.row, col: guard.col + 1, direction: guard.direction }
        }
        else if guard.direction == Direction::South {
            Guard { row: guard.row + 1, col: guard.col, direction: guard.direction }
        }
        else {
            Guard { row: guard.row, col: guard.col - 1, direction: guard.direction }
        }
    }
}*/

fn get_next_position(rocks: &HashSet<(i32, i32)>, guard: Guard) -> Guard {
    match guard.direction {
        Direction::North => {
            if rocks.contains(&(guard.row - 1, guard.col)) {
                Guard { row: guard.row, col: guard.col + 1, direction: Direction::East }
            } else {
                Guard { row: guard.row - 1, col: guard.col, direction: guard.direction }
            }
        }
        Direction::East => {
            if rocks.contains(&(guard.row, guard.col + 1)) {
                Guard { row: guard.row + 1, col: guard.col, direction: Direction::South }
            } else {
                Guard { row: guard.row, col: guard.col + 1, direction: guard.direction }
            }
        }
        Direction::South => {
            if rocks.contains(&(guard.row + 1, guard.col)) {
                Guard { row: guard.row, col: guard.col - 1, direction: Direction::West }
            } else {
                Guard { row: guard.row + 1, col: guard.col, direction: guard.direction }
            }
        }
        Direction::West => {
            if rocks.contains(&(guard.row, guard.col - 1)) {
                Guard { row: guard.row - 1, col: guard.col, direction: Direction::North }
            } else {
                Guard { row: guard.row, col: guard.col - 1, direction: guard.direction }
            }
        }
    }
}

impl Challenge for Day6 {
    fn part_1(&self) -> i32 {
        let (rocks, initial_guard, n_rows, n_cols) = parse_lines(&self.input_path);
        let mut visited_positions: HashSet<(i32, i32)> = HashSet::new();
        let mut all_positions: Vec<(i32, i32)> = Vec::new();

        let mut guard: Guard = initial_guard;
        println!("n_rows: {}, n_cols: {}", n_rows, n_cols);

        while guard.row < n_rows && guard.row >= 0 && guard.col < n_cols && guard.col >= 0 {
            println!("Guard: {}", guard);
            let pos = (guard.row, guard.col);
            visited_positions.insert(pos);
            all_positions.push(pos);
            guard = get_next_position(&rocks, guard);
        }
        visited_positions.len().try_into().unwrap()
    }

    fn part_2(&self) -> i32 {
        42
    }
}

fn main() {
    let input_path: String = get_input_path(6);
    let day = Day6 {input_path};
    day.run();
}
