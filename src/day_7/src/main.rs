use std::cmp::Ordering;
use std::collections::HashMap;

use once_cell::sync::Lazy;

use shared::input::AocBufReader;

static HAND_PRINTS: Lazy<HashMap<String, HandType>> = Lazy::new(|| {
    HashMap::from([
        ("5".to_string(), HandType::FiveOfAKind),
        ("41".to_string(), HandType::FourOfAKind),
        ("32".to_string(), HandType::FullHouse),
        ("311".to_string(), HandType::ThreeOfAKind),
        ("221".to_string(), HandType::TwoPair),
        ("2111".to_string(), HandType::OnePair),
        ("11111".to_string(), HandType::HighCard),
    ])
});

static TYPE_VALUE: Lazy<HashMap<char, char>> = Lazy::new(|| {
    HashMap::from([
        ('2', 'a'),
        ('3', 'b'),
        ('4', 'c'),
        ('5', 'd'),
        ('6', 'e'),
        ('7', 'f'),
        ('8', 'g'),
        ('9', 'h'),
        ('T', 'i'),
        ('J', 'j'),
        ('Q', 'k'),
        ('K', 'l'),
        ('A', 'm'),
    ])
});

static TYPE_VALUE_PART_2: Lazy<HashMap<char, char>> = Lazy::new(|| {
    HashMap::from([
        ('J', 'a'),
        ('2', 'b'),
        ('3', 'c'),
        ('4', 'd'),
        ('5', 'e'),
        ('6', 'f'),
        ('7', 'g'),
        ('8', 'h'),
        ('9', 'i'),
        ('T', 'j'),
        ('Q', 'k'),
        ('K', 'l'),
        ('A', 'm'),
    ])
});

fn main() {
    let result = part_1(AocBufReader::from_string("inputs/part_1.txt"));
    println!("part 1: {result}");

    let result = part_2(AocBufReader::from_string("inputs/part_1.txt"));
    println!("part 2: {result}");
}

fn part_1(reader: AocBufReader) -> usize {
    let mut hands_and_bids: Vec<(String, usize)> = reader
        .into_iter()
        .map(|line| {
            let mut split = line.split_whitespace();
            let hand = split.next().unwrap().to_string();
            let bid = split.next().unwrap().parse::<usize>().unwrap();
            (hand, bid)
        })
        .collect();

    hands_and_bids.sort_by(|a, b| {
        let (a_hand, _) = a;
        let (b_hand, _) = b;

        classify_hand_part_1(&a_hand)
            .relative_value()
            .cmp(&classify_hand_part_1(&b_hand).relative_value())
            .then_with(|| {
                let a_rank = a_hand
                    .chars()
                    .map(|c| TYPE_VALUE.get(&c).unwrap())
                    .collect::<String>();
                let b_rank = b_hand
                    .chars()
                    .map(|c| TYPE_VALUE.get(&c).unwrap())
                    .collect::<String>();

                a_rank.cmp(&b_rank)
            })
    });

    hands_and_bids
        .into_iter()
        .enumerate()
        .map(|(rank_minus_one, (_, bid))| (rank_minus_one + 1) * bid)
        .sum()
}

fn part_2(reader: AocBufReader) -> usize {
    let mut hands_and_bids: Vec<(String, usize)> = reader
        .into_iter()
        .map(|line| {
            let mut split = line.split_whitespace();
            let hand = split.next().unwrap().to_string();
            let bid = split.next().unwrap().parse::<usize>().unwrap();
            (hand, bid)
        })
        .collect();

    hands_and_bids.sort_by(|a, b| {
        let (a_hand, _) = a;
        let (b_hand, _) = b;

        classify_hand_part_2(&a_hand)
            .relative_value()
            .cmp(&classify_hand_part_2(&b_hand).relative_value())
            .then_with(|| {
                let a_rank = a_hand
                    .chars()
                    .map(|c| TYPE_VALUE_PART_2.get(&c).unwrap())
                    .collect::<String>();
                let b_rank = b_hand
                    .chars()
                    .map(|c| TYPE_VALUE_PART_2.get(&c).unwrap())
                    .collect::<String>();

                a_rank.cmp(&b_rank)
            })
    });

    hands_and_bids
        .into_iter()
        .enumerate()
        .map(|(rank_minus_one, (_, bid))| (rank_minus_one + 1) * bid)
        .sum()
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    fn relative_value(&self) -> usize {
        match self {
            HandType::FiveOfAKind => 6,
            HandType::FourOfAKind => 5,
            HandType::FullHouse => 4,
            HandType::ThreeOfAKind => 3,
            HandType::TwoPair => 2,
            HandType::OnePair => 1,
            HandType::HighCard => 0,
        }
    }
}

fn classify_hand_part_1(hand: &str) -> HandType {
    let mut char_count: HashMap<char, usize> = HashMap::new();
    for c in hand.chars() {
        *char_count.entry(c).or_insert(0) += 1;
    }
    let mut char_counts: Vec<usize> = char_count.values().map(|x| *x).collect();
    char_counts.sort_by(|a, b| b.cmp(a));
    HAND_PRINTS
        .get(
            &char_counts
                .into_iter()
                .map(|x| x.to_string())
                .collect::<String>(),
        )
        .expect("no hand type for char count {char_counts}")
        .clone()
}

fn classify_hand_part_2(hand: &str) -> HandType {
    let mut char_count: HashMap<char, usize> = HashMap::new();
    for c in hand.chars() {
        *char_count.entry(c).or_insert(0) += 1;
    }

    if char_count.len() == 1 {
        return HandType::FiveOfAKind;
    }

    let mut most_common_type = '!';
    let mut max_occurrences: usize = 0;
    for (c, count) in char_count.iter() {
        if *c != 'J' && *count > max_occurrences {
            most_common_type = *c;
            max_occurrences = *count;
        }
    }

    let n_jacks = match char_count.get(&'J') {
        Some(count) => *count,
        None => 0,
    };
    if n_jacks > 0 {
        char_count.remove(&'J');
        *char_count.get_mut(&most_common_type).unwrap() += n_jacks;
    }

    let mut char_counts: Vec<usize> = char_count.values().map(|x| *x).collect();
    char_counts.sort_by(|a, b| b.cmp(a));
    HAND_PRINTS
        .get(
            &char_counts
                .into_iter()
                .map(|x| x.to_string())
                .collect::<String>(),
        )
        .expect("no hand type for char count {char_counts}")
        .clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classify_hand_part_1() {
        assert_eq!(classify_hand_part_1("AAAAA"), HandType::FiveOfAKind);
        assert_eq!(classify_hand_part_1("TTT98"), HandType::ThreeOfAKind);
        assert_eq!(classify_hand_part_1("23456"), HandType::HighCard);
    }

    #[test]
    fn test_classify_hand_part_2() {
        assert_eq!(classify_hand_part_2("32T3K"), HandType::OnePair);
        assert_eq!(classify_hand_part_2("T55J5"), HandType::FourOfAKind);
        assert_eq!(classify_hand_part_2("QQQJA"), HandType::FourOfAKind);
        assert_eq!(classify_hand_part_2("KTJJT"), HandType::FourOfAKind);
    }
}
