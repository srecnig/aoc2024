use std::fs;

use dec11::blink_efficiently;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input!");
    let normalized_input = input.replace('\n', "");

    let stones: Vec<i64> = normalized_input
        .split(" ")
        .map(|n| n.parse().unwrap())
        .collect();

    let stone_count = blink_efficiently(&stones, 25);
    println!("pt1: {:?}", &stone_count);

    let stone_count = blink_efficiently(&stones, 75);
    println!("pt2: {:?}", &stone_count);
}
