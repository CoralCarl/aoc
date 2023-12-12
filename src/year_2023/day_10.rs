use std::collections::HashSet;

use utils::parsing;
use utils::structure::Solution;

#[derive(Default)]
pub struct Day10 {
    grid: Vec<Vec<char>>,
    start: (usize, usize),
    pipes: HashSet<(usize, usize)>,
    s: char,
}

impl Day10 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Solution for Day10 {
    fn parse(&mut self) {
        let input = parsing::get_string(2023, 10);
        self.grid = input
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        'start_search: for y in 0..self.grid.len() {
            for x in 0..self.grid[y].len() {
                if self.grid[y][x] == 'S' {
                    self.start = (x, y);
                    break 'start_search;
                }
            }
        }
    }
    fn part1(&mut self) -> String {
        let mut visited = Vec::from([self.start]);
        let x_len = self.grid[0].len();
        let y_len = self.grid.len();

        let (x, y) = self.start;
        let mut d = (0, 0);
        for d_start in [(1, 0), (0, 1), (-1, 0)] {
            let x = x.wrapping_add_signed(d_start.0);
            let y = y.wrapping_add_signed(d_start.1);

            if x >= x_len || y >= y_len {
                println!("out of bounds {:?}", (x, y));
                continue;
            }
            if !(d_start == (1, 0) && "-7J".contains(self.grid[y][x])
                || d_start == (-1, 0) && "-LF".contains(self.grid[y][x])
                || d_start == (0, 1) && "|LJ".contains(self.grid[y][x]))
            {
                continue;
            }
            visited.push((x, y));
            d = d_start;
            break;
        }

        let first = d;
        let last;

        println!("{:?} {:?}", d, visited);

        'find_pipes: loop {
            let &(x, y) = visited.last().unwrap();
            d = next_direction(d, self.grid[y][x]);

            let x = x.wrapping_add_signed(d.0);
            let y = y.wrapping_add_signed(d.1);

            if self.grid[y][x] == 'S' {
                last = d;
                break 'find_pipes;
            }
            visited.push((x, y));
        }

        self.pipes = HashSet::from_iter(visited);
        self.s = match (first, last) {
            ((1,0), (-1,0)) => '-',
            ((1,0), (0,1)) => 'L',
            ((1,0), (0,-1)) => 'F',
            ((0,1), (1,0)) => '7',
            ((0,1), (0,-1)) => '|',
            ((-1,0), (0,1)) => 'J',
            _ => {dbg!(first, last); panic!();},
        };

        format!("{}", (self.pipes.len()) / 2)
    }
    fn part2(&mut self) -> String {
        let mut enclosed: HashSet<(usize, usize)> = HashSet::new();

        let mut inside = false;
        let mut entry = '|';
        for (y, row) in self.grid.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if self.pipes.contains(&(x, y)) {
                    let mut tile = *tile;
                    if tile == 'S' {
                        tile = self.s;
                    }

                    if "|LF".contains(tile)
                        || tile == 'J' && entry == 'L'
                        || tile == '7' && entry == 'F'
                    {
                        inside = !inside;
                        entry = tile;
                    }
                } else if inside {
                    enclosed.insert((x, y));
                }
            }
        }

        for (y, row) in self.grid.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if self.pipes.contains(&(x, y)) {
                    print!("█");
                } else if enclosed.contains(&(x, y)) {
                    print!("O");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        format!("{}", enclosed.len())
    }
}

fn next_direction(d_in: (isize, isize), c: char) -> (isize, isize) {
    match d_in {
        (1, 0) => match c {
            '-' => d_in,
            '7' => (0, 1),
            'J' => (0, -1),
            _ => panic!("invalid node"),
        },
        (-1, 0) => match c {
            '-' => d_in,
            'F' => (0, 1),
            'L' => (0, -1),
            _ => panic!("invalid node"),
        },
        (0, 1) => match c {
            '|' => d_in,
            'L' => (1, 0),
            'J' => (-1, 0),
            _ => panic!("invalid node"),
        },
        (0, -1) => match c {
            '|' => d_in,
            'F' => (1, 0),
            '7' => (-1, 0),
            _ => panic!("invalid node"),
        },
        _ => panic!("invalid direction"),
    }
}
