use utils::structure::Solution;
use utils::parsing;

#[derive(Default)]
pub struct Day04 {
    data: String,
    i: usize,
}

impl Day04 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Solution for Day04 {
    fn part1(&mut self) -> Vec<String> {
        loop {
            let s = format!("{}{}", self.data, self.i);
            let hash = md5::compute(s.as_bytes());
            if format!("{:x}", hash).starts_with("00000") {
                break
            }
            self.i += 1;
        }
        vec![format!("{}", self.i)]
    }
    fn part2(&mut self) -> Vec<String> {
        loop {
            let s = format!("{}{}", self.data, self.i);
            let hash = md5::compute(s.as_bytes());
            if format!("{:x}", hash).starts_with("000000") {
                break
            }
            self.i += 1;
        }
        vec![format!("{}", self.i)]
    }
    fn parse(&mut self) {
        self.data = parsing::get_string(2015, 04);
    }
}
