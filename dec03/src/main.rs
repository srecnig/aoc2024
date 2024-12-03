use std::fs;

use dec03::{detect_multiplications, multiply};

fn main() {
    let mut multiplications: Vec<&str> = Vec::new();

    let input = fs::read_to_string("input1.txt").expect("Failed to read input!");
    for line in input.lines() {
        multiplications.extend(detect_multiplications(line));
    }
    let multiplication_result: i32 = multiplications.iter().map(|&m| multiply(m)).sum();

    println!("part1: {:?}", multiplication_result);
}
