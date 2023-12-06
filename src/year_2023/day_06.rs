use utils::parsing;
use utils::structure::Solution;

#[derive(Default)]
pub struct Day06 {
    races: Vec<Race>,
    race: Race,
}

impl Day06 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Solution for Day06 {
    fn part1(&mut self) -> String {
        let result: u64 = self.races.iter().map(|race| race.win_chances()).product();
        format!("{}", result)
    }
    fn part2(&mut self) -> String {
        let result = self.race.win_chances();
        format!("{}", result)
    }
    fn parse(&mut self) {
        let data = parsing::get_lines(2023, 06);
        let times = data[0].split_whitespace().skip(1);
        let distances = data[1].split_whitespace().skip(1);

        for (time, record) in times.zip(distances) {
            self.races.push(Race {
                time: time.parse::<u64>().unwrap(),
                distance: record.parse::<u64>().unwrap(),
            });
        }

        let time = data[0]["Time:".len()..]
            .replace(" ", "")
            .parse::<u64>()
            .unwrap();
        let distance = data[1]["Distance:".len()..]
            .replace(" ", "")
            .parse::<u64>()
            .unwrap();
        self.race = Race { time, distance };
    }
}

fn zero_point(p: f64, q: f64) -> Vec<f64> {
    let a = (p + (p.powi(2) - 4.0 * q).sqrt()) / 2.0;
    let b = (p - (p.powi(2) - 4.0 * q).sqrt()) / 2.0;

    if a < b {
        vec![a, b]
    } else {
        vec![b, a]
    }
}

#[derive(Default)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn win_chances(&self) -> u64 {
        let zeros = zero_point(self.time as f64, self.distance as f64);
        zeros[1].ceil() as u64 - zeros[0].floor() as u64 - 1
    }
}
