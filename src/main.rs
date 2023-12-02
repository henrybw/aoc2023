mod day0;
mod day1;

use std::env;
use std::fs::File;
use std::io::{self, Read};

fn read_from_filename_or_stdin(filename: String) -> String {
    let mut input = String::new();
    if filename == "-" {
        io::stdin().read_to_string(&mut input).unwrap();
    } else {
        let mut file = File::open(filename).unwrap();
        file.read_to_string(&mut input).unwrap();
    }
    input
}

fn usage() {
    println!("Advent of Code 2023");
    println!(
        "usage: {} [--help|-h] [day]N.P [input]",
        env::args().nth(0).unwrap()
    );
    println!();
    println!("N: Which day's puzzle to solve (1..25)");
    println!("P: Which part of the puzzle to solve (either 1 or 2)");
    println!(concat!(
        "input: Path to an input file; if omitted, use ",
        "example input for that day's puzzle"
    ));
}

fn main() {
    if let Some(arg) = env::args().nth(1) {
        if arg == "--help" || arg == "-h" {
            usage();
            std::process::exit(0);
        }
    }
    let puzzle = env::args()
        .nth(1)
        .expect("needs a puzzle to run, e.g. `day1.1` or `1.1`");
    let input = env::args().nth(2).map(read_from_filename_or_stdin);
    let output = match &puzzle[..] {
        "day0.1" | "0.1" => day0::part1(input) as i64,
        "day0.2" | "0.2" => day0::part2(input) as i64,
        "day1.1" | "1.1" => day1::part1(input) as i64,
        "day1.2" | "1.2" => day1::part2(input) as i64,
        _ => panic!("unknown puzzle: {}", puzzle),
    };
    println!("{}", output);
}
