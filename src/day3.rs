use std::collections::HashSet;

fn example_input() -> String {
    String::from_utf8_lossy(include_bytes!("day3_example.txt")).to_string()
}

#[derive(Debug)]
struct Grid {
    height: usize,
    width: usize,
    cells: Vec<char>,
}

impl Grid {
    fn new(input: String) -> Self {
        let lines: Vec<_> = input.lines().collect();
        Self {
            height: lines.len(),
            width: lines[0].len(),
            cells: lines.iter().map(|&line| line.chars()).flatten().collect(),
        }
    }

    fn get(&self, x: usize, y: usize) -> char {
        self.cells[y * self.height + x]
    }

    fn extract_number(&self, x: usize, y: usize) -> (u32, usize, usize) {
        let mut start_x = x as isize;
        while start_x >= 0 {
            let c = self.get(start_x as usize, y);
            if !c.is_ascii_digit() {
                break;
            }
            start_x -= 1;
        }
        start_x += 1;

        let mut end_x = start_x + 1;
        while end_x < self.width as isize {
            let c = self.get(end_x as usize, y);
            if !c.is_ascii_digit() {
                break;
            }
            end_x += 1;
        }
        let start = y as usize * self.height + start_x as usize;
        let end = y as usize * self.height + end_x as usize;
        let mut num = 0;
        for c in &self.cells[start..end] {
            num = 10 * num + c.to_digit(10).unwrap();
        }
        (num, start_x as usize, end_x as usize)
    }
}

pub fn part1(input: Option<String>) -> u32 {
    let grid = Grid::new(input.unwrap_or_else(example_input));
    let mut digits_seen: HashSet<(usize, usize)> = HashSet::new();
    let mut sum = 0;
    for y in 0..grid.height {
        for x in 0..grid.width {
            let c = grid.get(x, y);
            if c == '.' || c.is_ascii_digit() {
                continue;
            }

            // this is a symbol: scan its neighbors for numbers
            for dy in -1..=1 {
                let ny = y as isize + dy;
                if ny < 0 || ny >= grid.height as isize {
                    continue;
                }
                for dx in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let nx = x as isize + dx;
                    if nx < 0 || nx >= grid.width as isize {
                        continue;
                    }
                    if !grid.get(nx as usize, ny as usize).is_ascii_digit()
                        || digits_seen.contains(&(nx as usize, ny as usize))
                    {
                        continue;
                    }

                    // this is a number we haven't seen before
                    let (num, start_x, end_x) =
                        grid.extract_number(nx as usize, ny as usize);
                    for sx in start_x..end_x {
                        digits_seen.insert((sx, ny as usize));
                    }
                    sum += num
                }
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        assert_eq!(part1(None), 4361);
    }
}
