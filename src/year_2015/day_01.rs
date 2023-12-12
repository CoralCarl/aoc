use utils::Solution;

impl Problem {
    pub fn new() -> Box<dyn Solution> {
        Box::new(Self::default())
    }
}

#[derive(Default)]
pub struct Problem {
    data: String,
}

impl Solution for Problem {
    fn parse(&mut self, input: String) {
        self.data = input;
    }

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
}
