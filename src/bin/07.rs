use std::{collections::HashMap, str::FromStr};

use advent_of_code::ParseError;

advent_of_code::solution!(7);

#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Hash)]
struct Card {
    value: u32,
}

impl std::fmt::Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self.value {
            14 => "A",
            13 => "K",
            12 => "Q",
            11 => "J",
            10 => "T",
            9 => "9",
            8 => "8",
            7 => "7",
            6 => "6",
            5 => "5",
            4 => "4",
            3 => "3",
            2 => "2",
            _ => panic!("unexpected vard value {}", self.value),
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    fn get_type(&self) -> HandType {
        let mut histogram = HashMap::new();
        for c in self.cards.iter() {
            histogram
                .entry(c)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
        let max_card = histogram.iter().max_by_key(|(_k, v)| *v);
        match max_card {
            Some((_, 5)) => HandType::FiveOfAKind,
            Some((_, 4)) => HandType::FourOfAKind,
            Some((card, 3)) => match histogram.iter().find(|(c, q)| *c != card && **q == 2) {
                Some((_, 2)) => HandType::FullHouse,
                _ => HandType::ThreeOfAKind,
            },
            Some((card, 2)) => match histogram.iter().find(|(c, q)| *c != card && **q == 2) {
                Some((_, 2)) => HandType::TwoPair,
                _ => HandType::OnePair,
            },
            Some((_, 1)) => HandType::HighCard,
            Some((_, _)) => panic!("too many equal cards"),
            None => panic!("empty vector!"),
        }
    }
}

impl std::fmt::Debug for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: String = self.cards.iter().map(|c| format!("{c:?}")).collect();
        write!(f, "{}", s)
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let a = self.get_type();
        let b = other.get_type();
        match a.partial_cmp(&b) {
            Some(std::cmp::Ordering::Equal) => self.cards.partial_cmp(&other.cards),
            res => res,
        }
    }
}

impl FromStr for Hand {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_hand(s).ok_or(ParseError)
    }
}

fn parse_hand(input: &str) -> Option<Hand> {
    if input.len() != 5 {
        return None;
    }
    let cards = input
        .chars()
        .map(|c| {
            let value = match c {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 11,
                'T' => 10,
                '9' => 9,
                '8' => 8,
                '7' => 7,
                '6' => 6,
                '5' => 5,
                '4' => 4,
                '3' => 3,
                '2' => 2,
                _ => panic!("unexpected card: {}", c),
            };
            Card { value }
        })
        .collect();
    Some(Hand { cards })
}

type Bid = u32;

fn parse(input: &str) -> Vec<(Hand, Bid)> {
    input
        .lines()
        .flat_map(|l| {
            let (hand, bid) = l.split_once(' ')?;
            Some((parse_hand(hand)?, bid.parse().ok()?))
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut games = parse(input);
    games.sort_unstable_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap());
    Some(
        games
            .iter()
            .enumerate()
            .map(|(ix, (_, bid))| (ix as u32 + 1) * bid)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
