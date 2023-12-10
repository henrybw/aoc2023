use itertools::Itertools;
use std::collections::HashSet;

type Point = (isize, isize);

fn north(x: isize, y: isize) -> Point {
    (x, y - 1)
}

fn south(x: isize, y: isize) -> Point {
    (x, y + 1)
}

fn east(x: isize, y: isize) -> Point {
    (x + 1, y)
}

fn west(x: isize, y: isize) -> Point {
    (x - 1, y)
}

#[derive(Debug)]
struct Grid {
    tiles: Vec<char>,
    width: usize,
    height: usize,
    start: Point,
    start_conns: Vec<Point>,
}

impl Grid {
    fn new(input: &String) -> Self {
        let lines: Vec<_> = input.lines().collect();
        let height = lines.len();
        let width = lines[0].len();
        let mut start = None;
        let tiles = lines
            .iter()
            .enumerate()
            .map(|(y, &line)| {
                line.char_indices().for_each(|(x, c)| {
                    if c == 'S' {
                        start = Some((x as isize, y as isize));
                    }
                });
                line.chars()
            })
            .flatten()
            .collect();
        let start = start.expect("no starting S tile found");

        let mut grid = Self {
            height,
            width,
            tiles,
            start,
            start_conns: vec![],
        };
        grid.infer_start_conns();
        grid
    }

    fn get(&self, x: isize, y: isize) -> char {
        self.tiles[y as usize * self.height + x as usize]
    }

    fn adjacent(&self, x: isize, y: isize) -> Vec<Point> {
        vec![north(0, 0), south(0, 0), east(0, 0), west(0, 0)]
            .iter()
            .filter_map(|(dx, dy)| {
                let nx = x + dx;
                let ny = y + dy;
                if nx >= 0
                    && nx < self.width as isize
                    && ny >= 0
                    && ny < self.height as isize
                {
                    Some((nx, ny))
                } else {
                    None
                }
            })
            .collect_vec()
    }

    fn connections(&self, x: isize, y: isize) -> Option<Vec<Point>> {
        match self.get(x, y) {
            '|' => Some(vec![north(x, y), south(x, y)]),
            '-' => Some(vec![east(x, y), west(x, y)]),
            'L' => Some(vec![north(x, y), east(x, y)]),
            'J' => Some(vec![north(x, y), west(x, y)]),
            '7' => Some(vec![south(x, y), west(x, y)]),
            'F' => Some(vec![south(x, y), east(x, y)]),
            _ => None,
        }
    }

    fn infer_start_conns(&mut self) {
        if self.start_conns.len() == 2 {
            return;
        }
        assert_eq!(self.start_conns.len(), 0);
        for (nx, ny) in self.adjacent(self.start.0, self.start.1) {
            if let Some(conns) = self.connections(nx, ny) {
                if conns
                    .iter()
                    .any(|(x, y)| *x == self.start.0 && *y == self.start.1)
                {
                    self.start_conns.push((nx, ny));
                }
            }
        }
        assert_eq!(self.start_conns.len(), 2);
    }
}

fn example_input() -> String {
    String::from_utf8_lossy(include_bytes!("day10_example.txt")).to_string()
}

pub fn part1(input: Option<String>) -> u64 {
    let grid = Grid::new(&input.unwrap_or_else(example_input));
    let mut visited = HashSet::from([grid.start]);
    let mut frontier = grid.start_conns.clone();
    let mut cur = Vec::new();
    let mut dist = 0;
    while !frontier.is_empty() {
        dist += 1;
        cur.clear();
        cur.extend(frontier.drain(..));
        for tile in &cur {
            visited.insert(*tile);
            if let Some(conns) = grid.connections(tile.0, tile.1) {
                for conn in &conns {
                    if !visited.contains(conn) {
                        frontier.push(*conn);
                    }
                }
            }
        }
    }
    dist
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        assert_eq!(part1(None), 8);
    }
}
