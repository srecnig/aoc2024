use std::fs;

use dec02::is_report_safe;

fn main() {
    let mut reports: Vec<Vec<i32>> = Vec::new();

    let input = fs::read_to_string("input1.txt").expect("Failed to read input!");

    for line in input.lines() {
        let levels: Vec<i32> = line
            .split(" ")
            .map(|n| n.parse().expect("Not a valid integer!"))
            .collect();
        reports.push(levels);
    }

    let safe_report_count = reports
        .iter()
        .filter(|&report| is_report_safe(report))
        .count();

    println!("part1: {:?}", safe_report_count);
}
