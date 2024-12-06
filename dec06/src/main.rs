use std::fs;

use dec06::{build_map, Map};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input!");

    let mut map: Map = build_map(input.lines().collect());
    let mut inside: bool = true;
    while inside {
        inside = map.move_player();
    }
    println!("part1: {}", map.visited_count());
}
