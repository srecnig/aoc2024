use std::fs;

use dec05::do_something;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input!");
    for line in input.lines() {
        println!("{:?}", line);
    }
    do_something();
}
