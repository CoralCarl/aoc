use std::collections::HashMap;

use utils::Solution;

impl Problem {
    pub fn new() -> Box<dyn Solution> {
        Box::new(Self::default())
    }
}

#[derive(Default)]
pub struct Problem {
    instructions: Vec<usize>,
    maps: HashMap<String, [String; 2]>,
}

impl Solution for Problem {
    fn parse(&mut self, input: String) {
        let mut lines = input.lines();
        self.instructions = lines
            .next()
            .unwrap()
            .chars()
            .map(|ch| "LR".find(ch).unwrap())
            .collect::<Vec<usize>>();

        for line in lines.skip(1) {
            let (key, list) = line.split_once(" = ").unwrap();
            let (left, right) = list[1..list.len() - 1].split_once(", ").unwrap();
            self.maps
                .insert(key.to_string(), [left.to_string(), right.to_string()]);
        }
    }

    fn part1(&mut self) -> String {
        let mut location = &String::from("AAA");
        let mut steps = 0;
        for instruction in self.instructions.iter().cycle() {
            location = &self.maps.get(location).unwrap()[*instruction];
            steps += 1;
            if location == &String::from("ZZZ") {
                break;
            }
        }
        steps.to_string()
    }

    fn part2(&mut self) -> String {
        let mut locations = self
            .maps
            .keys()
            .filter(|s| s.ends_with('A'))
            .collect::<Vec<_>>();

        let mut cycles: Vec<usize> = Vec::new();

        for location in locations.iter_mut() {
            let mut instruction = self.instructions.iter().cycle();
            let mut visited: Vec<&String> = Vec::new();

            while !(visited.contains(&location) && location.ends_with('Z')) {
                visited.push(*location);
                *location = &self.maps.get(*location).unwrap()[*instruction.next().unwrap()];
            }

            let cycle = visited.iter().position(|loc| *loc == *location).unwrap();

            cycles.push(cycle );
        }

        lcm(cycles).to_string()
    }
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn lcm(nums: Vec<usize>) -> usize {
    let mut n = nums.iter();
    let mut a = *n.next().unwrap();
    while let Some(&b) = n.next() {
        a = a * b / gcd(a,b);
    }
    a
}
