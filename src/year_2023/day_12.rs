use std::collections::HashMap;

use utils::Solution;

impl Problem {
    pub fn new() -> Box<dyn Solution> {
        Box::new(Self::default())
    }
}

#[derive(Default)]
pub struct Problem {
    records: Vec<Record>,
    cache: HashMap<Record, usize>,
}

impl Solution for Problem {
    fn parse(&mut self, input: String) {
        for line in input.lines() {
            let (springs, defects) = line.split_once(' ').unwrap();
            let springs = springs
                .chars()
                .map(|ch| match ch {
                    '.' => Condition::Ok,
                    '#' => Condition::Broken,
                    '?' => Condition::Unknown,
                    _ => panic!(),
                })
                .collect::<Vec<_>>();
            let defects = defects
                .split(',')
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            self.records.push(Record { springs, defects });
        }
    }

    fn part1(&mut self) -> String {
        self.records
            .iter()
            .map(|r| calculate_possibilities(&mut self.cache, &r.springs, &r.defects))
            .sum::<usize>()
            .to_string()
    }

    fn part2(&mut self) -> String {
        for record in self.records.iter_mut() {
            record.springs.push(Condition::Unknown);
            record.springs = record.springs.repeat(5);
            record.springs.pop();
            record.defects = record.defects.repeat(5);
        }

        self.records
            .iter()
            .map(|r| calculate_possibilities(&mut self.cache, &r.springs, &r.defects))
            .sum::<usize>()
            .to_string()
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Record {
    springs: Vec<Condition>,
    defects: Vec<usize>,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
enum Condition {
    Ok,
    Broken,
    Unknown,
}

fn calculate_possibilities(
    mut cache: &mut HashMap<Record, usize>,
    springs: &[Condition],
    defects: &[usize],
) -> usize {
    if let Some(possibilities) = cache.get(&Record {
        springs: springs.to_vec(),
        defects: defects.to_vec(),
    }) {
        return *possibilities;
    }

    if defects_min_length(defects) > springs.len() {
        cache.insert(
            Record {
                springs: springs.to_vec(),
                defects: defects.to_vec(),
            },
            0,
        );
        return 0;
    }

    let mut idx_longest_defect = 0;
    for i in 1..defects.len() {
        if defects[i] > defects[idx_longest_defect] {
            idx_longest_defect = i;
        }
    }

    let longest_defect = defects[idx_longest_defect];

    let defects_left = &defects[..idx_longest_defect];
    let defects_right = &defects.get(idx_longest_defect + 1..).unwrap_or(&[]);

    let minimum_start = defects_min_length(defects_left);
    let maximum_end = springs.len()
        - if defects_right.len() > 0 {
            defects_min_length(defects_right) + 1
        } else {
            0
        };

    let mut possible_starts: Vec<usize> = Vec::new();

    'find_location: for i in minimum_start..maximum_end {
        for j in 0..longest_defect {
            if i + j >= springs.len() || springs[i + j] == Condition::Ok {
                continue 'find_location;
            }
        }

        if let Some(condition) = springs.get(i + longest_defect) {
            if *condition == Condition::Broken {
                continue 'find_location;
            }
        }
        if i > 0 {
            if let Some(condition) = springs.get(i - 1) {
                if *condition == Condition::Broken {
                    continue 'find_location;
                }
            }
        }
        possible_starts.push(i);
    }

    let mut branch_possibilities = 0;

    for start in &possible_starts {
        let mut possibilities = 1;
        let springs_left = &springs[..start.checked_sub(1).unwrap_or(0)];
        let springs_right = &springs.get(*start + longest_defect + 1..).unwrap_or(&[]);
        if !defects_left.is_empty() {
            if defects_min_length(defects_left) > springs_left.len() {
                continue;
            }
            possibilities *= calculate_possibilities(&mut cache, springs_left, defects_left);
        } else if springs_left.contains(&Condition::Broken) {
            continue;
        }
        if !defects_right.is_empty() {
            if defects_min_length(defects_right) > springs_right.len() {
                continue;
            }
            possibilities *= calculate_possibilities(&mut cache, springs_right, defects_right);
        } else if springs_right.contains(&Condition::Broken) {
            continue;
        }

        branch_possibilities += possibilities;
    }

    cache.insert(
        Record {
            springs: springs.to_vec(),
            defects: defects.to_vec(),
        },
        branch_possibilities,
    );
    branch_possibilities
}

fn defects_min_length(checksum: &[usize]) -> usize {
    if checksum.len() == 0 {
        return 0;
    } else {
        checksum.iter().map(|n| n + 1).sum::<usize>() - 1
    }
}
