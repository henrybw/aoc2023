use std::collections::HashMap;

fn example_input() -> String {
    String::from_utf8_lossy(include_bytes!("day2_example.txt")).to_string()
}

pub fn part1(input: Option<String>) -> u64 {
    let input = input.unwrap_or_else(example_input);
    let max_cubes = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    let mut sum = 0;

    'next: for line in input.lines() {
        let game: Vec<_> = line.split(": ").collect();
        let sets: Vec<_> = game[1].split("; ").collect();
        for set in sets {
            let cubes_by_color: Vec<_> = set.split(", ").collect();
            for cubes in cubes_by_color {
                let count_color: Vec<_> = cubes.split_whitespace().collect();
                let count: u64 = count_color[0].parse().unwrap();
                let color = count_color[1];
                if count > max_cubes[color] {
                    continue 'next;
                }
            }
        }
        let id: u64 = game[0]["Game ".len()..].parse().unwrap();
        sum += id;
    }
    sum
}

pub fn part2(input: Option<String>) -> u64 {
    let input = input.unwrap_or_else(example_input);
    let mut power_sum = 0;

    for line in input.lines() {
        let game: Vec<_> = line.split(": ").collect();
        let sets: Vec<_> = game[1].split("; ").collect();
        let mut min_set =
            HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);
        for set in sets {
            let cubes_by_color: Vec<_> = set.split(", ").collect();
            for cubes in cubes_by_color {
                let count_color: Vec<_> = cubes.split_whitespace().collect();
                let count: u64 = count_color[0].parse().unwrap();
                let color = count_color[1];
                if count > min_set[color] {
                    min_set.insert(color, count);
                }
            }
        }
        power_sum += min_set.values().product::<u64>();
    }
    power_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        assert_eq!(part1(None), 8);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(None), 2286);
    }
}
