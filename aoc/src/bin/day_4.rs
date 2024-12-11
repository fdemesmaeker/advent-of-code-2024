use core::fmt;

use array2d::Array2D;
use regex::Regex;


use aoc::utils::{get_input_path, read_contents};

struct Day {
    input_path: String
}

#[derive(PartialEq, Eq)]
enum Chars {
    X,
    M,
    A,
    S,
    UNKNOWN
}

impl fmt::Debug for Chars {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Chars::X => write!(f, "X"),
            Chars::M  => write!(f, "M"),
            Chars::A  => write!(f, "A"),
            Chars::S  => write!(f, "S"),
            Chars::UNKNOWN  => write!(f, "UNKNOWN"),
        }
    }
}

/*
println!("First row");
for element in first_row {
    print!("{} ", element);
}
println!("Last row");
for element in last_row {
    print!("{} ", element);
}
println!();
*/

fn get_char(c: &char) -> Chars {
    if c.eq(&'X') {
        Chars::X
    } else if c.eq(&'M') {
        Chars::M
    } else if c.eq(&'A') {
        Chars::A
    } else if c.eq(&'S') {
        Chars::S
    } else {
        Chars::UNKNOWN
    }
}

fn to_char(c: &Chars) -> char {
    match c {
        Chars::X => {'X'}
        Chars::M => {'M'}
        Chars::A => {'A'}
        Chars::S => {'S'}
        Chars::UNKNOWN => {'_'}
    }
}

fn build_diagonal_from_top_to_right_bottom(array: &Array2D<char>, start_col: &usize) -> Vec<Chars> {
    let mut diagonal: Vec<Chars> = vec![];
    for row in 0..array.num_rows() {
        let col = start_col + row;
        let item: Option<&char> = array.get(row, col);
        match item {
            None => { return diagonal; }
            Some(c) => { diagonal.push(get_char(c)); }
        }
    }
    diagonal
}

fn build_diagonal_from_top_to_left_bottom(array: &Array2D<char>, start_col: &usize) -> Vec<Chars> {
    let mut diagonal: Vec<Chars> = vec![];
    for row in 0..array.num_rows() {
        if row > *start_col {
            return diagonal;
        }
        let col = start_col - row;
        let item: Option<&char> = array.get(row, col);
        match item {
            None => { return diagonal; }
            Some(c) => { diagonal.push(get_char(c)); }
        }
    }
    diagonal
}

fn build_diagonal_from_bottom_to_top_right(array: &Array2D<char>, start_col: &usize) -> Vec<Chars> {
    let mut diagonal: Vec<Chars> = vec![];
    let mut col: usize = *start_col;
    for row in (0..array.num_rows()).rev() {
        let item: Option<&char> = array.get(row, col);
        match item {
            None => { return diagonal; }
            Some(c) => { diagonal.push(get_char(c)); }
        }
        col += 1;
    }
    diagonal
}

fn build_diagonal_from_bottom_to_top_left(array: &Array2D<char>, start_col: &usize) -> Vec<Chars> {
    let mut diagonal: Vec<Chars> = vec![];
    let mut col: usize = *start_col;
    for row in (0..array.num_rows()).rev() {
        let item: Option<&char> = array.get(row, col);
        match item {
            None => { return diagonal; }
            Some(c) => { diagonal.push(get_char(c)); }
        }
        if col == 0 {
            return diagonal;
        }
        col -= 1;
    }
    diagonal
}


fn count_xmas(chars: &Vec<&Chars>) -> i32 {
    let input: String = chars.iter().map(|c| to_char(c)).collect();
    let re = Regex::new(r"XMAS").unwrap();
    re.find_iter(&input).count() as i32
}

struct Window {
    top_left: Chars, top_right: Chars,
    center: Chars,
    bottom_left: Chars, bottom_right: Chars
}

fn _get_from_array(array: &Array2D<char>, row_index: usize, col_index: usize) -> Chars {
    get_char(array.get(row_index, col_index).unwrap())
}

fn build_window(array: &Array2D<char>, row_index: usize, col_index: usize) -> Window {
    Window {
        top_left: _get_from_array(array, row_index, col_index), top_right: _get_from_array(array, row_index, col_index+2),
        center: _get_from_array(array, row_index+1, col_index+1),
        bottom_left: _get_from_array(array, row_index+2, col_index), bottom_right: _get_from_array(array, row_index+2, col_index+2)
    }
}

fn is_valid_mas_cross(window: Window) -> bool {
    let pattern_1 = window.top_left == Chars::M && window.top_right == Chars::M && window.bottom_left == Chars::S && window.bottom_right == Chars::S;
    let pattern_2 = window.top_left == Chars::S && window.top_right == Chars::M && window.bottom_left == Chars::S && window.bottom_right == Chars::M;
    let pattern_3 = window.top_left == Chars::S && window.top_right == Chars::S && window.bottom_left == Chars::M && window.bottom_right == Chars::M;
    let pattern_4 = window.top_left == Chars::M && window.top_right == Chars::S && window.bottom_left == Chars::M && window.bottom_right == Chars::S;

    window.center == Chars::A
    && (pattern_1 || pattern_2 || pattern_3 || pattern_4)
}

impl Day {
    fn part_1(&self) -> i32 {
        let contents: String = read_contents(&self.input_path);
        let lines: Vec<Vec<char>> = contents.split("\n").map(|s| s.chars().collect()).collect();
        let array = Array2D::from_rows(&lines).unwrap();

        let mut sequences: Vec<Vec<Chars>> = vec![];
        // horizontal sequences
        for row_iter in array.rows_iter() {
            let row: Vec<Chars> = row_iter.map(get_char).collect();
            sequences.push(row);
        }
        // vertical sequences
        for col_iter in array.columns_iter() {
            let col: Vec<Chars> = col_iter.map(get_char).collect();
            sequences.push(col);
        }
        // diagonals
        for i in 0..array.num_columns() {
            let diagonal_to_right_bottom = build_diagonal_from_top_to_right_bottom(&array, &{ i });
            sequences.push(diagonal_to_right_bottom);
            let diagonal_to_left_bottom = build_diagonal_from_top_to_left_bottom(&array, &{ i });
            sequences.push(diagonal_to_left_bottom);
        }
        // Start from 1 to avoid counting the main diagonal twice
        for i in 1..array.num_columns() {
            let diagonal_to_top_right = build_diagonal_from_bottom_to_top_right(&array, &{ i });
            sequences.push(diagonal_to_top_right);
        }
        // End 1 before num columns to avoid counting the other main diagonal twice
        for i in 0..array.num_columns()-1 {
            let diagonal_to_top_left = build_diagonal_from_bottom_to_top_left(&array, &{ i });
            sequences.push(diagonal_to_top_left);
        }

        println!("Number of normal sequences: {}", sequences.len());

        let reversed_sequences: Vec<Vec<&Chars>> = sequences.iter()
            .map(|seq| seq.iter().rev().collect())
            .collect();

        println!("Number of reversed sequences: {}", reversed_sequences.len());

        let mut all_sequences: Vec<Vec<&Chars>> = vec![];
        reversed_sequences.iter().for_each(|rev_seq| {
            all_sequences.push(rev_seq.to_vec())
        });
        sequences.iter().for_each(|seq| {
            all_sequences.push(seq.iter().collect());
        });

        let x: Vec<&Vec<&Chars>> = all_sequences.iter().filter(|seq| seq.len() > 3).collect();
        println!("Number of all sequences: {}", x.len());

        all_sequences.iter()
            .filter(|seq| seq.len() > 3)
            .map(count_xmas)
            .sum()
    }
    
    fn part_2(&self) -> i32 {
        let contents: String = read_contents(&self.input_path);
        let lines: Vec<Vec<char>> = contents.split("\n").map(|s| s.chars().collect()).collect();
        let array: Array2D<char> = Array2D::from_rows(&lines).unwrap();
        let n_rows: usize = array.num_rows();
        let n_cols: usize = array.num_columns();

        let mut count_valid: i32 = 0;
        for row_index in 0..n_rows-2 {
            for col_index in 0..n_cols-2 {
                let window: Window = build_window(&array, row_index, col_index);
                if is_valid_mas_cross(window) {
                    count_valid += 1;
                }
            }
        }
        count_valid
    }
}

fn main() {
    let day_number = 5;
    let input_path: String = get_input_path(day_number);
    let day = Day {input_path};
    println!("Day {} part 1: {}", day_number, day.part_1());
    println!("Day {} part 2: {}", day_number, day.part_2());
}
