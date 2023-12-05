use std::collections::HashMap;

use utils::parsing;
use utils::structure::Solution;

#[derive(Default)]
pub struct Day05 {
    data: Vec<String>,
}

impl Day05 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Solution for Day05 {
    fn part1(&mut self) -> String {
        let mut nice = 0;
        'lines: for s in self.data.iter() {
            let mut vowels = 0;
            let mut twice = false;
            let mut prev = '?';

            let charstr = s.chars().collect::<Vec<char>>();

            for (i, &c) in charstr.iter().enumerate() {
                if vowels < 3 && ['a', 'e', 'i', 'u', 'o'].contains(&c) {
                    vowels += 1;
                }

                if !twice && c == *charstr.get(i + 1).unwrap_or(&' ') {
                    twice = true;
                }
                match prev {
                    'a' => {
                        if c == 'b' {
                            continue 'lines;
                        }
                    }
                    'c' => {
                        if c == 'd' {
                            continue 'lines;
                        }
                    }
                    'p' => {
                        if c == 'q' {
                            continue 'lines;
                        }
                    }
                    'x' => {
                        if c == 'y' {
                            continue 'lines;
                        }
                    }
                    _ => (),
                }
                prev = c;
            }
            if twice && vowels >= 3 {
                nice += 1;
            }
        }
        format!("{}", nice)
    }
    fn part2(&mut self) -> String {
        let mut nice = 0;
        'lines: for s in self.data.iter() {
            let s = s.chars().collect::<Vec<char>>();
            let mut pairs = HashMap::new();
            let mut found_overlap = false;
            let mut repeat = false;

            for (i, &c) in s.iter().enumerate() {
                if !found_overlap && i < s.len() - 1 {
                    let pair = [c, s[i + 1]];
                    if let Some(index) = pairs.get(&pair) {
                        if index + 1 < i {
                            found_overlap = true;
                        }
                    } else {
                        pairs.insert(pair, i);
                    }
                }
                if !repeat && i < s.len() - 2 && c == s[i + 2] {
                    repeat = true;
                }
                if repeat && found_overlap {
                    nice += 1;
                    continue 'lines;
                }
            }
            // println!("{:?}: {:?}",s, pairs);
        }

        format!("{}", nice)
    }
    fn parse(&mut self) {
        self.data = parsing::get_lines(2015, 05);
    }
}
