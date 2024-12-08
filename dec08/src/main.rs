use std::fs;

use dec08::AntennaMap;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input!");
    let map = AntennaMap::new(input.lines().collect());
    let antinodes = map.antinodes();
    print!("pt1: {}", antinodes.len());
}
