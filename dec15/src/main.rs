use std::fs;

use dec15::{Movement, Warehouse};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input!");
    let input_parts: Vec<&str> = input.split("\n\n").collect();
    let warehouse_input = input_parts[0];
    let movements_input_raw = input_parts[1];
    let movements_input = movements_input_raw.replace('\n', "");

    let mut warehouse = Warehouse::new(warehouse_input.lines().collect());
    let mut movements: Vec<Movement> = Vec::new();
    for ch in movements_input.chars() {
        movements.push(Movement::new(ch));
    }

    for movement in movements {
        warehouse.move_robot(&movement);
    }

    println!("pt1: {}", warehouse.gps_sum());
}
