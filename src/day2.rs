use crate::file_reader::read_lines;

pub fn day2() {
    let lines = read_lines("input/day2.txt");

    let mut safe_reports = 0;
    let mut safe_reports_with_removal = 0;
    for line in lines {
        let report = split_levels(line);
        if check_safe(&report) {
            safe_reports += 1;
        } else if check_safe_with_removal(&report) {
            safe_reports_with_removal += 1;
        }
    }
    println!(
        "safe without removal: {}, safe with removal: {}, total: {}",
        safe_reports,
        safe_reports_with_removal,
        safe_reports + safe_reports_with_removal
    );
}

fn split_levels(line: String) -> Vec<i32> {
    line.split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}

fn check_safe(report: &Vec<i32>) -> bool {
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

fn check_safe_with_removal(report: &Vec<i32>) -> bool {
    for i in 0..report.len() {
        let mut temp_report = report.clone();
        temp_report.remove(i);
        if check_safe(&temp_report) {
            return true;
        }
    }
    false
}
