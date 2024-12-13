use std::fs;

use dec13::{build_machine, calculate_price, win_prize, win_prize_efficiently, ClawMachine};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input!");
    let machine_input: Vec<&str> = input.split("\n\n").collect();

    let machines: Vec<ClawMachine> = machine_input
        .iter()
        .map(|&entry| {
            let machine_lines: Vec<&str> = entry.lines().collect();
            build_machine(machine_lines, false)
        })
        .collect();

    let price: i128 = machines
        .iter()
        .filter_map(win_prize_efficiently)
        .map(|(a, b)| calculate_price(a, b))
        .sum();

    println!("pt1-slow: {}", price);

    let machines: Vec<ClawMachine> = machine_input
        .iter()
        .map(|&entry| {
            let machine_lines: Vec<&str> = entry.lines().collect();
            build_machine(machine_lines, false)
        })
        .collect();

    let price: i128 = machines
        .iter()
        .filter_map(win_prize)
        .map(|(a, b)| calculate_price(a, b))
        .sum();

    println!("pt1-fast: {}", price);

    let machines: Vec<ClawMachine> = machine_input
        .iter()
        .map(|&entry| {
            let machine_lines: Vec<&str> = entry.lines().collect();
            build_machine(machine_lines, true)
        })
        .collect();

    let price: i128 = machines
        .iter()
        .filter_map(win_prize_efficiently)
        .map(|(a, b)| calculate_price(a, b))
        .sum();

    println!("pt2: {}", price);
}
