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

fn derive_sequences(history: &Vec<i64>) -> Vec<Vec<i64>> {
    let mut sequences = Vec::new();
    let mut next_seq = history.clone();
    while next_seq.iter().any(|&v| v != 0) {
        sequences.push(next_seq.clone());
        next_seq = next_seq
            .iter()
            .zip(next_seq.iter().skip(1))
            .map(|(v1, v2)| v2 - v1)
            .collect_vec();
    }
    sequences
}

fn extrapolate(history: Vec<i64>) -> i64 {
    derive_sequences(&history)
        .iter()
        // addition is commutative, so no need to reverse iterate from bottom up
        .filter_map(|seq| seq.iter().last())
        .sum()
}

fn extrapolate_backward(history: Vec<i64>) -> i64 {
    derive_sequences(&history)
        .iter()
        .rev()
        .filter_map(|seq| seq.iter().next())
        .fold(0, |acc, v| v - acc)
}

fn extrapolate_sum(
    input: Option<String>,
    extrapolate_fn: fn(Vec<i64>) -> i64,
) -> i64 {
    input
        .unwrap_or_else(example_input)
        .lines()
        .map(parse_history)
        .map(extrapolate_fn)
        .sum()
}

pub fn part1(input: Option<String>) -> u64 {
    let sum = extrapolate_sum(input, extrapolate);
    assert!(sum >= 0, "negative sum {sum}");
    sum as u64
}

pub fn part2(input: Option<String>) -> u64 {
    let sum = extrapolate_sum(input, extrapolate_backward);
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

    #[test]
    fn example_part2() {
        assert_eq!(part2(None), 2);
    }
}
