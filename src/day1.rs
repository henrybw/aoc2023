fn example_input() -> String {
    String::from_utf8_lossy(include_bytes!("day1_example.txt")).to_string()
}

pub fn part1(input: Option<String>) -> u32 {
    let input = input.unwrap_or_else(example_input);
    let mut sum = 0;
    for line in input.lines() {
        let digits = line
            .chars()
            .filter(char::is_ascii_digit)
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<u32>>();
        sum += digits.first().unwrap() * 10 + digits.last().unwrap();
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
}
