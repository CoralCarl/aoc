use utils::parsing;
use utils::structure::Solution;

#[derive(Default)]
pub struct Day01 {
    lines: Vec<String>,
}

impl Day01 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Solution for Day01 {
    fn part1(&mut self) -> Vec<String> {
        let mut nums: Vec<u32> = Vec::new();
        for line in &self.lines {
            let mut first: Option<u32> = None;
            let mut last: Option<u32> = None;

            for c in line.chars() {
                if c.is_ascii_digit() {
                    let num = c.to_digit(10);
                    if first.is_none() {
                        first = num;
                    }
                    last = num;
                }
            }

            nums.push(10 * first.unwrap() + last.unwrap());
        }

        vec![format!("{}", nums.iter().sum::<u32>())]
    }

    fn part2(&mut self) -> Vec<String> {
        let mut nums: Vec<u32> = Vec::new();

        for line in self.lines.iter() {
            let mut first: Option<u32> = None;
            let mut last: Option<u32> = None;

            for (i, c) in line.char_indices() {
                let mut num: Option<u32> = None;

                if c.is_ascii_digit() {
                    num = c.to_digit(10);
                } else {
                    for (j, s_num) in [
                        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
                    ]
                    .iter()
                    .enumerate()
                    {
                        if line[i..].starts_with(s_num) {
                            num = Some(j as u32 + 1);
                            break;
                        }
                    }
                }
                if num.is_some() {
                    if first.is_none() {
                        first = num;
                    }
                    last = num;
                }
            }
            nums.push(10 * first.unwrap() + last.unwrap());
        }
        vec![format!("{}", nums.iter().sum::<u32>())]
    }

    fn parse(&mut self) {
        self.lines = parsing::get_lines(2023, 01);
    }
}
