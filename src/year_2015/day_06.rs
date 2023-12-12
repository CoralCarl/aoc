use std::str::FromStr;

use utils::input;
use utils::structure::Solution;

#[derive(Default)]
pub struct Day06 {
    data: Vec<Command>,
}

impl Day06 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Solution for Day06 {
    fn part1(&mut self) -> String {
        let mut grid = [[0; 1000]; 1000];
        for cmd in self.data.iter() {
            for row in &mut grid[cmd.start.y..=cmd.end.y] {
                for cell in &mut row[cmd.start.x..=cmd.end.x] {
                    match cmd.value.as_str() {
                        "on" => *cell = 1,
                        "off" => *cell = 0,
                        "toggle" => *cell = 1 - *cell,
                        _ => panic!("invalid command"),
                    }
                }
            }
        }
        let result = grid
            .iter()
            .map(|row| row.iter().sum::<usize>())
            .sum::<usize>();
        format!("{}", result)
    }

    fn part2(&mut self) -> String {
        let mut grid = [[0; 1000]; 1000];
        for cmd in self.data.iter() {
            for row in &mut grid[cmd.start.y..=cmd.end.y] {
                for cell in &mut row[cmd.start.x..=cmd.end.x] {
                    match cmd.value.as_str() {
                        "on" => *cell += 1,
                        "off" => *cell -= 1,
                        "toggle" => *cell += 2,
                        _ => panic!("invalid command"),
                    }
                }
            }
        }
        let result = grid
            .iter()
            .map(|row| row.iter().sum::<usize>())
            .sum::<usize>();
        format!("{}", result)
    }

    fn parse(&mut self) {
        let lines = input::to_lines(2015, 06);
        for line in lines.iter() {
            let words: Vec<_> = line.split(' ').collect();
            let value: String;
            let start: Point<usize>;
            let end: Point<usize>;
            if words[0] == "turn" {
                value = words[1].to_string();
                start = Point::from_string(words[2], ",");
                end = Point::from_string(words[4], ",");
            } else {
                value = words[0].to_string();
                start = Point::from_string(words[1], ",");
                end = Point::from_string(words[3], ",");
            }
            self.data.push(Command { value, start, end });
        }
    }
}

#[derive(Eq, PartialEq, Hash)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn from_string(s: &str, delimiter: &str) -> Self
    where
        T: FromStr,
        <T as FromStr>::Err: std::fmt::Debug,
    {
        let mut spl = s.split(delimiter);
        Self {
            x: spl.next().unwrap().parse::<T>().unwrap(),
            y: spl.next().unwrap().parse::<T>().unwrap(),
        }
    }
}

struct Command {
    value: String,
    start: Point<usize>,
    end: Point<usize>,
}
