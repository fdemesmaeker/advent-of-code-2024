use std::fs::read_to_string;
use aoc::utils::{Challenge, get_input_path};

struct Day2 {
    input_path: String
}

fn parse_lines(filename: &str) -> Vec<Vec<i32>> {
    fn parse_line(line: String) -> Vec<i32> {
        line.split(" ").map(|n| n.parse::<i32>().unwrap()).collect()
    }
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .map(parse_line)
        .collect()
}

fn is_safe(levels: &Vec<i32>, tolerate_one_failed_report: bool) -> bool {
    _is_safe(levels, tolerate_one_failed_report, &0)
}

fn _is_safe(levels: &Vec<i32>, tolerate_one_failed_report: bool, current_index: &usize) -> bool {
    let levels_without_one: Vec<i32>;

    let levels_to_handle: &Vec<i32> = if tolerate_one_failed_report {
        levels_without_one = levels.iter().enumerate()
            .filter(|(pos, _)| pos != current_index)
            .map(|(_, elem)| *elem)
            .collect();
        &levels_without_one
    } else {
        levels
    };
    
    let n: usize = levels_to_handle.len();
    let levels_except_first_one: &[i32] = &levels_to_handle[1..n];
    let levels_except_last_one: &[i32] = &levels_to_handle[0..n-1];
    
    let differences: Vec<i32> = levels_except_last_one.iter().zip(levels_except_first_one.iter())
        .map(|(a,b)| a - b)
        .collect();
    
    let is_positive: bool = differences[0] > 0;
    let is_last_possible_toleration: bool = tolerate_one_failed_report && current_index + 1 == levels.len().try_into().unwrap();
    for diff in differences {
        if (diff > 0 && !is_positive) || (diff < 0 && is_positive) || diff.abs() < 1 || diff.abs() > 3 {
            if !tolerate_one_failed_report || is_last_possible_toleration {
                return false;
            }
            else {
                let new_index = current_index + (1 as usize);
                return _is_safe(levels, tolerate_one_failed_report, &new_index);
            }
        }
    }
    true
}


impl Challenge for Day2 {
    fn part_1(&self) -> i32 {
        let reports = parse_lines(&self.input_path);
        reports.iter().filter(|levels: &&Vec<i32>| is_safe(*levels, false)).count().try_into().unwrap()
    }
    
    fn part_2(&self) -> i32 {
        let reports = parse_lines(&self.input_path);
        reports.iter().filter(|levels: &&Vec<i32>| is_safe(*levels, true))
            .count().try_into().unwrap()
    }
}

fn main() {
    let input_path: String = get_input_path(2);
    let day = Day2 {input_path};
    day.run();
}

