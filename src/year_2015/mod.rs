use std::collections::HashMap;
use utils::Solution;

mod day_01;
mod day_02;

pub fn days() -> HashMap<usize, Box<dyn Fn() -> Box<dyn Solution>>> {
    let mut days: HashMap<usize, Box<dyn Fn() -> Box<dyn Solution>>> = HashMap::new();

    days.insert(1, Box::new(day_01::Problem::new));
    days.insert(2, Box::new(day_02::Problem::new));

    days
}
