use std::{collections::{HashMap, HashSet}, ops::Index, vec};

use aoc::utils::{Challenge, get_input_path, read_contents, vec_to_string};

struct Day5 {
    input_path: String
}

type RuleMap=HashMap<i32, Vec<Rule>>;
struct Rule {
    before: i32,
    after: i32
}


fn parse_lines(input_path: &str) -> (RuleMap, Vec<Vec<i32>>) {
    let contents: String = read_contents(input_path);
    let mut rules: RuleMap = HashMap::new();
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


fn ___check_rules(remainder: &[i32], rules: &Vec<Rule>) -> Vec<i32> {
    let befores: HashSet<i32> = rules.iter().map(|rule| rule.before).collect();
    let incorrect_befores: Vec<i32> = remainder.iter()
        .map(|page_number| befores.get(page_number))
        .filter(|before| before.is_some())
        .map(|before| *before.unwrap())
        .collect();
    incorrect_befores
}

fn check_rules(remainder: &[i32], rules: &Vec<Rule>) -> i32 {
    let befores: HashSet<i32> = rules.iter().map(|rule| rule.before).collect();
    let first_incorrect_before = remainder.iter().filter(|page_number| befores.contains(&page_number)).next();
    *first_incorrect_before.unwrap_or(&-1)
}

fn get_incorrect_before(update: &Vec<i32>, rules: &RuleMap) -> (i32, i32) {
    let update_length: usize = update.len();
    for page_number_index in 0..update_length-1 {
        let page_number: i32 = update[page_number_index];
        let remainder: &[i32] = &update[page_number_index..update_length];
        let page_number_rules = rules.get(&page_number);
        if page_number_rules.is_some() {
            let incorrect_before = check_rules(remainder, page_number_rules.unwrap());
            if incorrect_before != -1 {
                return (page_number, incorrect_before);
            }
        }
    }
    (-1, -1)
}

fn get_middle_element(update: &Vec<i32>) -> i32 {
    update[update.len() / 2]
}

fn fix_unordered(update: &Vec<i32>, rules: &RuleMap) -> Vec<i32> {
    let mut new_update: Vec<i32> = update.to_vec();
    let (mut page_number, mut incorrect_before) = get_incorrect_before(update, rules);
    while incorrect_before != -1 {
        let page_number_index = new_update.iter().position(|x| *x == page_number).unwrap();
        let before_index = new_update.iter().position(|x| *x == incorrect_before).unwrap();
        new_update.swap(page_number_index, before_index);
        (page_number, incorrect_before) = get_incorrect_before(&new_update, rules);
    }

    let new_update = new_update;
    new_update
}

impl Challenge for Day5 {
    fn part_1(&self) -> i32 {
        let (rules, updates) = parse_lines(&self.input_path);
        let middle_elements: Vec<i32> = updates.iter()
            .filter(|update| get_incorrect_before(update, &rules).0 == -1)
            .map(get_middle_element)
            .collect();
        middle_elements.iter().sum()
    }

    fn part_2(&self) -> i32 {
        let (rules, updates) = parse_lines(&self.input_path);
        let fixed_unordered: Vec<Vec<i32>> = updates.iter()
            .filter(|update| get_incorrect_before(update, &rules).0 != -1)
            .map(|update| fix_unordered(update, &rules))
            .collect();
        let middle_elements: Vec<i32> = fixed_unordered.iter()
            .map(get_middle_element)
            .collect();
        middle_elements.iter().sum()
    }
}

fn main() {
    let input_path: String = get_input_path(5);
    let day = Day5 {input_path};
    day.run();
}
