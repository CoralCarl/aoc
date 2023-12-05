use std::collections::HashMap;
use std::ops::Range;

use utils::parsing;
use utils::structure::Solution;

#[derive(Default)]
pub struct Day05 {
    seeds: Vec<u64>,
    map_sets: HashMap<String, MapSet>,
}

impl Day05 {
    pub fn new() -> Self {
        Self::default()
    }

    fn location(&self, value: u64) -> u64 {
        let mut category = "seed";
        let mut value = value;

        while category != "location" {
            let map_set = &self.map_sets[category];

            for map in &map_set.maps {
                if map.range.contains(&value) {
                    value += map.delta;
                    break;
                }
            }

            category = map_set.out_type.as_str();
        }

        value
    }
}

impl Solution for Day05 {
    fn part1(&mut self) -> String {
        let mut locations: Vec<u64> = Vec::new();

        for &seed in &self.seeds {
            locations.push(self.location(seed));
        }

        format!("{}", locations.iter().min().unwrap())
    }

    fn part2(&mut self) -> String {
        let mut locations: Vec<u64> = Vec::new();

        for range in self.seeds.chunks(2) {
            for seed in range[0]..range[0] + range[1] {
                locations.push(self.location(seed));
            }
        }

        format!("{}", locations.iter().min().unwrap())
    }

    fn parse(&mut self) {
        let data = parsing::get_string(2023, 05);

        let mut blocks = data.split("\n\n");

        let (_, seeds) = blocks.next().unwrap().split_once(": ").unwrap();
        self.seeds = seeds
            .split_whitespace()
            .map(|n| n.parse::<u64>().unwrap())
            .collect();

        while let Some(block) = blocks.next() {
            let mut lines = block.lines();

            let (in_out, _) = lines.next().unwrap().split_once(' ').unwrap();
            let (in_type, out_type) = in_out.split_once("-to-").unwrap();

            let mut maps: Vec<Map> = Vec::new();

            while let Some(line) = lines.next() {
                let nums = line
                    .split_whitespace()
                    .map(|x| x.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>();

                maps.push(Map {
                    range: nums[1]..nums[1] + nums[2],
                    delta: nums[0] - nums[1],
                });
            }

            self.map_sets
                .insert(in_type.to_string(), MapSet { out_type: out_type.to_string(), maps });
        }
    }
}

struct Map {
    range: Range<u64>,
    delta: u64,
}

struct MapSet {
    out_type: String,
    maps: Vec<Map>,
}
