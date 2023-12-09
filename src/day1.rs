use std::collections::HashMap;

fn part1_example_input() -> String {
    String::from_utf8_lossy(include_bytes!("day1_part1_example.txt"))
        .to_string()
}

fn part2_example_input() -> String {
    String::from_utf8_lossy(include_bytes!("day1_part2_example.txt"))
        .to_string()
}

pub fn part1(input: Option<String>) -> u64 {
    let input = input.unwrap_or_else(part1_example_input);
    let mut sum = 0;
    for line in input.lines() {
        let digits = line
            .chars()
            .filter(char::is_ascii_digit)
            .map(|c| c.to_digit(10).unwrap() as u64)
            .collect::<Vec<_>>();
        sum += digits.first().unwrap() * 10 + digits.last().unwrap();
    }
    sum
}

fn digit_from_string(s: &str) -> u64 {
    let spellings = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    spellings[s]
}

pub fn part2(input: Option<String>) -> u64 {
    let prefixes = HashMap::from([
        ('o', vec!["one"]),
        ('t', vec!["two", "three"]),
        ('f', vec!["four", "five"]),
        ('s', vec!["six", "seven"]),
        ('e', vec!["eight"]),
        ('n', vec!["nine"]),
    ]);
    let suffixes = HashMap::from([
        ('e', vec!["one", "three", "five", "nine"]),
        ('n', vec!["seven"]),
        ('o', vec!["two"]),
        ('r', vec!["four"]),
        ('t', vec!["eight"]),
        ('x', vec!["six"]),
    ]);

    let input = input.unwrap_or_else(part2_example_input);
    let mut sum = 0;
    for line in input.lines() {
        let mut first = 0;
        'outer: for (i, c) in line.char_indices() {
            if c.is_ascii_digit() {
                first = c.to_digit(10).unwrap() as u64;
                break 'outer;
            }
            if prefixes.contains_key(&c) {
                for &digit in &prefixes[&c] {
                    let j = i + digit.len();
                    if j <= line.len() {
                        let substr = &line[i..j];
                        if substr == digit {
                            first = digit_from_string(substr);
                            break 'outer;
                        }
                    }
                }
            }
        }

        let mut last = 0;
        'outer: for (i, c) in line.char_indices().rev() {
            if c.is_ascii_digit() {
                last = c.to_digit(10).unwrap() as u64;
                break 'outer;
            }
            if suffixes.contains_key(&c) {
                for &digit in &suffixes[&c] {
                    let j = i as isize - digit.len() as isize + 1;
                    if j >= 0 {
                        let substr = &line[j as usize..i + 1];
                        if substr == digit {
                            last = digit_from_string(substr);
                            break 'outer;
                        }
                    }
                }
            }
        }

        sum += first * 10 + last;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        assert_eq!(part1(None), 142);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(None), 281);
    }
}
