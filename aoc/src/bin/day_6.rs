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

fn contains_loop(game: Game) -> bool {
    let mut visited_positions: HashSet<(i32, i32)> = HashSet::new();
    let mut guard: Guard = game.guard;
    let mut current_loop: HashSet<(i32, i32)> = HashSet::new();
    
    while guard.row < game.n_rows && guard.row >= 0 && guard.col < game.n_cols && guard.col >= 0 {
        //println!("Guard: {}", guard);
        let current_pos: (i32, i32) = (guard.row, guard.col);
        guard = get_next_position(&game.rocks, guard);
        let next_pos: (i32, i32) = (guard.row, guard.col);
        
        // Need to detect loop correctly!!!!
        if visited_positions.contains(&current_pos) {
            if current_loop.contains(&current_pos) {
                return true;
            }
            else {
                current_loop.insert(current_pos);
            }
        }
        visited_positions.insert(current_pos);
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
            guard = get_next_position(&game.rocks, guard);
        }
        visited_positions.len().try_into().unwrap()
    }

    fn part_2(&self) -> i32 {
        let game = parse_lines(&self.input_path);
        let mut n_loops = 0;
        for i in 0..game.n_rows-1 {
            for j in 0..game.n_cols-1 {
                if !game.rocks.contains(&(i, j)) && !(i == game.guard.row && j == game.guard.col ) {
                    let mut modified_rocks = game.rocks.clone();
                    modified_rocks.insert((i,j));
                    if contains_loop(Game {rocks: modified_rocks, guard: game.guard, n_rows: game.n_rows, n_cols: game.n_cols}) {
                        n_loops += 1;
                    }
                }
            }
        }
        n_loops
    }
}

fn main() {
    let input_path: String = get_input_path(6);
    let day = Day6 {input_path};
    day.run();
}
