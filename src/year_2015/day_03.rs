use std::collections::HashSet;
use utils::parsing;
use utils::structure::Solution;

#[derive(Default)]
pub struct Day03 {
    data: String,
}

impl Day03 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Solution for Day03 {
    fn part1(&mut self) -> Vec<String> {
        let mut grid = HashSet::new();
        let mut x = 0;
        let mut y = 0;
        for c in self.data.chars() { 
            match c {
                '>' => x += 1,
                '<' => x -= 1,
                '^' => y += 1,
                'v' => y -= 1,
                _ => panic!("Invalid input '{c}'"),
            }
            grid.insert(format!("{x},{y}"));
        }

        vec![format!("{}", grid.len())]
    }
    fn part2(&mut self) -> Vec<String> {
        let mut grid = HashSet::new();
        let mut x = [0,0];
        let mut y = [0,0];
        let mut i = 0;
        for c in self.data.chars() { 
            match c {
                '>' => x[i] += 1,
                '<' => x[i] -= 1,
                '^' => y[i] += 1,
                'v' => y[i] -= 1,
                _ => panic!("Invalid input '{c}'"),
            }
            grid.insert(format!("{},{}", x[i], y[i]));
            i = 1 - i;
        }

        vec![format!("{}", grid.len())]
    }
    fn parse(&mut self) {
        self.data = parsing::get_string(2015, 03);
    }
}
