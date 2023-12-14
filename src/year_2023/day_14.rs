use std::collections::HashSet;
use std::time::Instant;

use utils::geometry::{Direction, Point};
use utils::Solution;

impl Problem {
    pub fn new() -> Box<dyn Solution> {
        Box::new(Self::default())
    }
}

#[derive(Default)]
pub struct Problem {
    grid: Grid,
}

impl Solution for Problem {
    fn parse(&mut self, input: String) {
        let x_max = input.find('\n').unwrap();
        let mut y_max = input.len() / x_max;

        if input.ends_with('\n') {
            y_max -= 1;
        }

        let mut boulders: Vec<Point<i32>> = Vec::new();
        let mut rocks: HashSet<Point<i32>> = HashSet::new();

        for (y, row) in input.lines().enumerate() {
            for (x, ch) in row.chars().enumerate() {
                match ch {
                    'O' => {
                        boulders.push(Point::from((x as i32, y as i32)));
                    }
                    '#' => {
                        rocks.insert(Point::from((x as i32, y as i32)));
                    }
                    '.' => (),
                    _ => panic!(),
                }
            }
        }

        self.grid = Grid {
            rocks,
            boulders,
            x_max,
            y_max,
        };
    }

    fn part1(&mut self) -> String {
        let mut grid = self.grid.clone();
        grid.tilt(Direction::North);
        grid.structural_load().to_string()
    }

    fn part2(&mut self) -> String {
        self.grid.spin_cycle(1000000000);
        self.grid.structural_load().to_string()
    }
}

#[derive(Default, Clone)]
struct Grid {
    rocks: HashSet<Point<i32>>,
    boulders: Vec<Point<i32>>,
    x_max: usize,
    y_max: usize,
}

impl Grid {
    fn structural_load(&self) -> usize {
        self.boulders
            .iter()
            .map(|b| self.y_max - b.y as usize)
            .sum::<usize>()
    }

    fn tilt(&mut self, d: Direction) {
        for i in 0..self.boulders.len() {
            let mut pos = self.boulders[i];
            loop {
                pos += d.as_point();
                if pos.x < 0
                    || pos.y < 0
                    || pos.x >= self.x_max as i32
                    || pos.y >= self.y_max as i32
                    || self.rocks.contains(&pos)
                {
                    break;
                }
                if !self.boulders.contains(&pos) {
                    self.boulders[i] = pos;
                }
            }
        }
    }

    fn spin_cycle(&mut self, cycles: usize) {
        let mut visited = Vec::from([self.boulders.clone()]);
        for i in 0..cycles {
            for d in Direction::ccw() {
                self.tilt(d);
            }

            // self.print();

            if visited.contains(&self.boulders) {
                let offset = visited
                    .iter()
                    .position(|state| *state == self.boulders)
                    .unwrap();
                let cycle = i + 1- offset;
                self.boulders = visited[(cycles - offset) % cycle + offset].clone();
                break;
            } else {
                visited.push(self.boulders.clone());
            }
        }
    }

    #[allow(dead_code)]
    fn print(&self) {
        for y in 0..self.y_max {
            for x in 0..self.x_max {
                let y = y as i32;
                let x = x as i32;
                if self.boulders.contains(&Point { x, y }) {
                    print!("O");
                } else if self.rocks.contains(&Point { x, y }) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}
