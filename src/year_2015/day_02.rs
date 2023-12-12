use utils::Solution;

impl Problem {
    pub fn new() -> Box<dyn Solution> {
        Box::new(Self::default())
    }
}

#[derive(Default)]
pub struct Problem {
    data: Vec<Present>,
}

impl Solution for Problem {
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
    fn parse(&mut self, input: String) {
        let data: Vec<Vec<usize>> = Vec::new();
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
