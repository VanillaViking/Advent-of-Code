use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();
    
    let cards: Vec<Card> = input.lines().enumerate().map(|(idx, line)| Card::build(line, idx)).collect();

    let cards_len = cards.len();

    println!("{}", cards.iter().map(|card| card.get_worth()).sum::<i32>());

    let total: usize = cards.iter().enumerate().map(|(idx, _card)| get_cards_won(&cards, idx)).sum();

    println!("{}", total + cards_len);
}


#[derive(Debug)]
struct Card {
    card_number: u32,
    winning_numbers: Vec<u32>,
    owned_numbers: Vec<u32>
}

impl Card {
    pub fn build(card_line: &str, line_idx: usize) -> Card {
        let (card_number_str, numbers_str) = card_line.split_once(":").unwrap();
        // let card_number: u32 = card_number_str.split_once(" ").unwrap().1.parse().unwrap();

        let (winning_numbers_str, owned_numbers_str) = numbers_str.split_once("|").unwrap();

        let winning_numbers: Vec<u32> = winning_numbers_str.split(" ").filter_map(|num_str| {
            num_str.parse::<u32>().ok()
        }).collect();

        let owned_numbers: Vec<u32> = owned_numbers_str.split(" ").filter_map(|num_str| {
            num_str.parse::<u32>().ok()
        }).collect();

        Card { card_number: u32::try_from(line_idx).unwrap() + 1, winning_numbers, owned_numbers }
    }

    pub fn get_worth(&self) -> i32 {
        let temp = self.winning_numbers.iter().filter(|winning_num| self.owned_numbers.contains(&winning_num)).count();

        let mut power = i32::try_from(temp).unwrap() - 1;

        if power >= 0 {
            return i32::pow(2, u32::try_from(power).unwrap_or(0))
        } else {
            return 0
        }
    }

}

fn get_cards_won(cards: &Vec<Card>, card_idx: usize) -> usize {
    let mut winnings = cards[card_idx].winning_numbers.iter().filter(|winning_num| cards[card_idx].owned_numbers.contains(&winning_num)).count();

    for n in 1..=winnings {
        winnings += get_cards_won(cards, card_idx+n)
    }

    return winnings
}
