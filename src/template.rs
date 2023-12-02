use utils::structure::Solution;
use utils::parsing;

#[derive(Default)]
pub struct Day05 {
    data: String,
}

impl Day05 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Solution for Day05 {
    fn part1(&mut self) -> Vec<String> {
        vec![format!("Not implemented")]
    }
    fn part2(&mut self) -> Vec<String> {
        vec![format!("Not implemented")]
    }
    fn parse(&mut self) {
        self.data = parsing::get_string(2015, 05);
    }
}
