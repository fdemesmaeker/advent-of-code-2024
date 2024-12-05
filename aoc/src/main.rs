use std::env;

use day::Challenge;

mod day;

mod day_1;
mod day_2;
mod day_3;

use day_1::Day1;
use day_2::Day2;
use day_3::Day3;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = if args.len() > (0 as usize) && args[0].eq("--test") {
        "test.txt"
    } else {
        "input.txt"
    };

    let d1 = Day1 {filename: ["data/day_1", input_file].join("/")};
    let d2 = Day2 {filename: ["data/day_2", input_file].join("/")};
    let d3 = Day3 {filename: ["data/day_3", input_file].join("/")};

    let challenges: Vec<&dyn Challenge> = vec![
        &d1,
        &d2,
        &d3
    ];

    challenges.iter().for_each(|c: &&dyn Challenge| c.run());
}
