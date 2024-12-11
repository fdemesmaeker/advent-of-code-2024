use std::fmt;
use std::hash::Hash;
use std::iter;
use itertools::Itertools;
use std::collections::HashMap;

use aoc::utils::{get_input_path, read_contents};

struct Day7 {
    input_path: String
}

struct Equation {
    result: i64,
    terms: Vec<i64>
}

impl fmt::Display for Equation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}= {}) ", self.result, self.terms.iter().join(","))
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
enum Operand {
    ADD,
    MULT
}

type OperandsCache=HashMap<usize, Vec<Vec<Operand>>>;

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Operand::ADD => write!(f, "ADD"),
            Operand::MULT => write!(f, "MULT")
        }
    }
}

fn parse_equation(s: &str) -> Equation {
    let (result, terms) = s.split_once(": ").unwrap();
    let terms_in_int: Vec<i64> = terms.split(" ").map(|x: &str| x.parse().unwrap()).collect();
    Equation {
        result: result.parse().unwrap(),
        terms: terms_in_int
    }
}

fn parse_lines(input_path: &str) -> Vec<Equation> {
    let contents: String = read_contents(input_path);
    contents.lines()
        .map(parse_equation)
        .collect()
}

fn get_permutations(operands: &[Operand]) -> Vec<Vec<Operand>> { 
    operands.iter().permutations(operands.len()).unique()// permutation
        .map(|combi: Vec<&Operand>| combi.iter().map(|c: &&Operand| **c).collect()).collect() // only to instantiate value instead of ref
}

fn instantiate_operands(n_operands: usize) -> Vec<Vec<Operand>> {
    let mut all_operands: Vec<Vec<Operand>> = vec![];

    for n_additions in 0..n_operands+1 {
        let operands: Vec<Operand> = iter::repeat_n(Operand::ADD, n_additions as usize)
            .chain(iter::repeat_n(Operand::MULT, (n_operands - n_additions) as usize))
            .collect();
        all_operands.extend_from_slice(&get_permutations(&operands));
    }
    all_operands
}

fn compute(a: &i64, b: &i64, op:&Operand) -> i64 {
    match op {
        Operand::ADD => {
            a + b
        }
        Operand::MULT => {
            a * b
        }
    }
}

fn check_equation_with_operands(eq: &Equation, operands: &Vec<Operand>) -> bool {
    let first_term = eq.terms[0];
    let rest_of_terms = &eq.terms[1..eq.terms.len()];
    let mut result = first_term;
    for (term, op) in rest_of_terms.iter().zip(operands.iter()) {
        result = compute(&result, term, op);
    }
    result == eq.result
}

fn can_be_filled_with_operands(eq: &Equation, all_operands: &Vec<Vec<Operand>>) -> bool {
    for operands in all_operands {
        if check_equation_with_operands(&eq, operands) {
            return true;
        }
    }
    false
}

fn check_equation(eq: &Equation, cache: &mut OperandsCache) -> bool {
    println!("Checking equation {}", eq);
    let n_operands = eq.terms.len() - 1;
    if !cache.contains_key(&n_operands) {
        let all_operands = instantiate_operands(n_operands);
        cache.insert(n_operands, all_operands);
    }
    can_be_filled_with_operands(eq, cache.get(&n_operands).unwrap())
}

impl Day7 {
    fn part_1(&self) -> () {
        let equations = parse_lines(&self.input_path);
        let mut cache: OperandsCache = HashMap::new();
        let sum: i64 = equations.iter()
            .filter(|eq| check_equation(eq, &mut cache))
            .map(|eq| eq.result)
            .sum();
        println!("{}", sum)
    }

    fn part_2(&self) -> () {
    }
}

fn main() {
    let input_path: String = get_input_path(7);
    let day = Day7 {input_path};
    day.part_1();
    day.part_2();
}
