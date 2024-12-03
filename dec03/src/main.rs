use std::fs;

use dec03::{apply_conditionals, detect_multiplications, multiply};

fn main() {
    let input = fs::read_to_string("input1.txt").expect("Failed to read input!");

    let mut multiplications: Vec<String> = Vec::new();
    for line in input.lines() {
        multiplications.extend(detect_multiplications(line));
    }
    let multiplication_result: i32 = multiplications.iter().map(|m| multiply(m)).sum();
    println!("part1: {:?}", multiplication_result);

    let mut conditional_multiplications: Vec<String> = Vec::new();
    for line in input.lines() {
        let conditional_line = apply_conditionals(line);
        conditional_multiplications.extend(detect_multiplications(&conditional_line));
    }
    let multiplication_result: i32 = conditional_multiplications
        .iter()
        .map(|m| multiply(m))
        .sum();
    println!("part2: {:?}", multiplication_result);
}
