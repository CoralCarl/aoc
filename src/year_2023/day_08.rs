use std::collections::HashMap;

use utils::input;
use utils::structure::Solution;

#[derive(Default)]
pub struct Day08 {
    instruction: String,
    map: HashMap<String, Vec<String>>,
}

impl Day08 {
    pub fn new() -> Self {
        Self::default()
    }

    fn next_instruction(&self, steps: usize) -> usize {
        "LR".find(
            self.instruction
                .chars()
                .nth(steps % self.instruction.len())
                .unwrap(),
        )
        .unwrap()
    }
}

impl Solution for Day08 {
    fn part1(&mut self) -> String {
        let mut location = String::from("AAA");

        let mut steps = 0;
        while location != "ZZZ" {
            location = self.map.get(&location).unwrap()[self.next_instruction(steps)].to_string();
            steps += 1;
        }

        format!("{}", steps)
    }
    fn part2(&mut self) -> String {
        let mut locations = self
            .map
            .keys()
            .filter(|s| s.ends_with('A'))
            .collect::<Vec<_>>();

        let mut steps = 0;
        while locations
            .iter()
            .filter(|s| !s.ends_with('Z'))
            .collect::<Vec<_>>()
            .len()
            > 0
        {
            for location in locations.iter_mut() {
                *location = self
                    .map
                    .get(location.as_str())
                    .unwrap()
                    .get(self.next_instruction(steps))
                    .unwrap();
            }
            steps += 1;
        }

        format!("{}", steps)
    }
    fn parse(&mut self) {
        let lines = input::to_lines(2023, 08);
        let _lines = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"
            .lines()
            .collect::<Vec<_>>();

        let instruction = lines[0].to_string();
        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        for line in &lines[2..] {
            let (key, list) = line.split_once(" = ").unwrap();
            println!("{key} {list:?}");
            let list = list[1..list.len() - 1]
                .split(", ")
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            map.insert(key.to_string(), list);
        }
        self.instruction = instruction;
        self.map = map;
    }
}
