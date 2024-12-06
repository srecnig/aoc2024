use std::fs;

use dec05::{filter_relevant_rules, validate_update, Rule};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input!");
    let mut rules: Vec<Rule> = Vec::new();
    let mut updates: Vec<Vec<i32>> = Vec::new();

    for line in input.lines() {
        if line.contains('|') {
            let parts: Vec<&str> = line.split("|").collect();
            rules.push(Rule {
                front: parts[0].parse().unwrap(),
                back: parts[1].parse().unwrap(),
            })
        }
        if line.contains(',') {
            updates.push(line.split(',').map(|n| n.parse().unwrap()).collect());
        }
    }

    let valid_updates: &Vec<Vec<i32>> = &updates
        .into_iter()
        .filter(|update| {
            let relevant_rules = filter_relevant_rules(update, &rules);
            validate_update(update, &relevant_rules)
        })
        .collect();

    let middle_page_sum: i32 = valid_updates
        .iter()
        .map(|update| update[update.len() / 2])
        .sum();

    println!("{:?}", middle_page_sum);
}
