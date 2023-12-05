use utils::structure::Solution;
use utils::parsing;

pub struct Day01 {
    data: String,
}

impl Day01 {
    pub fn new() -> Self {
        Self {
            data: String::new(),
        }
    }
}

impl Solution for Day01 {
    fn part1(&mut self) -> String {
        let mut floor = 0;
        for c in self.data.chars() {
            match c {
                '(' => floor += 1,
                ')' => floor -= 1,
                _ => (),
            }
        }
        floor.to_string()
    }
    fn part2(&mut self) -> String {
        let mut floor = 0;
        let mut i = 1;
        for c in self.data.chars() {
            match c {
                '(' => floor += 1,
                ')' => floor -= 1,
                _ => (),
            }
            if floor == -1 {
                break;
            }
            i += 1;
        }
        i.to_string()
    }
    fn parse(&mut self) {
        self.data = parsing::get_string(2015, 1);
    }
}
