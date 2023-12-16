use std::collections::{HashMap, HashSet};

use utils::geometry::{Direction, Point};
use utils::Solution;

impl Problem {
    pub fn new() -> Box<dyn Solution> {
        Box::new(Self::default())
    }

    fn energize(&self, start: (Point<usize>, Direction)) -> usize {
        let mut rays = vec![start];
        let mut visited: HashSet<(Point<usize>, Direction)> = HashSet::new();

        while rays.len() > 0 {
            let last = rays.len() - 1;
            match rays[last].1 {
                Direction::Up => rays[last].0.y -= 1,
                Direction::Down => rays[last].0.y += 1,
                Direction::Left => rays[last].0.x -= 1,
                Direction::Right => rays[last].0.x += 1,
            }

            let (p, d) = rays[last];

            if p.y >= self.y_max || p.x >= self.x_max {
                rays.pop();
                continue;
            }

            if !visited.insert((p, d)) {
                rays.pop();
                continue;
            }

            if let Some(m) = self.mirrors.get(&p) {
                match m {
                    Mirror::Vertical => {
                        if d == Direction::Right || d == Direction::Left {
                            rays[last].1 = Direction::Up;
                            rays.push((p, Direction::Down));
                        }
                    }
                    Mirror::Horizontal => {
                        if d == Direction::Up || d == Direction::Down {
                            rays[last].1 = Direction::Left;
                            rays.push((p, Direction::Right));
                        }
                    }
                    Mirror::Right => match d {
                        Direction::Up => {
                            rays[last].1 = Direction::Right;
                        }
                        Direction::Down => {
                            rays[last].1 = Direction::Left;
                        }
                        Direction::Right => {
                            rays[last].1 = Direction::Up;
                        }
                        Direction::Left => {
                            rays[last].1 = Direction::Down;
                        }
                    },
                    Mirror::Left => match d {
                        Direction::Up => {
                            rays[last].1 = Direction::Left;
                        }
                        Direction::Down => {
                            rays[last].1 = Direction::Right;
                        }
                        Direction::Right => {
                            rays[last].1 = Direction::Down;
                        }
                        Direction::Left => {
                            rays[last].1 = Direction::Up;
                        }
                    },
                }
            }
        }

        visited
            .iter()
            .map(|(k, _)| *k)
            .collect::<HashSet<_>>()
            .len()
    }
}

#[derive(Default)]
pub struct Problem {
    mirrors: HashMap<Point<usize>, Mirror>,
    y_max: usize,
    x_max: usize,
}

impl Solution for Problem {
    fn parse(&mut self, input: String) {
        let input = input.lines().collect::<Vec<_>>();
        for (y, row) in input.iter().enumerate() {
            for (x, ch) in row.chars().enumerate() {
                if let Some(mirror) = match ch {
                    '-' => Some(Mirror::Horizontal),
                    '|' => Some(Mirror::Vertical),
                    '/' => Some(Mirror::Right),
                    '\\' => Some(Mirror::Left),
                    '.' => None,
                    _ => panic!(),
                } {
                    self.mirrors.insert(Point { x, y }, mirror);
                }
            }
        }
        self.y_max = input.len();
        self.x_max = input[0].len();
    }

    fn part1(&mut self) -> String {
        self.energize((
            Point {
                x: usize::MAX,
                y: 0,
            },
            Direction::Right,
        ))
        .to_string()
    }

    fn part2(&mut self) -> String {
        let mut best = 0;
        for y in 0..self.y_max {
            best = best.max(self.energize((Point { x: usize::MAX, y }, Direction::Right)));
        }
        for y in 0..self.y_max {
            best = best.max(self.energize((Point { x: self.x_max, y }, Direction::Left)));
        }
        for x in 0..self.x_max {
            best = best.max(self.energize((Point { x, y: usize::MAX }, Direction::Down)));
        }
        for x in 0..self.x_max {
            best = best.max(self.energize((Point { x, y: self.y_max }, Direction::Up)));
        }
        best.to_string()
    }
}

#[derive(Debug)]
enum Mirror {
    Vertical,
    Horizontal,
    Right,
    Left,
}

#[derive(Default)]
struct Grid {}

impl Grid {}
