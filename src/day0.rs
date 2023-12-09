//! Dummy module for setting up the project scaffolding. Not an actual puzzle.

use std::str::Lines;

fn solve_part1(entries: &[u64]) -> u64 {
    entries[0]
}

fn solve_part2(entries: &[u64]) -> u64 {
    entries[1]
}

fn parse_lines(lines: Lines) -> Vec<u64> {
    lines
        .map(|line| line.parse().unwrap())
        .collect::<Vec<u64>>()
}

fn example_input() -> Vec<u64> {
    let example =
        String::from_utf8_lossy(include_bytes!("day0_example.txt")).to_string();
    parse_lines(example.lines())
}

fn parse_input(input: Option<String>) -> Vec<u64> {
    if let Some(contents) = input {
        parse_lines(contents.lines())
    } else {
        example_input()
    }
}

pub fn part1(input: Option<String>) -> u64 {
    solve_part1(&parse_input(input))
}

pub fn part2(input: Option<String>) -> u64 {
    solve_part2(&parse_input(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        assert_eq!(solve_part1(&example_input()), 69);
    }

    #[test]
    fn example_part2() {
        assert_eq!(solve_part2(&example_input()), 420);
    }
}
