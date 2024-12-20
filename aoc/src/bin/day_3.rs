use regex::Regex;

use aoc::utils::{get_input_path, read_contents};

struct Day {
    input_path: String
}


fn perform_mult(mult: &str) -> i32 {
    let re = Regex::new(r"[0-9]{1,3}").unwrap();
    let operands: Vec<&str> = re.find_iter(mult).map(|m| m.as_str()).collect();
    if operands.len() != 2_usize {
        panic!("Could not find two operands for detected mult: {mult}");
    } else {
        let left = operands[0].parse::<i32>().unwrap();
        let right = operands[1].parse::<i32>().unwrap();
        left * right
    }
}

struct Acc {
    should_multiply: bool,
    sum: i32
}

impl Day {
    fn part_1(&self) -> i32 {
        let contents = read_contents(&self.input_path);
    
        let re = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)").unwrap();
        let multiplications: Vec<&str> = re.find_iter(&contents).map(|m| m.as_str()).collect();
        multiplications.iter().map(|mult:&&str| perform_mult(mult)).sum()
    }
    
    
    fn part_2(&self) -> i32 {
        let contents = read_contents(&self.input_path);
    
        let re = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)|do\(\)|don't\(\)").unwrap();
        let matches: Vec<&str> = re.find_iter(&contents).map(|m| m.as_str()).collect();
        let initial_acc = Acc { should_multiply: true, sum: 0 };
    
        fn reduce(acc: Acc, elem: &str) -> Acc {
            if elem.eq("do()") {
                Acc { should_multiply: true, sum: acc.sum}
            } else if elem.eq("don't()") {
                Acc { should_multiply: false, sum: acc.sum}
            } else if acc.should_multiply {
                let product: i32 = perform_mult(elem);
                Acc { should_multiply: true, sum: acc.sum + product}
            } else {
                Acc { should_multiply: false, sum: acc.sum}
            }
        }
    
        matches.iter().fold(initial_acc, |acc: Acc, elem: &&str| reduce(acc, elem)).sum
    }
}

fn main() {
    let day_number = 3;
    let input_path: String = get_input_path(day_number);
    let day = Day {input_path};
    println!("Day {} part 1: {}", day_number, day.part_1());
    println!("Day {} part 2: {}", day_number, day.part_2());
}
