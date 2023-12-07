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

impl HandType {
    fn from_with_jokers(hand: &Vec<char>) -> HandType {
        let mut counts = hand.iter().counts();
        let jokers = counts.remove(&'J');
        if let Some(jokers) = jokers {
            if jokers == 5 {
                HandType::FiveOfAKind
            } else {
                // replace jokers with the highest, most frequent card
                let substitute = **counts
                    .iter()
                    .max_by(|a, b| {
                        let (&&card_a, count_a) = *a;
                        let (&&card_b, count_b) = *b;
                        let cmp = count_a.cmp(count_b);
                        if cmp == Ordering::Equal {
                            card_weight(card_a, true)
                                .cmp(&card_weight(card_b, true))
                        } else {
                            cmp
                        }
                    })
                    .unwrap()
                    .0;
                let new_hand = hand
                    .iter()
                    .map(|&card| if card == 'J' { substitute } else { card })
                    .collect_vec();
                HandType::from(&new_hand)
            }
        } else {
            HandType::from(hand)
        }
    }
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

fn card_weight(card: char, jokers: bool) -> u32 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => {
            if jokers {
                1
            } else {
                11
            }
        }
        'T' => 10,
        '2'..='9' => card.to_digit(10).unwrap(),
        _ => panic!("unexpected card value {card}"),
    }
}

fn total_winnings(input: &str, jokers: bool) -> usize {
    input
        .lines()
        .map(str::split_whitespace)
        .filter_map(|e| e.collect_tuple::<(&str, &str)>())
        .map(|(hand, bid)| {
            let hand = hand.chars().collect_vec();
            let hand_type = if jokers {
                HandType::from_with_jokers(&hand)
            } else {
                HandType::from(&hand)
            };
            (hand_type, hand, bid.parse::<usize>().unwrap())
        })
        .sorted_by(|a, b| {
            let (hand_type_a, hand_a, _) = a;
            let (hand_type_b, hand_b, _) = b;
            let cmp = hand_type_a.cmp(hand_type_b);
            if cmp != Ordering::Equal {
                return cmp;
            }
            for (&card_a, &card_b) in hand_a.iter().zip(hand_b.iter()) {
                let wcmp = card_weight(card_a, jokers)
                    .cmp(&card_weight(card_b, jokers));
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
    total_winnings(&input, false) as u32
}

pub fn part2(input: Option<String>) -> u32 {
    let input = input.unwrap_or_else(example_input);
    total_winnings(&input, true) as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_weight() {
        assert_eq!(card_weight('2', false), 2);
        assert_eq!(card_weight('9', false), 9);
        assert_eq!(card_weight('T', false), 10);
        assert_eq!(card_weight('A', false), 14);
        assert_eq!(card_weight('J', false), 11);
        assert_eq!(card_weight('J', true), 1);
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
        assert_eq!(
            HandType::from_with_jokers(&"JJJJJ".chars().collect_vec()),
            HandType::FiveOfAKind
        );
        assert_eq!(
            HandType::from_with_jokers(&"KJJJJ".chars().collect_vec()),
            HandType::FiveOfAKind
        );
        assert_eq!(
            HandType::from_with_jokers(&"TTJJJ".chars().collect_vec()),
            HandType::FiveOfAKind
        );
        assert_eq!(
            HandType::from_with_jokers(&"KTJJJ".chars().collect_vec()),
            HandType::FourOfAKind
        );
        assert_eq!(
            HandType::from_with_jokers(&"KTJJT".chars().collect_vec()),
            HandType::FourOfAKind
        );
        assert_eq!(
            HandType::from_with_jokers(&"KTJJ9".chars().collect_vec()),
            HandType::ThreeOfAKind
        );
        assert_eq!(
            HandType::from_with_jokers(&"T55J5".chars().collect_vec()),
            HandType::FourOfAKind
        );
        assert_eq!(
            HandType::from_with_jokers(&"QQQJA".chars().collect_vec()),
            HandType::FourOfAKind
        );
        assert_eq!(
            HandType::from_with_jokers(&"5234J".chars().collect_vec()),
            HandType::OnePair
        );
        assert_eq!(
            HandType::from_with_jokers(&"5333J".chars().collect_vec()),
            HandType::FourOfAKind
        );
        assert_eq!(
            HandType::from_with_jokers(&"5233J".chars().collect_vec()),
            HandType::ThreeOfAKind
        );
        assert_eq!(
            HandType::from_with_jokers(&"5322J".chars().collect_vec()),
            HandType::ThreeOfAKind
        );
        assert_eq!(
            HandType::from_with_jokers(&"23J32".chars().collect_vec()),
            HandType::FullHouse
        );
    }

    #[test]
    fn example_part1() {
        assert_eq!(part1(None), 6440);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(None), 5905);
    }
}
