pub mod structure;
pub mod input;
pub mod parse;
pub mod geometry;

pub trait Solution {
    fn part1(&mut self) -> String;
    fn part2(&mut self) -> String;
    fn parse(&mut self, input: String);
}

