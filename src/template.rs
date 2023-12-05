use utils::structure::Solution;
use utils::parsing;

#[derive(Default)]
pub struct DayDAY {
    data: String,
}

impl DayDAY {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Solution for DayDAY {
    fn part1(&mut self) -> String {
        let result = "Not implemented";
        format!("{}", result)
    }
    fn part2(&mut self) -> String {
        let result = "Not implemented";
        format!("{}", result)
    }
    fn parse(&mut self) {
        self.data = parsing::get_string(YEAR, DAY);
    }
}
