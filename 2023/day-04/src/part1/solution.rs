use std::collections::HashSet;

use regex::Regex;

struct Card {
    winning: HashSet<u32>,
    actual: HashSet<u32>,
}

impl Card {
    fn score(&self) -> i32 {
        let matching = self.winning.intersection(&self.actual).collect::<HashSet<_>>();
        if matching.is_empty() {
            return 0;
        }

        let exp = matching.len() as u32;

        let base: i32 = 2;
        base.pow(exp - 1)
    }
}

pub fn execute(input: &str) -> i32 {
    let mut cards = Vec::<Card>::new();

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let pattern = r"Card\s+(\d+)\s*:\s*";
        let re = Regex::new(pattern).unwrap();
        
        let parts = re.split(line).collect::<Vec<_>>();
        let numbers_str = parts[1];
        let numbers_parts = numbers_str.split(" | ").collect::<Vec<_>>();

        let re = Regex::new(r"\s+").unwrap();
        let winning = re.split(numbers_parts[0]).filter_map(to_num).collect::<HashSet<_>>();
        let actual = re.split(numbers_parts[1]).filter_map(to_num).collect::<HashSet<_>>();

        let card = Card {
            winning,
            actual,
        };

        cards.push(card);
    }
    
    cards.iter().map(Card::score).sum()
}

fn to_num(s: &str) -> Option<u32> {
    s.parse::<u32>().ok()
}
