use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let lines = read_lines("day1.txt");

    let mut pairs = Vec::new();
    for line in lines {
        pairs.push(extract_pair(line));
    }

    let mut first_list = Vec::new();
    let mut second_list = Vec::new();
    for pair in pairs {
        first_list.push(pair.0);
        second_list.push(pair.1);
    }

    first_list.sort();
    second_list.sort();

    let mut diff = 0;
    for i in 0..first_list.len() {
        diff += (first_list[i] - second_list[i]).abs();
    }
    println!("{}", diff);
}

fn extract_pair(line: String) -> (i32, i32) {
    let nums = line.split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    (nums[0], nums[1])
}

fn read_lines(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("Failed to open file");
    let reader = BufReader::new(file);
    reader.lines().map(|l| l.expect("Failed to read line")).collect()
}
