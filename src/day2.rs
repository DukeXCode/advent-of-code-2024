use crate::file_reader::read_lines;

pub fn day2() {
    let lines = read_lines("day2.txt");

    let mut safe_reports = 0;
    for line in lines {
        let report = split_levels(line);
        if check_safe(report) {
            safe_reports += 1;
        }
    }
    println!("safe reports: {}", safe_reports);
}

fn split_levels(line: String) -> Vec<i32> {
    line.split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}

fn check_safe(report: Vec<i32>) -> bool {
    (check_increasing(&report) || check_decreasing(&report)) && check_differences(&report)
}

fn check_increasing(report: &Vec<i32>) -> bool {
    let mut sorted: Vec<i32> = report.iter().cloned().collect();
    sorted.sort();
    report == &sorted
}

fn check_decreasing(report: &Vec<i32>) -> bool {
    let mut sorted: Vec<i32> = report.iter().cloned().collect();
    sorted.sort_by(|a, b| b.cmp(a));
    report == &sorted
}

fn check_differences(report: &Vec<i32>) -> bool {
    let mut last_level = report[0];
    for i in 1..report.len() {
        if (last_level - report[i]).abs() < 1 || (last_level - report[i]).abs() > 3 {
            return false;
        }
        last_level = report[i];
    }
    true
}
