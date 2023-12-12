use utils::Solution;

impl Problem {
    pub fn new() -> Box<dyn Solution> {
        Box::new(Self::default())
    }
}

#[derive(Default)]
pub struct Problem {
}

impl Solution for Problem {
    fn parse(&mut self, input: String) {
    }

    fn part1(&mut self) -> String {
        String::from("not implemented")
    }

    fn part2(&mut self) -> String {
        String::from("not implemented")
    }
}
