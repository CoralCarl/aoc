use std::collections::HashMap;
use std::ops::Range;

use utils::parsing;
use utils::structure::Solution;

#[derive(Default)]
pub struct Day05 {
    seeds: Vec<i128>,
    map_sets: HashMap<String, MapSet>,
}

impl Day05 {
    pub fn new() -> Self {
        Self::default()
    }

    fn range_location(&self, range: Range<i128>) -> i128 {
        let mut category = "seed";
        let mut ranges = vec![range];

        while category != "location" {
            let map_set = &self.map_sets[category];

            let mut new_ranges: Vec<Range<i128>> = Vec::new();
            for range in ranges {
                new_ranges.append(&mut range_map(&range, &map_set.maps));
            }

            category = map_set.out_type.as_str();

            new_ranges.sort_by(|a, b| a.start.cmp(&b.start));
            let mut i = 0;
            while i < new_ranges.len() - 1 {
                if new_ranges[i].end >= new_ranges[i + 1].start {
                    new_ranges[i].end = new_ranges[i + 1].end;
                    new_ranges.remove(i + 1);
                } else {
                    i += 1;
                }
            }

            ranges = new_ranges;
        }

        ranges.sort_by(|a, b| a.start.cmp(&b.start));
        ranges[0].start
    }

    fn location(&self, value: i128) -> i128 {
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
        let mut locations: Vec<i128> = Vec::new();

        for &seed in &self.seeds {
            locations.push(self.location(seed));
        }

        format!("{}", locations.iter().min().unwrap())
    }
    fn part2(&mut self) -> String {
        let mut locations: Vec<i128> = Vec::new();

        for range in self.seeds.chunks(2) {
            locations.push(self.range_location(range[0]..range[0] + range[1]));
        }

        format!("{}", locations.iter().min().unwrap())
    }

    fn parse(&mut self) {
        let data = parsing::get_string(2023, 05);

        let mut blocks = data.split("\n\n");

        let (_, seeds) = blocks.next().unwrap().split_once(": ").unwrap();
        self.seeds = seeds
            .split_whitespace()
            .map(|n| n.parse::<i128>().unwrap())
            .collect();

        while let Some(block) = blocks.next() {
            let mut lines = block.lines();

            let (in_out, _) = lines.next().unwrap().split_once(' ').unwrap();
            let (in_type, out_type) = in_out.split_once("-to-").unwrap();

            let mut maps: Vec<Map> = Vec::new();

            while let Some(line) = lines.next() {
                let nums = line
                    .split_whitespace()
                    .map(|x| x.parse::<i128>().unwrap())
                    .collect::<Vec<i128>>();

                maps.push(Map {
                    range: nums[1]..nums[1] + nums[2],
                    delta: nums[0] - nums[1],
                });
            }

            self.map_sets.insert(
                in_type.to_string(),
                MapSet {
                    out_type: out_type.to_string(),
                    maps,
                },
            );
        }
    }
}

#[derive(Debug)]
struct Map {
    range: Range<i128>,
    delta: i128,
}

struct MapSet {
    out_type: String,
    maps: Vec<Map>,
}

fn range_map(range: &Range<i128>, maps: &Vec<Map>) -> Vec<Range<i128>> {
    let mut todo = vec![range.clone()];
    let mut done: Vec<Range<i128>> = Vec::new();

    for map in maps {
        let mut remaining: Vec<Range<i128>> = Vec::new();

        while let Some(range) = todo.pop() {
            let left = range.start..range.end.min(map.range.start);
            let right = range.start.max(map.range.end)..range.end;
            let mid = range.start.max(map.range.start) + map.delta
                ..range.end.min(map.range.end) + map.delta;

            if left.start < left.end {
                remaining.push(left);
            }
            if right.start < right.end {
                remaining.push(right);
            }
            if mid.start < mid.end {
                done.push(mid);
            }
        }
        if remaining.is_empty() {
            break;
        }
        todo = remaining;
    }

    done.append(&mut todo);

    done
}

#[allow(dead_code)]
fn range_collapse<T>(ranges: &[Range<T>]) -> Vec<Range<T>>
where
    T: Ord + Copy,
{
    let mut new_ranges: Vec<Range<T>> = Vec::new();

    for range in ranges {
        new_ranges.push(range.start..range.end);
    }
    new_ranges.sort_by(|a, b| a.start.cmp(&b.start));

    let mut i = 0;

    while i < new_ranges.len() - 1 {
        if new_ranges[i].end >= new_ranges[i + 1].start {
            if new_ranges[i].end < new_ranges[i + 1].end {
                new_ranges[i].end = new_ranges[i + 1].end;
            }
            new_ranges.remove(i + 1);
        } else {
            i += 1;
        }
    }
    new_ranges
}
