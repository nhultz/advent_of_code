use aoc_derive::{aoc, aoc_input};
use std::collections::{HashMap, VecDeque};

#[aoc_input(day11)]
fn parse(input: &str) -> Result<Vec<Vec<u32>>, String> {
    input
        .lines()
        .map(|l| {
            let nums = l
                .chars()
                .map(|c| c.to_digit(10).ok_or(format!("unable to parse")))
                .collect();
            nums
        })
        .collect()
}

#[aoc(day11, part1)]
fn part1(input: &[Vec<u32>]) -> u64 {
    let mut monitor = Monitor::new(input);

    for _ in 0..100 {
        monitor.step();
    }

    monitor.flash_count
}

#[aoc(day11, part2)]
fn part2(input: &[Vec<u32>]) -> u32 {
    let mut monitor = Monitor::new(input);

    let mut step = 0;

    loop {
        step += 1;
        monitor.step();
        if monitor.all_flashed() {
            break;
        }
    }

    step
}

type Coord = (usize, usize);

#[derive(Debug)]
enum Action {
    Increment(Coord),
    Flash(Coord),
}

#[derive(Debug)]
struct Monitor {
    energy_levels: Vec<Vec<u32>>,
    neighbor_lookup: HashMap<Coord, Vec<Coord>>,
    rows: usize,
    cols: usize,
    flash_count: u64,
    visited: Vec<Vec<bool>>,
    queue: VecDeque<Action>,
}

impl Monitor {
    fn new(starting_levels: &[Vec<u32>]) -> Self {
        let num_rows = starting_levels.len();
        let num_cols = starting_levels[0].len();

        let mut neighbors = HashMap::new();
        for i in 0..num_rows {
            for j in 0..num_cols {
                let n = get_neighbors(i, j, num_rows, num_cols);
                neighbors.insert((i, j), n);
            }
        }

        Self {
            energy_levels: starting_levels.to_vec(),
            neighbor_lookup: neighbors,
            rows: num_rows,
            cols: num_cols,
            visited: vec![vec![false; num_cols]; num_rows],
            queue: VecDeque::new(),
            flash_count: 0,
        }
    }

    fn all_flashed(&self) -> bool {
        self.energy_levels
            .iter()
            .all(|row| row.iter().all(|v| *v == 0))
    }

    fn reset(&mut self) {
        for i in 0..self.rows {
            for j in 0..self.cols {
                self.visited[i][j] = false;
            }
        }
        self.queue.clear();
    }

    fn step(&mut self) {
        self.reset();

        for i in 0..self.rows {
            for j in 0..self.cols {
                self.queue.push_back(Action::Increment((i, j)));
            }
        }

        while let Some(action) = self.queue.pop_front() {
            match action {
                Action::Increment(coord) => {
                    let (x, y) = coord;

                    if self.visited[x][y] {
                        continue;
                    }

                    self.energy_levels[x][y] += 1;

                    if self.energy_levels[x][y] > 9 {
                        self.queue.push_back(Action::Flash(coord));
                    }
                }
                Action::Flash(coord) => {
                    let (x, y) = coord;

                    if self.visited[x][y] {
                        continue;
                    }

                    self.energy_levels[x][y] = 0;
                    self.flash_count += 1;

                    if let Some(neighbors) = self.neighbor_lookup.get(&coord) {
                        for n in neighbors {
                            self.queue.push_back(Action::Increment(*n));
                        }
                    }

                    self.visited[x][y] = true;
                }
            }
        }
    }
}

fn get_neighbors(x: usize, y: usize, max_row: usize, max_col: usize) -> Vec<Coord> {
    let mut neighbors = vec![];

    if x > 0 {
        neighbors.push((x - 1, y));
        if y > 0 {
            neighbors.push((x - 1, y - 1));
        }
        if y < (max_col - 1) {
            neighbors.push((x - 1, y + 1));
        }
    }

    if x < (max_row - 1) {
        neighbors.push((x + 1, y));
        if y > 0 {
            neighbors.push((x + 1, y - 1));
        }
        if y < (max_col - 1) {
            neighbors.push((x + 1, y + 1));
        }
    }

    if y > 0 {
        neighbors.push((x, y - 1));
    }

    if y < (max_col - 1) {
        neighbors.push((x, y + 1));
    }

    neighbors
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    #[test]
    fn test_part1() {
        let entries = parse(INPUT).unwrap();
        assert_eq!(part1(&entries), 1656);
    }

    #[test]
    fn test_part2() {
        let entries = parse(INPUT).unwrap();
        assert_eq!(part2(&entries), 195);
    }
}
