use core::fmt;

use array2d::{Array2D};

use aoc::utils::{Challenge, get_input_path, read_contents};

struct Day4 {
    input_path: String
}

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

impl Challenge for Day4 {
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
        // Starting from top row
        for i in 0..array.num_columns() {
            let diagonal_to_right_bottom = build_diagonal_from_top_to_right_bottom(&array, &(i as usize));
            sequences.push(diagonal_to_right_bottom);
            let diagonal_to_left_bottom = build_diagonal_from_top_to_left_bottom(&array, &(i as usize));
            sequences.push(diagonal_to_left_bottom);
        }
        41
    }
    
    fn part_2(&self) -> i32 {
        42
    }
}

fn main() {
    let input_path: String = get_input_path(4);
    let day = Day4 {input_path};
    day.run();
}
