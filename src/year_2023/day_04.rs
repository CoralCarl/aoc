use utils::input;
use utils::structure::Solution;

#[derive(Default)]
pub struct Day04 {
    cards: Vec<u32>,
}

impl Day04 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Solution for Day04 {
    fn part1(&mut self) -> String {
        let mut result = 0;
        for &card in &self.cards {
            if card > 0 {
                result += u32::pow(2, card - 1);
            }
        }
        format!("{}", result)
    }
    fn part2(&mut self) -> String {
        let mut cards = vec![1_u32; self.cards.len()];

        for i in 0..cards.len() {
            let last_card = cards.len().min(i + self.cards[i] as usize);
            for j in i..last_card {
                cards[j+1] += cards[i];
            }
        }

        format!("{}", cards.iter().sum::<u32>())
    }
    fn parse(&mut self) {
        for line in input::to_lines(2023, 04) {
            let (_, line) = line.split_once(": ").unwrap();
            let (left, right) = line.split_once(" | ").unwrap();

            let winners: Vec<u32> = left
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect();
            let chosen: Vec<u32> = right
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect();

            let mut hit = 0;
            for num in &chosen {
                if winners.contains(&num) {
                    hit += 1;
                }
            }

            self.cards.push(hit);
        }
    }
}
