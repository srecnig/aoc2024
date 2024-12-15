use std::fs;

use dec15::{Movement, Tile, Warehouse};

fn main() {
    let input = fs::read_to_string("input1.txt").expect("Failed to read input!");
    let input_parts: Vec<&str> = input.split("\n\n").collect();
    let warehouse_input = input_parts[0];
    let movements_input_raw = input_parts[1];
    let movements_input = movements_input_raw.replace('\n', "");

    let mut warehouse = Warehouse::new(warehouse_input.lines().collect());
    let mut movements: Vec<Movement> = Vec::new();
    for ch in movements_input.chars() {
        movements.push(Movement::new(ch));
    }
    // println!("******* inital **********");
    // println!("{:?}", warehouse.tiles);

    // warehouse.move_robot(&movements[0]);
    // println!("{:?}", warehouse.tiles);

    // warehouse.move_robot(&movements[1]);
    // println!("{:?}", warehouse.tiles);

    // warehouse.move_robot(&movements[2]);
    // println!("{:?}", warehouse.tiles);

    // println!("******* {:?} **********", &movements[3]);
    // warehouse.move_robot(&movements[3]);
    // println!("{:?}", warehouse.tiles);

    // println!("******* 4 {:?} **********", &movements[4]);
    // warehouse.move_robot(&movements[4]);
    // println!("{:?}", warehouse.tiles);

    for (step, movement) in movements.iter().enumerate() {
        println!();
        println!("step {} - went {:?}", step, movement);
        warehouse.move_robot(&movement);
        print_display(&warehouse.tiles);
    }

    // println!("{:?}", warehouse.tiles);
    // println!("{:?}", warehouse);
    // println!("********");
    // println!("{:?}", movements);

    // for line in input.lines() {
    //     println!("{:?}", line);
    // }
    // do_something();
}

fn print_display(display: &Vec<Vec<Tile>>) {
    for row in display {
        for tile in row {
            print!("{}", tile.on_tile);
        }
        println!(); // Move to the next eline after printing each row
    }
}

// let machine_input: Vec<&str> = input.split("\n\n").collect();
