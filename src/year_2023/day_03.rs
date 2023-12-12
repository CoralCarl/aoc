use std::collections::HashMap;

use utils::input;
use utils::structure::Solution;

#[derive(Default)]
pub struct Day03 {
    symbols: HashMap<[usize; 2], char>,
    numbers: Vec<Number>,
    size_x: usize,
    size_y: usize,
}

impl Day03 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Solution for Day03 {
    fn part1(&mut self) -> String {
        let mut result = 0;
        
        let mut lonely_nums: Vec<usize> = Vec::new();

        'next_num: for (i, num) in self.numbers.iter().enumerate() {
            let y_min = num.y.checked_sub(1).unwrap_or(0);
            let x_min = num.x_min.checked_sub(1).unwrap_or(0);
            let y_max = num.y + 1;
            let x_max = num.x_max + 1;

            for y in y_min..=y_max {
                for x in x_min..=x_max {
                    if self.symbols.get(&[x, y]).is_some() {
                        result += num.value;
                        continue 'next_num;
                    }
                }
            }

            lonely_nums.push(i);
        }

        for i in lonely_nums.iter().rev() {
            self.numbers.remove(*i);
        }

        format!("{}", result)
    }
    fn part2(&mut self) -> String {
        let mut result = 0;

        for ([x, y], ch) in &self.symbols {
            if *ch != '*' {
                continue;
            }

            let y_min = y.checked_sub(1).unwrap_or(0);
            let x_min = x.checked_sub(1).unwrap_or(0);
            let y_max = self.size_y.min(y + 1);
            let x_max = self.size_x.min(x + 1);

            let mut adjacent: Vec<usize> = Vec::new();

            'next_num: for num in &self.numbers {
                for y in y_min..=y_max {
                    for x in x_min..=x_max {
                        if num.y == y && x >= num.x_min && x <= num.x_max {
                            adjacent.push(num.value);
                            continue 'next_num;
                        }
                    }
                }
            }

            if adjacent.len() == 2 {
                result += adjacent.iter().product::<usize>();
            }
        }

        format!("{}", result)
    }
    fn parse(&mut self) {
        let lines = input::to_lines(2023, 03);

        self.size_y = lines.len();
        self.size_x = lines[0].len();

        for (y, line) in lines.iter().enumerate() {
            let mut num_str = String::new();

            for (x, ch) in line.chars().enumerate() {
                match ch {
                    '0'..='9' => {
                        num_str += &ch.to_string();
                    }
                    _ => {
                        if !num_str.is_empty() {
                            self.numbers.push(Number {
                                value: num_str.parse::<usize>().unwrap(),
                                x_min: x - num_str.len(),
                                x_max: x - 1,
                                y,
                            });
                            num_str.clear();
                        }
                        if ch != '.' {
                            self.symbols.insert([x, y], ch);
                        }
                    }
                }
                if x == line.len() - 1 && !num_str.is_empty() {
                    self.numbers.push(Number {
                        value: num_str.parse::<usize>().unwrap(),
                        x_min: x - num_str.len() + 1,
                        x_max: x,
                        y,
                    });
                }
            }
        }
    }
}

#[derive(Debug)]
struct Number {
    value: usize,
    x_min: usize,
    x_max: usize,
    y: usize,
}
