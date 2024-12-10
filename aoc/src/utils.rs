use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn get_path(day_number: i8, is_test: bool) -> String {
    let input_file: &str = if is_test { "test.txt" } else { "input.txt" };
    "data/day_".to_string() + &day_number.to_string() + "/" + input_file
}

pub fn get_input_path(day_number: i8) -> String {
    let args: Vec<String> = env::args().collect();
    let is_test = args.len() > 1_usize && args[1].eq("test");
    get_path(day_number, is_test)
}

pub fn vec_to_string(vec: &Vec<i32>) -> String {
    let vec_of_str: Vec<String> = vec.iter().map(|i| i.to_string()).collect();
    vec_of_str.join(",")
}

pub trait Challenge {
    fn part_1(&self) -> i32;
    fn part_2(&self) -> i32;
    fn run(&self) {
        let solution_1: i32 = self.part_1();
        println!("Solution part 1: {}", solution_1);
    
        let solution_2: i32 = self.part_2();
        println!("Solution part 2: {}", solution_2);
    }
}

pub fn read_contents(input_path: &str) -> String {
    let path = Path::new(input_path);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(path) {
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
