use utils::structure::Solution;
use utils::parsing;

#[derive(Default)]
pub struct Day02 {
    data: Vec<Present>,
}

impl Day02 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Solution for Day02 {
    fn part1(&mut self) -> String {
        let mut paper = 0;
        
        for present in self.data.iter() {
            paper += present.surface() + present.slack();
        }
        
        format!("{}", paper)
        
    }
    fn part2(&mut self) -> String {
        let mut ribbon = 0;

        for present in self.data.iter() {
            ribbon += present.ribbon() + present.bow();
        }

        format!("{}", ribbon)
    }
    fn parse(&mut self) {
        let data: Vec<Vec<usize>> = parsing::get_numbers(2015, 2, "x");
        for mut d in data.into_iter(){
            d.sort();
            self.data.push(Present::new(d[0], d[1], d[2]));
        }
    }
}

struct Present {
    length: usize,
    width: usize,
    height: usize,
}

impl Present {
    fn new(length: usize, width: usize, height: usize) -> Self {
        Self {
            length,
            width,
            height,
        }
    }

    fn surface(&self) -> usize {
        2*(self.length*(self.width+self.height) + self.width*self.height)
    }

    fn slack(&self) -> usize {
        self.length * self.width
    }

    fn ribbon(&self) -> usize {
        2 * (self.length + self.width)
    }

    fn bow(&self) -> usize {
        self.length * self.width * self.height
    }
}
