use utils::Solution;

impl Problem {
    pub fn new() -> Box<dyn Solution> {
        Box::new(Self::default())
    }
}

#[derive(Default)]
pub struct Problem {
    steps: Vec<String>,
}

impl Solution for Problem {
    fn parse(&mut self, input: String) {
        for step in input.trim_end().split(',') {
            self.steps.push(step.to_string());
        }
    }

    fn part1(&mut self) -> String {
        self.steps
            .iter()
            .map(|s| hash(s))
            .sum::<usize>()
            .to_string()
    }

    fn part2(&mut self) -> String {
        let mut boxes: Vec<Vec<(&str, u8)>> = Vec::with_capacity(256);
        for _ in 0..256 {
            boxes.push(Vec::new());
        }

        for step in &self.steps {
            let b: &mut Vec<(&str, u8)>;
            if let Some((label, value)) = step.split_once('=') {
                let h = hash(label);
                b = boxes.get_mut(h).unwrap();
                if let Some(index) = b.iter().position(|(lense, _)| *lense == label) {
                    b[index] = (label, value.parse::<u8>().unwrap());
                } else {
                    b.push((label, value.parse::<u8>().unwrap()));
                }
            } else {
                let label = &step[..step.len() - 1];
                let h = hash(label);
                b = boxes.get_mut(h).unwrap();
                if let Some(index) = b.iter().position(|(lense, _)| *lense == label) {
                    b.remove(index);
                }
            }
        }

        let mut result = 0;

        for b in 0..boxes.len() {
            for l in 0..boxes[b].len() {
                result += (b + 1) * (l + 1) * boxes[b][l].1 as usize;
            }
        }
        result.to_string()
    }
}

fn hash(msg: &str) -> usize {
    let mut val: u128 = 0;
    for ch in msg.as_bytes() {
        val = 17 * (val + *ch as u128);
    }
    (val % 256) as usize
}
