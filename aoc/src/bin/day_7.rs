use std::fmt;
use std::hash::Hash;
use itertools::Itertools;
use std::collections::HashMap;

use aoc::utils::{get_input_path, read_contents};

struct Day {
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
    MULT,
    CONCAT
}

type OperandsCache=HashMap<usize, Vec<Vec<Operand>>>;

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Operand::ADD => write!(f, "+"),
            Operand::MULT => write!(f, "*"),
            Operand::CONCAT => write!(f, "||")
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

/*
    From https://users.rust-lang.org/t/enumerate-permutations-with-repetitions-for-lengths-1-to-n/64962
*/
fn perms_iter<'a, T: Copy>(
	input: &'a [T],
	max_len: u32,
) -> impl Iterator<Item = impl Iterator<Item = T> + 'a> {
	(1..=max_len)
		.flat_map(move |len| (0..input.len().pow(len)).zip(std::iter::repeat(len)))
		.map(move |(mut n, j)| {
			(0..j).map(move |_| {
				let s = input[n % input.len()];
				n /= input.len();
				s
			})
		})
}

fn instantiate_operands(n_operands: usize) -> Vec<Vec<Operand>> {
    let distinct_operands = vec![Operand::ADD, Operand::MULT, Operand::CONCAT];
    
    perms_iter(&distinct_operands, n_operands as u32)
        .map(|perm| perm.collect())
        .filter(|perm: &Vec<Operand>| perm.len() == n_operands)
        .collect()
}

fn compute(a: &i64, b: &i64, op:&Operand) -> i64 {
    match op {
        Operand::ADD => {
            a + b
        }
        Operand::MULT => {
            a * b
        }
        Operand::CONCAT => {
            (a.to_string() + &b.to_string()).parse().unwrap()
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
    if result == eq.result {
        println!("Result is {} while eq result is {}", result, eq.result);
        println!("Proof");
        let mut proof_result = first_term;
        for (term, op) in rest_of_terms.iter().zip(operands.iter()) {
            let next_result = compute(&proof_result, term, op);
            println!("Performing {} {} {} lead to {}", proof_result, op.to_string(), term, next_result);
            proof_result = next_result;
        }
    }
    result == eq.result
}

fn can_be_filled_with_operands(eq: &Equation, all_operands: &Vec<Vec<Operand>>) -> Option<Vec<Operand>> {
    for operands in all_operands {
        if check_equation_with_operands(&eq, operands) {
            return Some(operands.to_vec());
        }
    }
    None
}

fn format_op_and_eq(eq: &Equation, operands: &Vec<Operand>) -> String {
    let first_term = eq.terms[0];
    let rest_of_terms = &eq.terms[1..eq.terms.len()];
    let mut result: String = vec![eq.result.to_string(), first_term.to_string()].join(" = ");
    for (term, op) in rest_of_terms.iter().zip(operands.iter()) {
        result = vec![result, op.to_string(), term.to_string()].join(" ");
    }
    result
}

fn check_equation(eq: &Equation, cache: &mut OperandsCache) -> bool {
    let n_operands = eq.terms.len() - 1;
    if !cache.contains_key(&n_operands) {
        let all_operands = instantiate_operands(n_operands);
        cache.insert(n_operands, all_operands);
    }
    let ops = can_be_filled_with_operands(eq, cache.get(&n_operands).unwrap());
    if ops.is_some() {
        println!("Found equation solution: {}", format_op_and_eq(eq, &ops.unwrap()));
        true
    } else {
        println!("Unsolvable equation: {}", eq);
        false
    }
}

impl Day {
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
    let input_path: String = get_input_path(7);//"data/day_7/input_thomas.txt".to_string();
    let day = Day {input_path};
    day.part_1();
    day.part_2();
}
