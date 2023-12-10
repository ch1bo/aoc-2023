use std::{cmp::Ordering, collections::HashMap, str::FromStr};

use advent_of_code::ParseError;

advent_of_code::solution!(7);

type Card = char;

// #[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Hash)]
// struct Card {
//     value: u32,
// }

// impl std::fmt::Debug for Card {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let s = match self.value {
//             14 => "A",
//             13 => "K",
//             12 => "Q",
//             10 => "T",
//             9 => "9",
//             8 => "8",
//             7 => "7",
//             6 => "6",
//             5 => "5",
//             4 => "4",
//             3 => "3",
//             2 => "2",
//             1 => "J", // TODO: combine part1/2
//             _ => panic!("unexpected vard value {}", self.value),
//         };
//         write!(f, "{}", s)
//     }
// }

fn fold_cmp_card(acc: Ordering, (a, b): (&Card, &Card)) -> Ordering {
    let order = "J23456789TQKA";
    match acc {
        Ordering::Less => Ordering::Less,
        Ordering::Greater => Ordering::Greater,
        Ordering::Equal => {
            let ixa = order.find(*a).unwrap();
            let ixb = order.find(*b).unwrap();
            ixa.cmp(&ixb)
        }
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
        println!("{:?}", self.cards);
        let mut histogram = HashMap::new();
        for c in self.cards.iter() {
            if *c == 'J' {
                continue;
            }
            histogram
                .entry(c)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
        // Joker of part 2
        let max_card = histogram.clone().into_iter().max_by_key(|(_k, v)| *v);

        match max_card {
            None => {
                // all jokers
                return HandType::FiveOfAKind;
            }
            Some(max_card) => {
                println!("{max_card:?}");

                // Joker of part 2
                let number_of_jokers = self.cards.iter().filter(|c| **c == 'J').count();
                println!("{number_of_jokers:?}");
                let max_card = (max_card.0, max_card.1 + number_of_jokers);
                println!("{max_card:?}");

                match max_card {
                    (_, 5) => HandType::FiveOfAKind,
                    (_, 4) => HandType::FourOfAKind,
                    (card, 3) => match histogram.iter().find(|(c, q)| **c != card && **q == 2) {
                        Some((_, 2)) => HandType::FullHouse,
                        _ => HandType::ThreeOfAKind,
                    },
                    (card, 2) => match histogram.iter().find(|(c, q)| **c != card && **q == 2) {
                        Some((_, 2)) => HandType::TwoPair,
                        _ => HandType::OnePair,
                    },
                    (_, 1) => HandType::HighCard,
                    (_, _) => panic!("too many equal cards"),
                }
            }
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
        println!("{a:?}");
        let b = other.get_type();
        println!("{b:?}");
        match a.partial_cmp(&b) {
            Some(std::cmp::Ordering::Equal) => Some(
                self.cards
                    .iter()
                    .zip(other.cards.iter())
                    .fold(Ordering::Equal, fold_cmp_card),
            ),
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
    let cards = input.chars().collect();
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
    return None;
    Some(
        games
            .iter()
            .enumerate()
            .map(|(ix, (_, bid))| (ix as u32 + 1) * bid)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut games = parse(input);
    games.sort_unstable_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap());
    for g in games.iter() {
        println!("{g:?}");
    }
    Some(
        games
            .iter()
            .enumerate()
            .map(|(ix, (_, bid))| (ix as u32 + 1) * bid)
            .sum(),
    )
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
        assert_eq!(result, Some(5905));
    }

    #[test]
    fn test_get_type() {
        assert_eq!(
            parse_hand("JJJJJ").unwrap().get_type(),
            HandType::FiveOfAKind
        );
        assert_eq!(
            parse_hand("JJ958").unwrap().get_type(),
            HandType::ThreeOfAKind
        );
    }

    #[test]
    fn test_cmp_hand() {
        let a = parse_hand("QQQJA").unwrap();
        let b = parse_hand("KTJJT").unwrap();
        assert_eq!(a.partial_cmp(&b), Some(Ordering::Less));
    }
}
