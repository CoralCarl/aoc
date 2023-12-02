use std::collections::HashMap;

pub trait Solution {
    fn part1(&mut self) -> Vec<String>;
    fn part2(&mut self) -> Vec<String>;
    fn parse(&mut self);
}

pub trait Year {
    fn get_days(&mut self) -> &mut HashMap<usize, Box<dyn Solution>>;
}
