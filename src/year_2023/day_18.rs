use utils::Solution;

impl Problem {
    pub fn new() -> Box<dyn Solution> {
        Box::new(Self::default())
    }
}

#[derive(Default)]
pub struct Problem {
    plan: Vec<(char, usize)>,
    correct_plan: Vec<(char, usize)>,
}

impl Solution for Problem {
    fn parse(&mut self, input: String) {
        for line in input.lines() {
            let mut spl = line.split_whitespace();
            self.plan.push((
                spl.next().unwrap().chars().next().unwrap(),
                spl.next().unwrap().parse::<usize>().unwrap(),
            ));
            let (d, v) = spl
                .next()
                .unwrap()
                .trim_matches(|ch| ch == '(' || ch == ')')[1..]
                .split_at(5);
            self.correct_plan.push((
                ['R', 'D','L','U'][v.parse::<usize>().unwrap()],
                usize::from_str_radix(d, 16).unwrap(),
            ));
        }
    }

    fn part1(&mut self) -> String {
        area(&self.plan).to_string()
    }

    fn part2(&mut self) -> String {
        area(&self.correct_plan).to_string()
    }
}

fn area(plan: &Vec<(char, usize)>) -> i64 {
    let mut vertices = Vec::from([(0, 0)]);

    let mut x = 0i64;
    let mut y = 0i64;

    for &(d, v) in plan {
        let d = match d {
            'R' => (1, 0),
            'D' => (0, 1),
            'L' => (-1, 0),
            'U' => (0, -1),
            _ => panic!(),
        };

        x += d.0 * v as i64;
        y += d.1 * v as i64;

        vertices.push((x, y));
    }

    vertices.push(vertices[0]);

    vertices
        .iter()
        .zip(vertices.iter().skip(1))
        .map(|(v1, v2)| v1.0 * v2.1 - v1.1 * v2.0 + (v2.0 - v1.0 + v2.1 - v1.1).abs())
        .sum::<i64>()
        / 2
        + 1
}
