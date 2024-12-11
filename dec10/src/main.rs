use std::fs;

use dec10::Map;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input!");
    let map = Map::new(&input);
    println!("pt1: {}", map.score());

    let map = Map::new(&input);
    println!("pt2: {}", map.rating());
}
