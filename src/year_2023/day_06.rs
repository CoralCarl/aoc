// use utils::input;
//
// #[derive(Default)]
// pub struct Day06 {
//     races: Vec<Race>,
//     race: Race,
// }
//
// impl Day06 {
//     pub fn new() -> Self {
//         Self::default()
//     }
// }
//
// impl Solution for Day06 {
//     fn part1(&mut self) -> String {
//         let result: u64 = self.races.iter().map(|race| race.win_chances()).product();
//         format!("{}", result)
//     }
//     fn part2(&mut self) -> String {
//         let result = self.race.win_chances();
//         format!("{}", result)
//     }
//     fn parse(&mut self) {
//         let data = input::to_lines(2023, 06);
//         let times = data[0].split_whitespace().skip(1);
//         let distances = data[1].split_whitespace().skip(1);
//
//         for (time, record) in times.zip(distances) {
//             self.races.push(Race {
//                 time: time.parse::<f64>().unwrap(),
//                 distance: record.parse::<f64>().unwrap(),
//             });
//         }
//
//         let time = data[0]["Time:".len()..]
//             .replace(" ", "")
//             .parse::<f64>()
//             .unwrap();
//         let distance = data[1]["Distance:".len()..]
//             .replace(" ", "")
//             .parse::<f64>()
//             .unwrap();
//         self.race = Race { time, distance };
//     }
// }

fn win_chances(time: &f64, distance: &f64) -> u64 {
    let diff =(time.powi(2) - 4.0 * distance).sqrt();
    let a = (time + diff) / 2.0;
    let b = (time - diff) / 2.0;
    a.ceil() as u64 - b.floor() as u64 - 1
}

pub fn part1(input: &[u8]) -> String {
    let mut nums = [0f64; 8];
    let mut i = 13;
    let mut count = 0;
    while count < 8 {
        while input[i] == b' ' {
            i += 1;
        }
        let mut j = i + 1;
        while input[j] != b' ' && input[j] != b'\n' {
            j += 1;
        }
        while i < j {
            nums[count] = nums[count] * 10.0 + (input[i] - b'0') as f64;
            i += 1;
        }
        count += 1;
        if input[i] == b'\n' {
            i += 13;
        }
    }
    let mut result = 1;
    for (i, j) in [(0, 4), (1,5), (2,6), (3,7)] {
        result *= win_chances(&nums[i], &nums[j]);
    }
    result.to_string()
}

pub fn part2(input: &[u8]) -> String {
    let x = input[0] as char;
    x.to_string()
}
