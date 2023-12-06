fn example_input() -> String {
    String::from_utf8_lossy(include_bytes!("day6_example.txt")).to_string()
}

fn parse(input: String) -> Vec<(u32, u32)> {
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
        .map(|n| n.parse::<u32>().unwrap());
    let distances = distance_line
        .next()
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse::<u32>().unwrap());
    Vec::from_iter(times.zip(distances))
}

fn possible_wins(race: &(u32, u32)) -> u32 {
    let (time, record) = *race;
    let mut wins = 0;
    for speed in 1..time {
        let time_left = time - speed;
        let distance = speed * time_left;
        if distance > record {
            wins += 1;
        } else if wins > 0 {
            break;
        }
    }
    wins
}

pub fn part1(input: Option<String>) -> u32 {
    parse(input.unwrap_or_else(example_input))
        .iter()
        .map(possible_wins)
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        assert_eq!(part1(None), 288);
    }
}
