use std::collections::HashMap;

// mod day_01;
// mod day_02;
// mod day_03;
// mod day_04;
// mod day_05;
mod day_06;
// mod day_07;
// mod day_08;
// mod day_09;
// mod day_10;
// mod day_11;
// mod day_12;
// mod day_13;
// mod day_14;
// mod day_15;
// mod day_16;
// mod day_17;
mod day_18;
mod day_19;
mod day_20;
mod day_21;
mod day_22;
mod day_23;
mod day_24;

type Solution = (Box<dyn Fn(&[u8]) -> String>, Box<dyn Fn(&[u8]) -> String>);

pub fn days() -> HashMap<usize, Solution> {
    let mut days: HashMap<usize, Solution> = HashMap::new();
    days.insert(6, (Box::new(day_06::part1), Box::new(day_06::part2)));
    days.insert(18, (Box::new(day_18::part1), Box::new(day_18::part2)));
    days.insert(19, (Box::new(day_19::part1), Box::new(day_19::part2)));
    days.insert(20, (Box::new(day_20::part1), Box::new(day_20::part2)));
    days.insert(21, (Box::new(day_21::part1), Box::new(day_21::part2)));
    days.insert(22, (Box::new(day_22::part1), Box::new(day_22::part2)));
    days.insert(23, (Box::new(day_23::part1), Box::new(day_23::part2)));
    days.insert(24, (Box::new(day_24::part1), Box::new(day_24::part2)));
    days
}
