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

    let safe_report_dampened_count = reports
        .iter()
        .filter(|&report| {
            if is_report_safe(report) {
                true
            } else {
                for i in 0..report.len() {
                    let mut dampened_report = report.clone();
                    dampened_report.remove(i);
                    if is_report_safe(&dampened_report) {
                        return true;
                    }
                }
                false
            }
        })
        .count();

    println!("part2: {:?}", safe_report_dampened_count);
}
