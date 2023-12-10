use utils::parsing;
use utils::structure::Solution;

#[derive(Default)]
pub struct Day09 {
    sequences: Vec<Vec<i32>>,
}

impl Day09 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Solution for Day09 {
    fn part1(&mut self) -> String {
        let result: i32 = self.sequences.iter().map(|seq| extrapolate(&seq)).sum();
        format!("{}", result)
    }
    fn part2(&mut self) -> String {
        let result: i32 = self.sequences.iter().map(|seq| extrapolate_rev(&seq)).sum();
        format!("{}", result)
    }
    fn parse(&mut self) {
        let lines = parsing::get_lines(2023, 09);
        for line in &lines {
            self.sequences.push(
                line.split_whitespace()
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
    }
}

fn extrapolate(seq: &[i32]) -> i32 {
    let mut sequences = vec![Vec::from(seq)];

    while !all_zero(sequences.last().unwrap()) {
        sequences.push(differences(sequences.last().unwrap()));
    }

    let mut extrapolated = 0;
    for seq in sequences.iter().rev().skip(1) {
        extrapolated = extrapolated + seq.last().unwrap();
    }

    extrapolated
}

fn extrapolate_rev(seq: &[i32]) -> i32 {
    let mut sequences = vec![Vec::from(seq)];

    while !all_zero(sequences.last().unwrap()) {
        sequences.push(differences(sequences.last().unwrap()));
    }

    let mut extrapolated = 0;
    for seq in sequences.iter().rev().skip(1) {
        extrapolated = seq.first().unwrap() - extrapolated;
    }

    extrapolated
}

fn all_zero(seq: &[i32]) -> bool {
    for &num in seq {
        if num != 0 {
            return false;
        }
    }
    true
}

fn differences(seq: &[i32]) -> Vec<i32> {
    let mut diffs: Vec<i32> = Vec::new();

    for i in 1..seq.len() {
        diffs.push(seq[i] - seq[i - 1]);
    }

    diffs
}
