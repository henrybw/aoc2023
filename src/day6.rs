fn example_input() -> String {
    String::from_utf8_lossy(include_bytes!("day6_example.txt")).to_string()
}

fn parse_part1(input: String) -> Vec<(u64, u64)> {
    let lines: Vec<_> = input.lines().collect();
    assert_eq!(lines.len(), 2);

    let mut time_line = lines[0].split(":");
    assert_eq!(time_line.next(), Some("Time"));
    let mut distance_line = lines[1].split(":");
    assert_eq!(distance_line.next(), Some("Distance"));

    let times = time_line
        .next()
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse().unwrap());
    let distances = distance_line
        .next()
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse().unwrap());
    Vec::from_iter(times.zip(distances))
}

fn possible_wins(race: &(u64, u64)) -> u64 {
    fn distance(time: u64, speed: u64) -> u64 {
        speed * (time - speed)
    }

    let (time, record) = *race;
    let mut lo = 0;
    for speed in 1..time {
        if distance(time, speed) > record {
            lo = speed;
            break;
        }
    }
    let mut hi = 0;
    for speed in (1..time).rev() {
        if distance(time, speed) > record {
            hi = speed;
            break;
        }
    }
    hi - lo + 1
}

pub fn part1(input: Option<String>) -> u64 {
    parse_part1(input.unwrap_or_else(example_input))
        .iter()
        .map(possible_wins)
        .product()
}

fn parse_part2(input: String) -> (u64, u64) {
    let lines: Vec<_> = input.lines().collect();
    assert_eq!(lines.len(), 2);

    let mut time_line = lines[0].split(":");
    assert_eq!(time_line.next(), Some("Time"));
    let mut distance_line = lines[1].split(":");
    assert_eq!(distance_line.next(), Some("Distance"));

    let time = time_line.next().unwrap().replace(" ", "").parse().unwrap();
    let distance = distance_line
        .next()
        .unwrap()
        .replace(" ", "")
        .parse()
        .unwrap();
    (time, distance)
}

pub fn part2(input: Option<String>) -> u64 {
    possible_wins(&parse_part2(input.unwrap_or_else(example_input)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        assert_eq!(part1(None), 288);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(None), 71503);
    }
}
