use std::fs::read_to_string;
use std::collections::HashMap;
use aoc::utils::{Challenge, get_input_path};

struct Day1 {
    input_path: String
}

fn parse_lines(filename: &str) -> (Vec<i32>, Vec<i32>) {
    fn parse_line(line: String) -> (i32, i32) {
        let t: Vec<&str> = line.split("   ").collect();
        (t[0].parse::<i32>().unwrap(), t[1].parse::<i32>().unwrap())
    }
    let values: Vec<(i32, i32)> = read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .map(parse_line)
        .collect();

    let mut left: Vec<i32> = values.iter().map(|x| x.0).collect();
    let mut right: Vec<i32> = values.iter().map(|x| x.1).collect();
    
    left.sort();
    right.sort();

    (left, right)
}

impl Challenge for Day1 {
    fn part_1(&self) -> i32 {
        let (left, right) = parse_lines(&self.input_path);
        
        left.iter().zip(right.iter())
            .map(|(a, b)| (a - b).abs())
            .sum()
    }
    
    fn part_2(&self) -> i32 {
        let (left, right) = parse_lines(&self.input_path);
        let mut right_map: HashMap<i32, i32> = HashMap::new();
    
        let add_to_map = |i: &i32| -> () {
            match right_map.get(i) {
                None => right_map.insert(*i, 1),
                Some(count) => right_map.insert(*i, count+1)
            };
        };
        right.iter().for_each(add_to_map);
    
        left.iter().map(|i| match right_map.get(i) {
            None => 0,
            Some(count) => *count * i
        }).sum()
    }
}

fn main() {
    let input_path: String = get_input_path(1);
    let day = Day1 {input_path};
    day.run();
}
