use std::collections::HashMap;

pub trait Solution {
    fn part1(&mut self) -> String;
    fn part2(&mut self) -> String;
    fn parse(&mut self);
}

pub trait Year {
    fn get_days(&mut self) -> &mut HashMap<usize, Box<dyn Solution>>;
}
