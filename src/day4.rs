use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn example_input() -> String {
    String::from_utf8_lossy(include_bytes!("day4_example.txt")).to_string()
}

fn winning_numbers(line: &str) -> HashSet<&str> {
    let card = line.split(": ").nth(1).unwrap();
    let (winning, have) = card
        .split(" | ")
        .map(|numbers| HashSet::<&str>::from_iter(numbers.split_whitespace()))
        .collect_tuple()
        .unwrap();
    winning.intersection(&have).cloned().collect()
}

pub fn part1(input: Option<String>) -> u64 {
    let mut sum = 0;
    for line in input.unwrap_or_else(example_input).lines() {
        let won = winning_numbers(line);
        if won.is_empty() {
            continue;
        }
        sum += 2_u64.pow(won.len() as u32 - 1);
    }
    sum
}

pub fn part2(input: Option<String>) -> u64 {
    let mut copies_by_card = HashMap::new();
    for (i, line) in input.unwrap_or_else(example_input).lines().enumerate() {
        let count = copies_by_card.get(&i).unwrap_or(&0);
        copies_by_card.insert(i, count + 1);

        let won = winning_numbers(line);
        if won.is_empty() {
            continue;
        }
        let copies = copies_by_card[&i];
        for j in i + 1..=i + won.len() {
            let count = copies_by_card.get(&j).unwrap_or(&0);
            copies_by_card.insert(j, count + copies);
        }
    }
    copies_by_card.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        assert_eq!(part1(None), 13);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(None), 30);
    }
}
