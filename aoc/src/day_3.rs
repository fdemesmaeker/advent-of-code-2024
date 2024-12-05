use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use regex::Regex;

fn read_contents(filename: &str) -> String {
    let path = Path::new(filename);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => s
    }
}

fn perform_mult(mult: &str) -> i32 {
    let re = Regex::new(r"[0-9]{1,3}").unwrap();
    let operands: Vec<&str> = re.find_iter(&mult).map(|m| m.as_str()).collect();
    if operands.len() != (2 as usize) {
        panic!("Could not find two operands for detected mult: {mult}");
    } else {
        let left = operands[0].parse::<i32>().unwrap();
        let right = operands[1].parse::<i32>().unwrap();
        left * right
    }
}

fn part_1(filename: &str) -> i32 {
    let contents = read_contents(filename);

    let re = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)").unwrap();
    let multiplications: Vec<&str> = re.find_iter(&contents).map(|m| m.as_str()).collect();
    multiplications.iter().map(|mult:&&str| perform_mult(*mult)).sum()
}

struct Acc {
    should_multiply: bool,
    sum: i32
}



fn part_2(filename: &str) -> i32 {
    let contents = read_contents(filename);

    let re = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)|do\(\)|don't\(\)").unwrap();
    let matches: Vec<&str> = re.find_iter(&contents).map(|m| m.as_str()).collect();
    let initial_acc = Acc { should_multiply: true, sum: 0 };

    fn reduce(acc: Acc, elem: &str) -> Acc {
        if elem.eq("do()") {
            Acc { should_multiply: true, sum: acc.sum}
        } else if elem.eq("don't()") {
            Acc { should_multiply: false, sum: acc.sum}
        } else {
            if acc.should_multiply {
                let product: i32 = perform_mult(elem);
                Acc { should_multiply: true, sum: acc.sum + product}
            } else {
                Acc { should_multiply: false, sum: acc.sum}
            }
        }
    }

    matches.iter().fold(initial_acc, |acc: Acc, elem: &&str| reduce(acc, elem)).sum
}

pub fn main(filename: &str) {
    let solution_1 = part_1(filename);
    println!("Solution Day 3 part 1: {}", solution_1);

    let solution_2 = part_2(filename);
    println!("Solution Day 3 part 2: {}", solution_2);
}
