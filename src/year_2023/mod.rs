use std::collections::HashMap;

use utils::structure::*;

mod day_01;
use day_01::*;
mod day_02;
use day_02::*;
mod day_03;
use day_03::*;
mod day_04;
use day_04::*;
mod day_05;
use day_05::*;
mod day_06;
use day_06::*;
mod day_07;
use day_07::*;
mod day_08;
use day_08::*;

pub struct Year2023 {
    days: HashMap<usize, Box<dyn Solution>>,
}

impl Year2023 {
    pub fn new() -> Self {
        Self {
            days: HashMap::from([
                (1, Box::new(Day01::new()) as Box<dyn Solution>),
                (2, Box::new(Day02::new()) as Box<dyn Solution>),
                (3, Box::new(Day03::new()) as Box<dyn Solution>),
                (4, Box::new(Day04::new()) as Box<dyn Solution>),
                (5, Box::new(Day05::new()) as Box<dyn Solution>),
                (6, Box::new(Day06::new()) as Box<dyn Solution>),
                (7, Box::new(Day07::new()) as Box<dyn Solution>),
                (8, Box::new(Day08::new()) as Box<dyn Solution>),
            ]),
        }
    }
}

impl Year for Year2023 {
    fn get_days(&mut self) -> &mut HashMap<usize, Box<dyn Solution>> {
        &mut self.days
    }
}
