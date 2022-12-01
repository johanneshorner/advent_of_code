mod day1;

use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let mut args = std::env::args();
    args.next(); // skip exectuable name

    let day = args.next().unwrap();
    let input_file_path = args.next().unwrap();

    let run_fn: fn(&[&str]) = match day.as_str() {
        "day1" => day1::run,
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
