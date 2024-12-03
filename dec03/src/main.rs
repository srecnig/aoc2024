use std::fs;

use dec03::{apply_conditionals, detect_multiplications, multiply};

fn main() {
    let input = fs::read_to_string("input1.txt").expect("Failed to read input!");
    let normalized_input = input.replace('\n', "");

    let multiplications = detect_multiplications(&normalized_input);
    let multiplication_result: i32 = multiplications.iter().map(|m| multiply(m)).sum();
    println!("part1: {:?}", multiplication_result);

    let conditional_input = apply_conditionals(&normalized_input);
    let conditional_multiplications = detect_multiplications(&conditional_input);
    let multiplication_result: i32 = conditional_multiplications
        .iter()
        .map(|m| multiply(m))
        .sum();
    println!("part2: {:?}", multiplication_result);
}
