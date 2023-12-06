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
                time: time.parse::<f64>().unwrap(),
                distance: record.parse::<f64>().unwrap(),
            });
        }

        let time = data[0]["Time:".len()..]
            .replace(" ", "")
            .parse::<f64>()
            .unwrap();
        let distance = data[1]["Distance:".len()..]
            .replace(" ", "")
            .parse::<f64>()
            .unwrap();
        self.race = Race { time, distance };
    }
}

#[derive(Default)]
struct Race {
    time: f64,
    distance: f64,
}

impl Race {
    fn win_chances(&self) -> u64 {
        let a = (self.time + (self.time.powi(2) - 4.0 * self.distance).sqrt()) / 2.0;
        let b = (self.time - (self.time.powi(2) - 4.0 * self.distance).sqrt()) / 2.0;
        a.ceil() as u64 - b.floor() as u64 - 1
    }
}
