use std::{collections::{HashSet, VecDeque}, fmt::Debug};

use regex::Regex;

#[derive(Clone)]
struct Card {
    number: u32,
    winning: HashSet<u32>,
    actual: HashSet<u32>,
}

impl Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("<Card {:?}>", self.number))
    }
}

impl Card {
    fn score(&self) -> i32 {
        let matching = self.winning_numbers();
        if matching.is_empty() {
            return 0;
        }

        let exp = matching.len() as u32;

        let base: i32 = 2;
        base.pow(exp - 1)
    }

    fn winning_numbers(&self) -> HashSet<u32> {
        return self.winning.intersection(&self.actual).cloned().collect();
    }

    fn is_winning(&self) -> bool {
        !self.winning_numbers().is_empty()
    }

    fn from_str_with(str: &str, number: u32) -> Option<Self> {
        let card = Self::from_str(str);
        if card.number == number {
            Some(card)
        } else {
            None
        }
    }

    fn from_str(str: &str) -> Self {
        let pattern = r"Card\s*(\d+)\s*:\s*";
        let re = Regex::new(pattern).unwrap();

        let number = to_num(re.captures(str).unwrap_or_else(|| panic!("capture haystack `{str}` failed")).get(1).expect("to have captures").as_str()).expect("number to be a valid number");

        let parts = re.split(str).collect::<Vec<_>>();
        let numbers_str = parts[1];
        let numbers_parts = numbers_str.split(" | ").collect::<Vec<_>>();

        let re = Regex::new(r"\s+").unwrap();
        let winning = re
            .split(numbers_parts[0])
            .filter_map(to_num)
            .collect::<HashSet<_>>();
        let actual = re
            .split(numbers_parts[1])
            .filter_map(to_num)
            .collect::<HashSet<_>>();

        Self {
            number,
            winning,
            actual,
        }
    }
}

pub fn execute(input: &str) -> i32 {
    let mut lines_to_parse = Vec::<&str>::new();
    let mut lines = input.lines().collect::<Vec<_>>();
    
    let mut cards = lines.into_iter().filter(|s| !s.is_empty()).map(Card::from_str).collect::<Vec<_>>();
    let mut card_copies: Vec<Card> = Vec::new();
    let mut final_cards = Vec::<Card>::new();

    add_to_cards(&mut final_cards, cards);

    // final_cards.iter().for_each(|card| {
    //     println!("F card: {:?}", card);
    // });

    // add_to_cards(&mut cards, &mut all_lines, 1);
    // let winning_cards = cards.iter().filter(|c| c.is_winning()).collect::<Vec<_>>();

    // for (i, line) in input.lines().enumerate() {
    //     if line.is_empty() {
    //         continue;
    //     }

    //     let card = Card::from_str(line);

    //     let winning_count = card.winning_numbers().len();
    //     if winning_count > 0 {
    //         println!("Card {card:?} is winning. Grabbing next {winning_count} card(s)", card=card.number, winning_count=winning_count);
    //     }

    //     let start = i + 1;
    //     let lines_to_add = all_lines.clone()[start..(start + winning_count - 1)].to_vec();

    //     cards.push(card);
    //     // add_to_cards(&mut cards, lines_to_add, card, lines_to_add);

    //     println!("lines_to_add: {:?}\n\n", lines_to_add);
    // }

    // for (i, card) in cards.iter().enumerate() {
    //     let grab_next_cards = card.winning_numbers().len() as u32;
    //     if grab_next_cards == 0 {
    //         continue;
    //     }

    //     let start = i + 1;
    //     println!("Card {card:?} is winning. Grabbing next {grab_next_cards} card(s) starting from {start} til {end}", card=card.number, grab_next_cards=grab_next_cards, start=start, end=(start + ((grab_next_cards - 1) as usize)));
    //     let cards_to_add = cards.clone()[start..(start + (grab_next_cards - 1) as usize)].to_vec();
    //     println!("cards_to_add: {:?}", cards_to_add);
    //     winning_cards.extend(cards_to_add);

    //     // (0..grab_next_cards).for_each(|_| {
    //     //     winning_cards.push(card.clone());
    //     // });
    // }

    // println!("winning_cards: {:?}", winning_cards.len());

    final_cards.len() as i32
}

fn add_to_cards_with_copy(final_cards: &mut Vec<Card>, cards: Vec<Card>, init: bool) {
    if cards.is_empty() && !init {
        return;
    }

    let mut copied_cards = Vec::<Card>::new();

    for (i, card) in cards.iter().enumerate() {
        let winning_count = card.winning_numbers().len();
        if winning_count == 0 {
            continue;
        }

        println!("card {:?} is winning. Grabbing next {} card(s)", card, winning_count);

        let card_indices_to_add = i + 1..i + 1 + winning_count;

        println!("range: {:?}", card_indices_to_add);

        for j in card_indices_to_add {
            let card = cards.get(j);
            if let Some(card) = card {
                println!("card found at index {}", j);
                copied_cards.push(card.clone());
            } else {
                println!("card not found at index {}", j);
            }
        }

        final_cards.push(card.clone());
        
        // add_to_cards(final_cards, copied_cards);
    }

    // add_to_cards_with_copy(final_cards, cards, copied_cards, false);

}

fn add_to_cards(final_cards: &mut Vec<Card>, cards: Vec<Card>) {
    if cards.is_empty() {
        return;
    }

    let mut copies = Vec::<Card>::new();

    for (i, card) in cards.iter().enumerate() {
        let n = card.winning_numbers().len();
        if n == 0 {
            // println!("card {:?} is not winning. Adding to final_cards", card);
            final_cards.push(card.clone());
            continue;
        }

        // println!("card {:?} is winning. Grabbing next {} card(s)", card, n);
        let min_num = card.number + 1;
        let max_num = card.number + n as u32;
        let acceptable_card_numbers = min_num..max_num + 1;

        for acceptable_card_number in acceptable_card_numbers.clone() {
            let card = cards.iter().find(|c| c.number == acceptable_card_number);
            if let Some(card) = card {
                copies.push(card.clone());
            }
        }

        // println!("acceptable_card_numbers: {:?}", acceptable_card_numbers.collect::<Vec<_>>());

        // for card in next_n_cards {
        //     copies.push(card.clone());
        // }

        final_cards.push(card.clone());
    }

    // add_to_cards_with_copy(final_cards, cards, card_copies, true);

    // println!("copies: {:?}", copies);
    // println!("final_cards: {:?}", final_cards);

    add_to_cards(final_cards, copies);

    // for card in cards.iter() {
    //     let winning_count = card.winning_numbers().len();
    //     let card_numbers_to_add = card.number + 1..card.number + winning_count as u32;

    //     println!("card: {:?} (to add: {:?})", card, card_numbers_to_add);

    //     for number in card_numbers_to_add {
    //         let card = cards.get((number as usize) - 1);
    //         if let Some(card) = card {
    //             if card.is_winning() {
    //                 final_cards.push(card.clone());
    //             }
    //         }
    //     }
    // }

    // if lines.is_empty() {
    //     return;
    // }

    // let mut matching_card_indices = Vec::<usize>::new();
    // for (i, line) in lines.iter().enumerate() {
    //     let card = Card::from_str(line);
    //     if card.number == start_at {
    //         matching_card_indices.push(i);
    //     }
    // }

    // for i in matching_card_indices.iter().rev() {
    //     let line = lines.remove(*i);
    //     let card = Card::from_str(line);
    // }

    // let current_line = lines.pop_front().unwrap();
    // let card = Card::from_str_with(current_line, start_at);
    // println!("card: {:?}", card);
    
    // let grab_next_cards = card.winning_numbers().len();
    // // let mut new_lines = VecDeque::<&str>::new();
    // for i in 0..grab_next_cards {
    //     if let Some(line) = lines.get(i) {
    //         lines.push_back(line);
    //     }
    // }

    // if card.is_winning() {
        // }
        
        // add_to_cards(cards, lines, start_at + 1);
        // cards.push(card);
}

// pub fn add_winning_card(mut cards: Vec<Card>, card: Card, times_left: u32) {

// }

fn to_num(s: &str) -> Option<u32> {
    s.parse::<u32>().ok()
}
