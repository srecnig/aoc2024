use std::fs;

use dec11::blink;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input!");
    let normalized_input = input.replace('\n', "");

    let mut stones: Vec<i64> = normalized_input
        .split(" ")
        .map(|n| n.parse().unwrap())
        .collect();

    for _ in 0..25 {
        stones = blink(stones);
    }
    println!("pt1: {:?}", stones.len());

    for i in 0..50 {
        stones = blink(stones);
        println!("{}", i);
    }
    println!("pt2: {:?}", stones.len());
}
