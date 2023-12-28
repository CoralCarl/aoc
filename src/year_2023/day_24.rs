#[derive(Debug)]
struct Hailstone {
    position: (f64, f64, f64),
    velocity: (f64, f64, f64),
}

impl Hailstone {
    fn parallel_2d(&self, other: &Hailstone) -> bool {
        let factor = (
            self.velocity.0 / other.velocity.0,
            self.velocity.1 / other.velocity.1,
        );
        factor.0 == factor.1
    }

    fn intersect_2d(&self, other: &Hailstone) -> (f64, f64) {
        let one = (
            self.position.0 / self.velocity.0,
            1.0,
            other.position.0 / self.velocity.0,
            other.velocity.0 / self.velocity.0,
        );
        let two = (
            self.position.1 / self.velocity.1,
            1.0,
            other.position.1 / self.velocity.1,
            other.velocity.1 / self.velocity.1,
        );
        let mu = ((one.0 - two.0) - (one.2 - two.2)) / (one.3 - two.3);

        let x = other.position.0 + mu * other.velocity.0;
        let y = other.position.1 + mu * other.velocity.1;

        (x, y)
    }

    fn is_past(&self, point: &(f64, f64)) -> bool {
        if self.position.0 < point.0 && self.velocity.0 < 0.0
            || self.position.1 < point.1 && self.velocity.1 < 0.0
            || self.position.0 > point.0 && self.velocity.0 > 0.0
            || self.position.1 > point.1 && self.velocity.1 > 0.0
        {
            return true;
        }
        false
    }
}

fn parse(input: &[u8]) -> Vec<Hailstone> {
    let mut hailstones = Vec::new();

    let mut nums = vec![0.0];
    let mut negative = false;

    for c in input {
        match c {
            b',' | b'@' => {
                negative = false;
                nums.push(0.0);
            }
            b' ' => (),
            b'-' => {
                negative = true;
            }
            b'\n' => {
                hailstones.push(Hailstone {
                    position: (nums[0], nums[1], nums[2]),
                    velocity: (nums[3], nums[4], nums[5]),
                });
                negative = false;
                nums = vec![0.0];
            }
            _ => {
                *nums.last_mut().unwrap() *= 10.0;
                if negative {
                    *nums.last_mut().unwrap() -= (c - b'0') as f64;
                } else {
                    *nums.last_mut().unwrap() += (c - b'0') as f64;
                }
            }
        }
    }

    hailstones
}

const MIN: f64 = 200000000000000.0;
const MAX: f64 = 400000000000000.0;

pub fn part1(input: &[u8]) -> String {
    let hailstones = parse(input);
    let mut count = 0;
    for (i, hailstone) in hailstones.iter().enumerate() {
        for other in hailstones.iter().skip(i + 1) {
            if hailstone.parallel_2d(&other) {
                continue;
            }

            let intersect = hailstone.intersect_2d(&other);
            if hailstone.is_past(&intersect) || other.is_past(&intersect) {
                continue;
            }

            if intersect.0 >= MIN && intersect.0 <= MAX && intersect.1 >= MIN && intersect.1 <= MAX
            {
                count += 1;
            }
        }
    }
    count.to_string()
}

pub fn part2(input: &[u8]) -> String {
    "I did not solve this".to_string()
}
