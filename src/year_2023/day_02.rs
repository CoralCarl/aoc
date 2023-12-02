use utils::parsing;
use utils::structure::Solution;

#[derive(Default)]
pub struct Day02 {
    games: Vec<Game>,
}

impl Day02 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Solution for Day02 {
    fn part1(&mut self) -> Vec<String> {
        let mut result = 0;

        'outer: for (i, game) in self.games.iter().enumerate() {
            for set in game.sets.iter() {
                if 12 < set.red || 13 < set.green || 14 < set.blue {
                    continue 'outer;
                }
            }
            result += i + 1;
        }

        vec![format!("{}", result)]
    }
    fn part2(&mut self) -> Vec<String> {
        let mut result = 0;

        for game in self.games.iter() {
            let mut max_set = Set {
                blue: 0,
                green: 0,
                red: 0,
            };

            for set in game.sets.iter() {
                if set.blue > max_set.blue {
                    max_set.blue = set.blue;
                }
                if set.red > max_set.red {
                    max_set.red = set.red;
                }
                if set.green > max_set.green {
                    max_set.green = set.green;
                }
            }

            result += max_set.blue * max_set.green * max_set.red;
        }

        vec![format!("{}", result)]
    }
    fn parse(&mut self) {
        let lines = parsing::get_lines(2023, 02);
        let mut games: Vec<Game> = Vec::new();

        for line in lines.iter() {
            let spl: Vec<_> = line
                .split(|c| c == ':' || c == ';')
                .map(|s| s.trim().split(", ").collect::<Vec<_>>())
                .collect();

            let mut sets: Vec<Set> = Vec::new();

            for set in spl[1..].iter() {
                let mut new = Set {
                    blue: 0,
                    green: 0,
                    red: 0,
                };

                for cube in set.iter() {
                    let (count, color) = cube.split_once(' ').unwrap();
                    match color {
                        "green" => new.green = count.parse().unwrap(),
                        "red" => new.red = count.parse().unwrap(),
                        "blue" => new.blue = count.parse().unwrap(),
                        _ => panic!("invalid input"),
                    }
                }

                sets.push(new);
            }
            games.push(Game { sets });
        }
        self.games = games;
    }
}

#[derive(Debug)]
struct Game {
    sets: Vec<Set>,
}

#[derive(Debug)]
struct Set {
    blue: usize,
    green: usize,
    red: usize,
}
