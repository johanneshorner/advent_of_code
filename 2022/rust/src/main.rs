mod day1;
mod day10;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let mut args = std::env::args();
    args.next(); // skip exectuable name

    let day = args.next().unwrap();
    let input_file_path = args.next().unwrap();

    let run_fn: fn(&[&str]) = match day.as_str() {
        "day1" => day1::run,
        "day2" => day2::run,
        "day3" => day3::run,
        "day4" => day4::run,
        "day5" => day5::run,
        "day6" => day6::run,
        "day7" => day7::run,
        "day8" => day8::run,
        "day9" => day9::run,
        "day10" => day10::run,
        _ => panic!("Day not found!"),
    };

    let input_lines: Vec<String> = {
        let file = File::open(input_file_path).unwrap();
        io::BufReader::new(file)
            .lines()
            .map(|line| line.unwrap())
            .collect()
    };

    run_fn(
        &input_lines
            .iter()
            .map(String::as_str)
            .collect::<Vec<&str>>(),
    );
}
