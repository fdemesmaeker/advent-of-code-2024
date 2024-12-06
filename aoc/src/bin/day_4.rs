use array2d::{Array2D};

use aoc::utils::{Challenge, get_input_path, read_contents};

struct Day4 {
    input_path: String
}

impl Challenge for Day4 {
    fn part_1(&self) -> i32 {
        let contents: String = read_contents(&self.input_path);
        let lines: Vec<Vec<char>> = contents.split("\n").map(|s| s.chars().collect()).collect();
        let array = Array2D::from_rows(&lines).unwrap();
        for row_iter in array.rows_iter() {
            for element in row_iter {
                print!("{} ", element);
            }
            println!();
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
