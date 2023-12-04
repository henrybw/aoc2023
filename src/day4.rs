use itertools::Itertools;
use std::collections::HashSet;

fn example_input() -> String {
    String::from_utf8_lossy(include_bytes!("day4_example.txt")).to_string()
}

pub fn part1(input: Option<String>) -> u32 {
    let mut sum = 0;
    for line in input.unwrap_or_else(example_input).lines() {
        let card = line.split(": ").nth(1).unwrap();
        let (winning, have) = card
            .split(" | ")
            .map(|numbers| {
                HashSet::<&str>::from_iter(numbers.split_whitespace())
            })
            .collect_tuple()
            .unwrap();
        let won: HashSet<_> = winning.intersection(&have).collect();
        if won.is_empty() {
            continue;
        }
        sum += 2_u32.pow(won.len() as u32 - 1);
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        assert_eq!(part1(None), 13);
    }
}
