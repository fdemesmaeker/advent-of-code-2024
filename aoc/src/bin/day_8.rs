use std::{collections::HashSet, fmt};
use std::hash::Hash;
use std::collections::HashMap;

use itertools::Itertools;

use aoc::utils::{get_input_path, read_contents};

struct Day {
    input_path: String
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
struct Pos {
    row: i32,
    col: i32
}

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.row, self.col)
    }
}

struct Game {
    n_rows: i32,
    n_cols: i32
}

type Antennas=HashMap<char, Vec<Pos>>;

fn parse_lines(input_path: &str) -> (Game, Antennas) {
    let contents: String = read_contents(input_path);
    let mut all_antennas: Antennas = HashMap::new();
    let mut row: i32 = 0;
    let mut col: i32 = 0;
    for line in contents.split("\n") {
        col = 0;
        for c in line.chars() {
            if c != '.' {
                let pos = Pos { row, col };
                if let Some(char_antennas) = all_antennas.get_mut(&c) {
                    char_antennas.push(pos);
                } 
                else {
                    let mut char_antennas: Vec<Pos> = Vec::new();
                    char_antennas.push(pos);
                    all_antennas.insert(c, char_antennas);
                }
            }
            col += 1;
        }
        row += 1;
    }
    (Game {n_rows: row, n_cols: col}, all_antennas)
}

fn is_antinode_valid(pos: &Pos, n_rows: i32, n_cols: i32) -> bool {
    pos.row >= 0 && pos.row < n_rows && pos.col >= 0 && pos.col < n_cols
}

fn get_antinodes(positions: &[Pos], game: &Game) -> HashSet<Pos> {
    let mut antinodes: HashSet<Pos> = HashSet::new();
    for pair in positions.iter().combinations(2) {
        let pos_a = pair[0];
        let pos_b = pair[1];
        let col_diff = pos_a.col.abs_diff(pos_b.col) as i32;
        let left_most = if pos_a.col < pos_b.col { pos_a } else { pos_b };
        let right_most = if pos_a.col > pos_b.col { pos_a } else { pos_b };

        let row_diff= pos_a.row.abs_diff(pos_b.row) as i32;
        let up_most = if pos_a.row < pos_b.row { pos_a } else { pos_b };
        let down_most = if pos_a.row > pos_b.row { pos_a } else { pos_b };
        if col_diff == 0 {
            
        }
        if row_diff == 0 {

        }

        /*  One point top left, the other down right
            a..
            ...
            ..a
        */
        if left_most == up_most {
            if down_most != right_most {
                panic!("NOPE NOPE NOPE");
            }
            let antinode_left = Pos {
                row: up_most.row - row_diff,
                col: left_most.col - col_diff
            };
            let antinode_right = Pos {
                row: down_most.row + row_diff,
                col: right_most.col + col_diff
            };
            antinodes.insert(antinode_left);
            antinodes.insert(antinode_right);
        }
        /*  One point down left, the other up right
            ..a
            ...
            a..
        */
        else if left_most == down_most {
            if up_most != right_most {
                panic!("NOPE NOPE NOPE");
            }
            let antinode_left = Pos {
                row: down_most.row + row_diff,
                col: left_most.col - col_diff
            };
            let antinode_right = Pos {
                row: up_most.row - row_diff,
                col: right_most.col + col_diff
            };
            antinodes.insert(antinode_left);
            antinodes.insert(antinode_right);
        }
    }
    antinodes.iter()
        .filter(|pos| is_antinode_valid(pos, game.n_rows, game.n_cols))
        .map(|pos| *pos)
        .collect()
}

fn print_antinodes(antinodes: &HashSet<Pos>, game: &Game) {
    for row in 0..game.n_rows {
        for col in 0..game.n_cols {
            let pos = Pos { row, col };
            let elem = if antinodes.contains(&pos) { "#" } else { "." };
            print!("{}", elem);
        }
        println!("");
    }
}

impl Day {
    fn part_1(&self) -> () {
        let (game, all_antennas) = parse_lines(&self.input_path);
        let mut all_antinodes: HashSet<Pos> = HashSet::new();
        for (k, antennas) in all_antennas {
            println!("Looking for antinodes for antenna {}...", k);
            let antinodes = get_antinodes( &antennas, &game);
            all_antinodes = all_antinodes.union(&antinodes).map(|pos| *pos).collect();
        }
        print_antinodes(&all_antinodes, &game);
        println!("Day 8 part 1: {}", all_antinodes.len());
    }

    fn part_2(&self) -> () {
    }
}

fn main() {
    let input_path: String = get_input_path(8);
    let day = Day {input_path};
    day.part_1();
    day.part_2();
}
