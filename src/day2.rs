use std::collections::HashMap;

fn example_input() -> String {
    String::from_utf8_lossy(include_bytes!("day2_example.txt")).to_string()
}

pub fn part1(input: Option<String>) -> u32 {
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
                let count: u32 = count_color[0].parse().unwrap();
                let color = count_color[1];
                if count > max_cubes[color] {
                    continue 'next;
                }
            }
        }
        let id: u32 = game[0]["Game ".len()..].parse().unwrap();
        sum += id;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        assert_eq!(part1(None), 8);
    }
}
