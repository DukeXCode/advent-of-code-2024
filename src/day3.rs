use crate::file_reader::read_string;
use regex::{Captures, Regex};

pub fn day3() {
    let input = read_string("input/day3.txt");
    println!(
        "Sum total: {}, Sum enabled: {}",
        process_instructions(&input),
        process_enabled_instructions(&input)
    );
}

fn process_instructions(input: &String) -> i32 {
    let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut result = 0;
    for cap in regex.captures_iter(input) {
        result += multiply_from_cap(cap);
    }
    result
}

fn multiply_from_cap(cap: Captures) -> i32 {
    let num1 = cap[1].parse::<i32>().unwrap();
    let num2 = cap[2].parse::<i32>().unwrap();
    num1 * num2
}

fn process_enabled_instructions(input: &String) -> i32 {
    let regex = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
    let mut result = 0;
    let mut enabled = true;
    for cap in regex.captures_iter(input) {
        match cap[0].to_string().as_str() {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ if enabled => result += multiply_from_cap(cap),
            _ => {}
        }
    }
    result
}
