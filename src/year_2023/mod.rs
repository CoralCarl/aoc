use std::collections::HashMap;

use utils::structure::*;

mod day_01;
use day_01::*;
mod day_02;
use day_02::*;

pub struct Year2023 {
    days: HashMap<usize, Box<dyn Solution>>,
}

impl Year2023 {
    pub fn new() -> Self {
        Self {
            days: HashMap::from([
                (1, Box::new(Day01::new()) as Box<dyn Solution>),
                (2, Box::new(Day02::new()) as Box<dyn Solution>),
            ]),
        }
    }
}

impl Year for Year2023 {
    fn get_days(&mut self) -> &mut HashMap<usize, Box<dyn Solution>> {
        &mut self.days
    }
}
