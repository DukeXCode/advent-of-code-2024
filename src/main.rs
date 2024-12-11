use crate::day1::day1;
use crate::day2::day2;
use std::env;

mod day1;
mod day2;
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
        _ => println!("Illegal command"),
    }
}
