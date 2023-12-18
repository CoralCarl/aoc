use std::collections::HashMap;
use utils::Solution;

// mod day_01;
// mod day_02;
// mod day_03;
// mod day_04;
// mod day_05;
// mod day_06;
// mod day_07;
mod day_08;
// mod day_09;
// mod day_10;
// mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;
mod day_18;

pub fn days() -> HashMap<usize, Box<dyn Fn() -> Box<dyn Solution>>> {
    let mut days: HashMap<usize, Box<dyn Fn() -> Box<dyn Solution>>> = HashMap::new();

    // days.insert(1, Box::new(day_01::Problem::new));
    // days.insert(2, Box::new(day_02::Problem::new));
    // days.insert(3, Box::new(day_03::Problem::new));
    // days.insert(4, Box::new(day_04::Problem::new));
    // days.insert(5, Box::new(day_05::Problem::new));
    // days.insert(6, Box::new(day_06::Problem::new));
    // days.insert(7, Box::new(day_07::Problem::new));
    days.insert(8, Box::new(day_08::Problem::new));
    // days.insert(9, Box::new(day_09::Problem::new));
    // days.insert(10, Box::new(day_10::Problem::new));
    // days.insert(11, Box::new(day_11::Problem::new));
    days.insert(12, Box::new(day_12::Problem::new));
    days.insert(13, Box::new(day_13::Problem::new));
    days.insert(14, Box::new(day_14::Problem::new));
    days.insert(15, Box::new(day_15::Problem::new));
    days.insert(16, Box::new(day_16::Problem::new));
    days.insert(17, Box::new(day_17::Problem::new));
    days.insert(18, Box::new(day_18::Problem::new));

    days
}
