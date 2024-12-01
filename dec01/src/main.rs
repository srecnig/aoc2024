use std::fs;

use dec01::{difference_vector, similarity_score, sum_up_vector};

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

    // part 1
    let diff_vector = difference_vector(&left, &right);
    let diff_sum = sum_up_vector(&diff_vector);
    println!("part 1: {:?}", diff_sum);

    // part 2
    let mut similarity_vector: Vec<i32> = Vec::new();
    for number in &left {
        similarity_vector.push(similarity_score(*number, &right))
    }
    let similarity_sum = sum_up_vector(&similarity_vector);
    println!("part 2: {:?}", similarity_sum)
}
