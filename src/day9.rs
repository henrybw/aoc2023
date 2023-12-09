use itertools::Itertools;

fn example_input() -> String {
    String::from_utf8_lossy(include_bytes!("day9_example.txt")).to_string()
}

fn parse_history(input: &str) -> Vec<i64> {
    input
        .split_whitespace()
        .filter_map(|v| v.parse().ok())
        .collect_vec()
}

fn extrapolate(history: Vec<i64>) -> i64 {
    let mut sequences = Vec::new();
    let mut next_seq = history;
    while next_seq.iter().any(|&v| v != 0) {
        sequences.push(next_seq.clone());
        next_seq = next_seq
            .iter()
            .zip(next_seq.iter().skip(1))
            .map(|(v1, v2)| v2 - v1)
            .collect_vec();
    }
    sequences.iter().filter_map(|seq| seq.iter().last()).sum()
}

pub fn part1(input: Option<String>) -> u64 {
    let sum: i64 = input
        .unwrap_or_else(example_input)
        .lines()
        .map(parse_history)
        .map(extrapolate)
        .sum();
    assert!(sum >= 0, "negative sum {sum}");
    sum as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        assert_eq!(part1(None), 114);
    }
}
