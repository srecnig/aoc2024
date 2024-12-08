use std::fs;

use dec07::Equation;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input!");
    let mut equations: Vec<Equation> = Vec::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split(':').collect();
        let result: i64 = parts[0].trim().parse().expect("Failed to parse result");
        let numbers: Vec<i64> = parts[1]
            .split_whitespace()
            .map(|n| n.trim().parse().expect("Failed to parse number"))
            .collect();

        equations.push(Equation { result, numbers });
    }

    let valid_sum: i64 = equations
        .iter()
        .filter(|&e| e.check(false))
        .map(|e| e.result)
        .sum();
    println!("pt1: {:?}", valid_sum);

    let valid_sum: i64 = equations
        .iter()
        .filter(|&e| e.check(true))
        .map(|e| e.result)
        .sum();
    println!("pt2: {:?}", valid_sum);
}
