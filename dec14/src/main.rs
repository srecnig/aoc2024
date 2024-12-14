use std::fs;
use std::io::{stdin, Read};

use dec14::{count_by_quadrant, robot_display, Map, Robot};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input!");
    let mut robots: Vec<Robot> = input.lines().map(Robot::new).collect();
    //let map_input1 = Map::new(11, 7);
    let map_input = Map::new(101, 103);

    for i in 0..200000 {
        for robot in &mut robots {
            robot.travel(&map_input);
        }

        if i == 99 {
            let (tl, tr, bl, br) = count_by_quadrant(&robots, &map_input);
            println!("-------");
            println!("| pt1: {}", tl * tr * bl * br);
            println!("-------");
        }

        let display = &robot_display(&robots, &map_input);
        let empty_lines = empty_line_count(display);
        if empty_lines > 5 {
            println!("After second: {}, empty lines: {}", i + 1, empty_lines);
            print_display(display);
            stdin().read_exact(&mut [0]).unwrap();
        }
    }
}

fn print_display(display: &Vec<Vec<char>>) {
    for row in display {
        for &ch in row {
            print!("{}", ch);
        }
        println!(); // Move to the next eline after printing each row
    }
}

fn empty_line_count(display: &Vec<Vec<char>>) -> u32 {
    display
        .iter()
        .filter(|row| row.iter().all(|&ch| ch == ' '))
        .count() as u32
}
