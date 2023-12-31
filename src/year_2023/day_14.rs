use std::collections::HashSet;
use utils::geometry::{Direction, Point};
use utils::Solution;

impl Problem {
    pub fn new() -> Box<dyn Solution> {
        Box::new(Self::default())
    }
}

#[derive(Default)]
pub struct Problem {
    dish: Dish,
}

impl Solution for Problem {
    fn parse(&mut self, input: String) {
        let x_size = input.find('\n').unwrap();
        let mut y_size = input.len() / x_size;

        if input.ends_with('\n') {
            y_size -= 1;
        }

        let mut boulders: HashSet<Point<usize>> = HashSet::new();
        let mut rocks: HashSet<Point<usize>> = HashSet::new();

        for (y, row) in input.lines().enumerate() {
            for (x, ch) in row.chars().enumerate() {
                match ch {
                    'O' => boulders.insert(Point { x, y }),
                    '#' => rocks.insert(Point { x, y }),
                    '.' => false,
                    _ => panic!(),
                };
            }
        }

        self.dish = Dish {
            rocks,
            boulders,
            x_size,
            y_size,
        };
    }

    fn part1(&mut self) -> String {
        let mut grid = self.dish.clone();
        grid.tilt(Direction::Up);
        grid.structural_load().to_string()
    }

    fn part2(&mut self) -> String {
        self.dish.spin_cycle(1000000000);
        self.dish.structural_load().to_string()
    }
}

#[derive(Default, Clone)]
struct Dish {
    rocks: HashSet<Point<usize>>,
    boulders: HashSet<Point<usize>>,
    x_size: usize,
    y_size: usize,
}

impl Dish {
    fn structural_load(&self) -> usize {
        self.boulders
            .iter()
            .map(|b| self.y_size - b.y as usize)
            .sum::<usize>()
    }

    fn tilt(&mut self, d: Direction) {
        let mut boulders: HashSet<Point<usize>> = HashSet::new();
        let mut removed: HashSet<Point<usize>> = HashSet::new();

        for b in self.boulders.iter() {
            let mut valid_position = *b;
            let mut position = *b;
            loop {
                position.move_by(d, 1);
                if position.x >= self.x_size
                    || position.y >= self.y_size
                    || self.rocks.contains(&position)
                {
                    break;
                }
                if !boulders.contains(&position)
                    && (!self.boulders.contains(&position) || removed.contains(&position))
                {
                    valid_position = position;
                }
            }
            boulders.insert(valid_position);
            removed.insert(*b);
        }
        self.boulders = boulders;
    }

    fn spin_cycle(&mut self, cycles: usize) {
        let mut visited = Vec::from([self.boulders.clone()]);
        for i in 0..cycles {
            for d in Direction::ccw() {
                self.tilt(d);
            }

            if let Some(offset) = visited.iter().position(|state| *state == self.boulders) {
                let cycle = i + 1 - offset;
                self.boulders = visited[(cycles - offset) % cycle + offset].clone();
                break;
            } else {
                visited.push(self.boulders.clone());
            }
        }
    }

    #[allow(dead_code)]
    fn print(&self) {
        for y in 0..self.y_size {
            for x in 0..self.x_size {
                let y = y as usize;
                let x = x as usize;
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
