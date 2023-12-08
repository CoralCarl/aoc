use utils::parsing;
use utils::structure::Solution;

#[derive(Default)]
pub struct Day07 {
    set: Vec<(Hand, u32)>,
}

impl Day07 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Solution for Day07 {
    fn part1(&mut self) -> String {
        self.set.sort_by(|a, b| a.0.value().cmp(&b.0.value()));

        let mut result = 0;
        for (i, (_hand, bet)) in self.set.iter().enumerate() {
            result += bet * (i as u32 + 1);
        }

        format!("{}", result)
    }
    fn part2(&mut self) -> String {
        self.set
            .sort_by(|a, b| a.0.j_value().cmp(&b.0.j_value()));

        let mut result = 0;
        for (i, (_hand, bet)) in self.set.iter().enumerate() {
            result += bet * (i as u32 + 1);
        }

        format!("{}", result)
    }
    fn parse(&mut self) {
        for line in parsing::get_lines(2023, 07) {
            let (s, b) = line.split_once(' ').unwrap();
            self.set.push((Hand::new(s), b.parse::<u32>().unwrap()));
        }
    }
}

#[derive(Debug)]
struct Hand {
    cards: [u8; 5],
}

impl Hand {
    fn new(s: &str) -> Self {
        let mut cards = [0u8; 5];

        for (i, c) in s.chars().enumerate() {
            cards[i] = match c {
                '2'..='9' => c as u8 - '0' as u8,
                'A' | 'K' | 'Q' | 'J' | 'T' => "TJQKA".find(c).unwrap() as u8 + 10,
                _ => panic!("invalid input"),
            };
        }

        Self { cards }
    }

    fn value(&self) -> u32 {
        let mut value = 0u32;

        let mut card_count = [0u8; 13];
        for &card in &self.cards {
            card_count[card as usize - 2] += 1;
        }

        let mut best = 0;
        let mut pairs = 0;

        for i in 0..card_count.len() {
            if card_count[i] > card_count[best] {
                best = i;
            }
            if card_count[i] == 2 {
                pairs += 1;
            }
        }

        value += Hand::combo_value(card_count[best], pairs) as u32;

        for &card in &self.cards {
            value = value << 4;
            value += card as u32;
        }

        value
    }

    fn j_value(&self) -> u32 {
        let mut value = 0u32;

        let mut card_count = [0u8; 13];
        for &card in &self.cards {
            card_count[card as usize - 2] += 1;
        }

        let mut best = 0;

        for i in 1..card_count.len() {
            if card_count[i] > card_count[best] && i != 9 {
                best = i;
            }
        }

        card_count[best] += card_count[9];
        card_count[9] = 0;

        let mut pairs = 0;

        for count in card_count {
            if count == 2 {
                pairs += 1;
            }
        }

        value += Hand::combo_value(card_count[best], pairs) as u32;

        for &card in &self.cards {
            value = value << 4;
            value += if card != 11 { card } else { 0 } as u32;
        }

        value
    }

    fn combo_value(best: u8, pairs: u8) -> u8 {
        match best {
            5 => 6,
            4 => 5,
            3 => 3 + pairs,
            2 => pairs,
            _ => 0,
        }
    }
}
