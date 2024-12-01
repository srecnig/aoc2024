use std::fs;

use dec01::{difference_vector, sum_up_vector};

fn main() {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    let input = fs::read_to_string("input1.txt").expect("Failed to read input!");
    for line in input.lines() {
        let numbers: Vec<i32> = line
            .split("   ")
            .map(|n| n.parse().expect("Not a valid integer!"))
            .collect();

        left.push(numbers[0]);
        right.push(numbers[1]);
    }

    let diff_vector = difference_vector(&left, &right);
    let diff_sum = sum_up_vector(&diff_vector);
    println!("{:?}", diff_sum)
}
