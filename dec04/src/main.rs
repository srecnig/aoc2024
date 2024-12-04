use std::fs;

use dec04::{count_xmas, detect_x_mas_at, extract_lines};

fn main() {
    let input = fs::read_to_string("input1.txt").expect("Failed to read input!");
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        grid.push(line.chars().collect());
    }

    let extracted_lines = extract_lines(&grid);
    let xmas_count: i32 = extracted_lines.into_iter().map(|l| count_xmas(&l)).sum();
    println!("part1: {:?}", xmas_count);

    let mut x_mas_count = 0;
    for x in 0..(grid[0].len() - 1) {
        for y in 0..(grid.len() - 1) {
            if detect_x_mas_at(&grid, x, y) {
                x_mas_count += 1;
            }
        }
    }
    println!("part2: {:?}", x_mas_count);
}
