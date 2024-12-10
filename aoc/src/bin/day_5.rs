use std::{collections::{HashMap, HashSet}, vec};

use aoc::utils::{Challenge, get_input_path, read_contents, vec_to_string};

struct Day5 {
    input_path: String
}

// TODO: Use type RuleMap=HashMap<i32, Vec<usize>> ?
struct Rule {
    before: i32,
    after: i32
}


fn parse_lines(input_path: &str) -> (HashMap<i32, Vec<Rule>>, Vec<Vec<i32>>) {
    let contents: String = read_contents(input_path);
    let mut rules: HashMap<i32, Vec<Rule>> = HashMap::new();
    let mut updates: Vec<Vec<i32>> = vec![];
    for line in contents.split("\n") {
        if line.contains("|") {
            let parts: Vec<&str> = line.split("|").collect();
            let before: i32 = parts[0].parse().unwrap();
            let after: i32 = parts[1].parse().unwrap();
            let new_rule: Rule = Rule {before, after};

            if let Some(current_rules) = rules.get_mut(&after) {
                current_rules.push(new_rule);
            } 
            else {
                let mut initial_rules: Vec<Rule> = Vec::new();
                    initial_rules.push(new_rule);
                    rules.insert(after, initial_rules);
            }
        } else if !line.is_empty() {
            let update: Vec<i32> = line.split(",").map(|s: &str| s.parse::<i32>().unwrap()).collect();
            updates.push(update);
        }
    }
    (rules, updates)
}

/* 
    hashmap 
    key: value after "|" symbol
    value: set of rules with key after "|" symbol
*/
fn check_rules(remainder: &[i32], rules: &Vec<Rule>) -> bool {
    let befores: HashSet<i32> = rules.iter().map(|rule| rule.before).collect();
    !remainder.iter().any(|page_number| befores.contains(page_number))
}

fn check_update(update: &Vec<i32>, rules: &HashMap<i32, Vec<Rule>>) -> bool {
    let update_length: usize = update.len();
    for page_number_index in 0..update_length-1 {
        let page_number: i32 = update[page_number_index];
        let remainder: &[i32] = &update[page_number_index..update_length];
        let page_number_rules = rules.get(&page_number);
        if page_number_rules.is_some() && !check_rules(remainder, page_number_rules.unwrap()) {
            return false;
        }
    }
    true
}

fn get_middle_element(update: &Vec<i32>) -> i32 {
    update[update.len() / 2]
}

impl Challenge for Day5 {
    fn part_1(&self) -> i32 {
        let (rules, updates) = parse_lines(&self.input_path);
        let middle_elements: Vec<i32> = updates.iter()
            .filter(|update| check_update(update, &rules))
            .map(get_middle_element)
            .collect();
        middle_elements.iter().sum()
    }
    
    fn part_2(&self) -> i32 {
        42
    }
}

fn main() {
    let input_path: String = get_input_path(5);
    let day = Day5 {input_path};
    day.run();
}
