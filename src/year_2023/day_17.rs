use std::collections::HashMap;

use utils::Solution;

impl Problem {
    pub fn new() -> Box<dyn Solution> {
        Box::new(Self::default())
    }
}

#[derive(Default)]
pub struct Problem {
    blocks: Vec<Vec<u32>>,
}

impl Solution for Problem {
    fn parse(&mut self, input: String) {
        self.blocks = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|ch| ch.to_digit(10).unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
    }

    fn part1(&mut self) -> String {
        hottest_path(&self.blocks, false).to_string()
    }

    fn part2(&mut self) -> String {
        hottest_path(&self.blocks, true).to_string()
    }
}

fn hottest_path(blocks: &Vec<Vec<u32>>, ultra: bool) -> u32 {
    let max_turn: u32 = if ultra { 10 } else { 3 };
    let min_turn: u32 = if ultra { 3 } else { 0 };

    let mut open: Vec<((usize, usize), (isize, isize), u32, u32)> =
        vec![((0, 0), (1, 0), 0, 0), ((0, 0), (0, 1), 0, 0)];
    let mut visited: HashMap<(usize, usize), HashMap<((isize, isize), u32), u32>> = HashMap::new();

    let mut best = u32::MAX;
    let end = (blocks[0].len() - 1, blocks.len() - 1);

    'find_path: while let Some(((mut x, mut y), (dx, dy), turn, mut heat)) = open.pop() {
        x = x.wrapping_add_signed(dx);
        y = y.wrapping_add_signed(dy);

        if x >= blocks[0].len() || y >= blocks.len() {
            continue;
        }

        heat += blocks[y][x];

        if heat >= best {
            continue;
        }

        if (x, y) == end {
            if heat < best && turn >= min_turn {
                best = heat;
            }
            continue;
        }

        if let Some(prev) = visited.get_mut(&(x, y)) {
            if let Some(pheat) = prev.get(&((dx, dy), turn)) {
                if heat < *pheat {
                    prev.insert(((dx, dy), turn), heat);
                    for i in min_turn.max(turn + 1)..max_turn {
                        prev.insert(((dx, dy), i), heat);
                    }
                } else {
                    continue 'find_path;
                }
            } else {
                prev.insert(((dx, dy), turn), heat);
                for i in min_turn.max(turn + 1)..max_turn {
                    prev.insert(((dx, dy), i), heat);
                }
            }
        } else {
            visited.insert((x, y), HashMap::from([(((dx, dy), turn), heat)]));
        }

        if turn < max_turn - 1 {
            open.push(((x, y), (dx, dy), turn + 1, heat));
        }

        if turn >= min_turn {
            if dy == 1 || dx == -1 {
                open.push(((x, y), (-dy, dx), 0, heat));
                open.push(((x, y), (dy, -dx), 0, heat));
            } else {
                open.push(((x, y), (dy, -dx), 0, heat));
                open.push(((x, y), (-dy, dx), 0, heat));
            }
        }
    }

    best
}
