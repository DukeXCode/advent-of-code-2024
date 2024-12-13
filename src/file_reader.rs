use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_lines(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("Failed to open file");
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|l| l.expect("Failed to read line"))
        .collect()
}

pub fn read_string(filename: &str) -> String {
    fs::read_to_string(&filename).expect("Failed to read file")
}
