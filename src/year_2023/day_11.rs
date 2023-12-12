use std::collections::HashSet;

use itertools::Itertools;
use utils::geometry::Point;
use utils::input;
use utils::structure::Solution;

#[derive(Default)]
pub struct Day11 {
    grid: Vec<Vec<char>>,
    pairs: Vec<Pair>,
}

impl Day11 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Solution for Day11 {
    fn part1(&mut self) -> String {
        let result = self.pairs.iter().map(|p| p.expand(1)).sum::<usize>();
        format!("{}", result)
    }
    fn part2(&mut self) -> String {
        let result = self.pairs.iter().map(|p| p.expand(999999)).sum::<usize>();
        format!("{}", result)
    }
    fn parse(&mut self) {
        let data = input::to_string(2023, 11);

        for line in data.lines() {
            self.grid.push(line.chars().collect::<Vec<_>>());
        }

        let mut empty_rows: HashSet<usize> = HashSet::new();
        let mut empty_cols: HashSet<usize> = HashSet::new();
        let mut galaxies: HashSet<Point<usize>> = HashSet::new();

        for y in 0..self.grid.len() {
            if self.grid[y].iter().all(|ch| *ch == '.') {
                empty_rows.insert(y);
            }
        }

        for x in 0..self.grid[0].len() {
            let mut galaxy = false;
            for y in 0..self.grid.len() {
                if self.grid[y][x] == '#' {
                    galaxies.insert(Point { x, y });
                    galaxy = true;
                }
            }
            if !galaxy {
                empty_cols.insert(x);
            }
        }

        for combo in galaxies.iter().combinations(2) {
            let a = *combo[0];
            let b = *combo[1];
            let dist = a.distance(b);
            let mut rows = 0;
            let mut cols = 0;

            let start = a.y.min(b.y) + 1;
            let end = a.y.max(b.y);
            for row in start..end {
                if empty_rows.contains(&row) {
                    rows += 1;
                }
            }

            let start = a.x.min(b.x) + 1;
            let end = a.x.max(b.x);
            for col in start..end {
                if empty_cols.contains(&col) {
                    cols += 1;
                }
            }

            self.pairs.push(Pair { dist, rows, cols });
        }
    }
}

struct Pair {
    dist: usize,
    rows: usize,
    cols: usize,
}

impl Pair {
    fn expand(&self, light_years: usize) -> usize {
        self.dist + (self.rows + self.cols) * light_years
    }
}
