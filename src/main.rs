use crate::day1::day1;
use crate::day2::day2;
use crate::day3::day3;
use crate::day4::day4;
use std::env;

mod day1;
mod day2;
mod day3;
mod day4;
mod file_reader;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Illegal number of args");
        return;
    }
    match args[1].as_str() {
        "1" => day1(),
        "2" => day2(),
        "3" => day3(),
        "4" => day4(),
        _ => println!("Illegal command"),
    }
}
