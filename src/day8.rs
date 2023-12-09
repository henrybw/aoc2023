use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

fn part1_example_input() -> String {
    String::from_utf8_lossy(include_bytes!("day8_part1_example.txt"))
        .to_string()
}

fn part2_example_input() -> String {
    String::from_utf8_lossy(include_bytes!("day8_part2_example.txt"))
        .to_string()
}

fn gcd(a: u64, b: u64) -> u64 {
    // the euclidean algorithm
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(nums: &[u64]) -> u64 {
    assert!(!nums.is_empty());
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd(a, b)
}

struct Network {
    instrs: Vec<char>,
    nodes: HashMap<String, (String, String)>,
}

impl Network {
    fn new(input: String) -> Self {
        let mut lines = input.lines();
        let instrs = lines.next().unwrap().chars().collect_vec();
        let mut nodes: HashMap<String, (String, String)> = HashMap::new();
        lines
            .skip(1)
            .map(|line| {
                let node = line.split(" = ").collect_vec();
                let re =
                    Regex::new(r"\(([A-Z1-9]{3}), ([A-Z1-9]{3})\)").unwrap();
                for (_, [left, right]) in
                    re.captures_iter(node[1]).map(|c| c.extract())
                {
                    nodes.insert(
                        node[0].to_string(),
                        (left.to_string(), right.to_string()),
                    );
                }
            })
            .collect_vec();
        Self { instrs, nodes }
    }

    fn next(&self, node: &str, instr: char) -> &str {
        match instr {
            'L' => &self.nodes[node].0,
            'R' => &self.nodes[node].1,
            _ => panic!("unknown instruction {instr}"),
        }
    }

    fn aaa_to_zzz(&self) -> u64 {
        let mut node = "AAA";
        for (i, &instr) in self.instrs.iter().cycle().enumerate() {
            node = self.next(node, instr);
            if node == "ZZZ" {
                return (i + 1) as u64;
            }
        }
        unreachable!("no ZZZ node found");
    }

    fn a_to_z(&self) -> u64 {
        let mut current = self
            .nodes
            .keys()
            .filter(|node| node.ends_with('A'))
            .map(String::from)
            .collect_vec();
        let mut z_steps = vec![0; current.len()];
        for (i, &instr) in self.instrs.iter().cycle().enumerate() {
            for node in current.iter_mut() {
                *node = self.next(node, instr).to_string();
            }
            for (j, _) in current
                .iter()
                .enumerate()
                .filter(|(_, node)| node.ends_with('Z'))
            {
                if z_steps[j] == 0 {
                    z_steps[j] = (i + 1) as u64;
                }
            }
            if z_steps.iter().all(|&steps| steps > 0) {
                break;
            }
        }
        lcm(&z_steps)
    }
}

pub fn part1(input: Option<String>) -> u64 {
    let network = Network::new(input.unwrap_or_else(part1_example_input));
    network.aaa_to_zzz()
}

pub fn part2(input: Option<String>) -> u64 {
    let network = Network::new(input.unwrap_or_else(part2_example_input));
    network.a_to_z()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        // from https://en.wikipedia.org/wiki/Greatest_common_divisor
        assert_eq!(gcd(48, 18), 6);
        assert_eq!(gcd(54, 24), 6);
        assert_eq!(gcd(48, 180), 12);
    }

    #[test]
    fn test_lcm() {
        // from https://en.wikipedia.org/wiki/Least_common_multiple
        assert_eq!(lcm(&[4, 6]), 12);
        assert_eq!(lcm(&[21, 6]), 42);
        assert_eq!(lcm(&[8, 9, 21]), 504);
        assert_eq!(lcm(&[48, 180]), 720);
    }

    #[test]
    fn example_part1() {
        assert_eq!(part1(None), 6);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(None), 6);
    }
}
