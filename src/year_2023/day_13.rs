use utils::parse;
use utils::Solution;

impl Problem {
    pub fn new() -> Box<dyn Solution> {
        Box::new(Self::default())
    }
}

#[derive(Default)]
pub struct Problem {
    grids: Vec<Vec<Vec<char>>>,
}

impl Solution for Problem {
    fn parse(&mut self, input: String) {
        for block in input.split("\n\n") {
            self.grids.push(parse::grid(block));
        }
    }

    fn part1(&mut self) -> String {
        self.grids
            .iter()
            .map(|g| reflection(g, 0))
            .sum::<usize>()
            .to_string()
    }

    fn part2(&mut self) -> String {
        self.grids
            .iter()
            .map(|g| reflection(g, 1))
            .sum::<usize>()
            .to_string()
    }
}

fn reflection(grid: &Vec<Vec<char>>, errors: usize) -> usize {
    if let Some(rows) = row_reflection(grid, errors) {
        return 100 * rows;
    } else {
        return col_reflection(grid, errors).unwrap();
    }
}

fn row_reflection(grid: &Vec<Vec<char>>, errors: usize) -> Option<usize> {
    'next_row: for row in 0..grid.len() - 1 {
        let mut current = 0;
        for (up, down) in (0..=row).rev().zip(row + 1..grid.len()) {
            for col in 0..grid[0].len() {
                if grid[up][col] != grid[down][col] {
                    if current < errors {
                        current += 1;
                    } else {
                        continue 'next_row;
                    }
                }
            }
        }
        if current == errors {
            return Some(row + 1);
        }
    }
    None
}

fn col_reflection(grid: &Vec<Vec<char>>, errors: usize) -> Option<usize> {
    'next_col: for col in 0..grid[0].len() - 1 {
        let mut current = 0;
        for (left, right) in (0..=col).rev().zip(col + 1..grid[0].len()) {
            for row in 0..grid.len() {
                if grid[row][left] != grid[row][right] {
                    if current < errors {
                        current += 1;
                    } else {
                        continue 'next_col;
                    }
                }
            }
        }
        if current == errors {
            return Some(col + 1);
        }
    }
    None
}
