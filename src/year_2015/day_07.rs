use utils::structure::Solution;
use utils::input;

#[derive(Default)]
pub struct Day07 {
    data: String,
}

impl Day07 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Solution for Day07 {
    fn part1(&mut self) -> String {
        format!("Not implemented")
    }
    fn part2(&mut self) -> String {
        format!("Not implemented")
    }
    fn parse(&mut self) {
        self.data = input::to_string(2015, 07);
    }
}
