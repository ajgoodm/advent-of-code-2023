use std::collections::{HashMap, HashSet};

use shared::input::AocBufReader;

fn main() {
    let result = part_1(AocBufReader::from_string("inputs/part_1.txt"));
    println!("part 1: {}", result);

    let result = part_2(AocBufReader::from_string("inputs/part_1.txt"));
    println!("part 2: {}", result);
}

fn part_1(reader: AocBufReader) -> usize {
    parse_input(reader)
        .into_iter()
        .map(|card| card.part_1_score())
        .sum()
}

fn part_2(reader: AocBufReader) -> usize {
    let cards = parse_input(reader);
    let mut card_counts: HashMap<usize, usize> = cards.iter().map(|card| (card.id, 1)).collect();

    for card in cards {
        let score = card.n_matches();
        let n_copies = *card_counts.get(&card.id).unwrap();

        for card_id in (card.id + 1)..=(card.id + score) {
            *card_counts.get_mut(&card_id).unwrap() += n_copies;
        }
    }

    card_counts.values().sum()
}

struct Card {
    id: usize,
    winning_numbers: HashSet<usize>,
    your_numbers: HashSet<usize>,
}

impl Card {
    fn matching_numbers(&self) -> HashSet<usize> {
        self.winning_numbers
            .intersection(&self.your_numbers)
            .cloned()
            .collect()
    }

    fn n_matches(&self) -> usize {
        self.matching_numbers().len()
    }

    fn part_1_score(&self) -> usize {
        let n_matches: u32 = self.matching_numbers().len().try_into().unwrap();
        if n_matches == 0 {
            0
        } else {
            2usize.pow(n_matches - 1)
        }
    }
}

fn parse_input(reader: AocBufReader) -> Vec<Card> {
    reader.into_iter().map(parse_line).collect()
}

fn parse_line(line: String) -> Card {
    let mut card_and_numbers = line.split(":");
    let card_str = card_and_numbers.next().unwrap().trim();
    let numbers_str = card_and_numbers.next().unwrap().trim();

    let card_id: usize = card_str
        .split(" ")
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();

    let mut numbers_split = numbers_str.split("|");
    let winning_numbers_str = numbers_split.next().unwrap().trim();
    let your_numbers_str = numbers_split.next().unwrap().trim();

    let winning_numbers: HashSet<usize> = winning_numbers_str
        .split_whitespace()
        .map(|number| number.parse::<usize>().unwrap())
        .collect();

    let your_numbers: HashSet<usize> = your_numbers_str
        .split_whitespace()
        .map(|number| number.parse::<usize>().unwrap())
        .collect();

    Card {
        id: card_id,
        winning_numbers: winning_numbers,
        your_numbers: your_numbers,
    }
}
