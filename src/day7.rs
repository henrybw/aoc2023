use itertools::Itertools;
use std::cmp::Ordering;

fn example_input() -> String {
    String::from_utf8_lossy(include_bytes!("day7_example.txt")).to_string()
}

// from weakest to strongest
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl From<&Vec<char>> for HandType {
    fn from(hand: &Vec<char>) -> HandType {
        assert_eq!(hand.len(), 5);
        let counts = hand.iter().counts();
        let max = *counts.values().max().unwrap();
        if counts.len() == 1 {
            HandType::FiveOfAKind
        } else if counts.len() == 2 {
            if max == 4 {
                HandType::FourOfAKind
            } else {
                assert_eq!(max, 3);
                HandType::FullHouse
            }
        } else if counts.len() == 3 {
            if max == 3 {
                HandType::ThreeOfAKind
            } else {
                assert_eq!(max, 2);
                HandType::TwoPair
            }
        } else if counts.len() == 4 {
            HandType::OnePair
        } else {
            HandType::HighCard
        }
    }
}

impl From<&str> for HandType {
    fn from(hand: &str) -> HandType {
        Self::from(&hand.chars().collect_vec())
    }
}

fn card_weight(card: char) -> u32 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        '2'..='9' => card.to_digit(10).unwrap(),
        _ => panic!("unexpected card value {card}"),
    }
}

fn total_winnings(input: &str) -> usize {
    input
        .lines()
        .map(str::split_whitespace)
        .filter_map(|e| e.collect_tuple::<(&str, &str)>())
        .map(|(hand, bid)| {
            let hand = hand.chars().collect_vec();
            (HandType::from(&hand), hand, bid.parse::<usize>().unwrap())
        })
        .sorted_by(|a, b| {
            let (hand_type_a, hand_a, _) = a;
            let (hand_type_b, hand_b, _) = b;
            let cmp = Ord::cmp(&hand_type_a, &hand_type_b);
            if cmp != Ordering::Equal {
                return cmp;
            }
            for (&card_a, &card_b) in hand_a.iter().zip(hand_b.iter()) {
                let wcmp = Ord::cmp(&card_weight(card_a), &card_weight(card_b));
                if wcmp != Ordering::Equal {
                    return wcmp;
                }
            }
            Ordering::Equal
        })
        .enumerate()
        .fold(0, |acc, rank_hand_bid| {
            let (rank, (_, _, bid)) = rank_hand_bid;
            acc + bid * (rank + 1)
        })
}

pub fn part1(input: Option<String>) -> u32 {
    let input = input.unwrap_or_else(example_input);
    total_winnings(&input) as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_weight() {
        assert_eq!(card_weight('2'), 2);
        assert_eq!(card_weight('9'), 9);
        assert_eq!(card_weight('T'), 10);
        assert_eq!(card_weight('A'), 14);
    }

    #[test]
    fn test_hand_type() {
        assert_eq!(HandType::from("AAAAA"), HandType::FiveOfAKind);
        assert_eq!(HandType::from("AA8AA"), HandType::FourOfAKind);
        assert_eq!(HandType::from("23332"), HandType::FullHouse);
        assert_eq!(HandType::from("TTT98"), HandType::ThreeOfAKind);
        assert_eq!(HandType::from("23432"), HandType::TwoPair);
        assert_eq!(HandType::from("A23A4"), HandType::OnePair);
        assert_eq!(HandType::from("23456"), HandType::HighCard);
    }

    #[test]
    fn example_part1() {
        assert_eq!(part1(None), 6440);
    }
}
