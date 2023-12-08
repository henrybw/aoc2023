use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

fn example_input() -> String {
    String::from_utf8_lossy(include_bytes!("day8_example.txt")).to_string()
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
                let re = Regex::new(r"\(([A-Z]{3}), ([A-Z]{3})\)").unwrap();
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

    fn aaa_to_zzz(&self) -> usize {
        let mut node = "AAA";
        for (i, &instr) in self.instrs.iter().cycle().enumerate() {
            node = self.next(node, instr);
            if node == "ZZZ" {
                return i + 1;
            }
        }
        unreachable!("no ZZZ node found");
    }
}

pub fn part1(input: Option<String>) -> u32 {
    let network = Network::new(input.unwrap_or_else(example_input));
    network.aaa_to_zzz() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        assert_eq!(part1(None), 6);
    }
}
