use std::{fs, collections::HashMap};

#[derive(Debug)]
struct Card {
    label: char,
    strength: u32,
}
impl Card {
    fn new(char: char) -> Card {
        let strength = match char {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 1,
            'T' => 10,
            '9' => 9,
            '8' => 8,
            '7' => 7,
            '6' => 6,
            '5' => 5,
            '4' => 4,
            '3' => 3,
            '2' => 2,
            _ => panic!(),
        };

        Card { label: char, strength }
    }
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    strength: u64,
    bid: u32,
}


fn calculate_strength(cards: &Vec<Card>) -> u64 {
    let mut hand_type = 1;

    let mut card_occurences = HashMap::new();

    for card in cards.iter() {
        let count = card_occurences.entry(card.label).or_insert(0);
        *count +=1;
    }

    if card_occurences.contains_key(&'J') {
        let j_count = card_occurences.remove(&'J').unwrap();

        let max_count_char = card_occurences
            .iter()
            .max_by(|a, b| a.1.cmp(&b.1))
            .map(|(k, _v)| k).unwrap_or(&'J');

        *card_occurences.entry(max_count_char.to_owned()).or_insert(0) += j_count;
    }

    let counts: Vec<u32> = card_occurences.into_values().collect();

    if counts.contains(&5) {
        hand_type = 7;
    } else if counts.contains(&4) {
        hand_type = 6;
    } else if counts.contains(&3) {
        if counts.contains(&2) {
            hand_type = 5;
        } else {
            hand_type = 4;
        }
    } else if counts.contains(&2) {
        if counts.iter().filter(|&x| *x == 2).count() == 2 {
            hand_type = 3;
        } else {
            hand_type = 2;
        }
    }

    let strength_str = format!("{}{:0>2}{:0>2}{:0>2}{:0>2}{:0>2}", hand_type, cards[0].strength, cards[1].strength, cards[2].strength, cards[3].strength, cards[4].strength);

    strength_str.parse().unwrap()

}

impl Hand {
    pub fn new(input: &str) -> Hand {
        let (cards_str, bid_str) = input.split_once(" ").unwrap();

        let bid: u32 = bid_str.parse().unwrap();
        let cards: Vec<Card> = cards_str.chars().map(|char| {
            Card::new(char)
        }).collect();

        let strength = calculate_strength(&cards);

        Hand { cards, strength, bid }
    }

}

#[derive(Debug)]
struct Game {
    hands: Vec<Hand>
}

impl Game {
    fn parse_input(input: &str) -> Game {
        let mut hands: Vec<Hand> = input.lines().map(|line| {
            Hand::new(line)
        }).collect();

        hands.sort_by_key(|hand| hand.strength);

        Game { hands }
    }

    pub fn part1(&self) -> u32 {
        let mut total = 0;

        for (idx, hand) in self.hands.iter().enumerate() {
            total += (idx as u32 +1) * hand.bid;
        }

        total
    }
}


fn main() {
    let input = fs::read_to_string("input").unwrap();

    let game = Game::parse_input(&input);
    dbg!(&game);

    println!("{}", game.part1());
}
