use std::{cmp::Ordering, collections::HashMap};

advent_of_code::solution!(7);

type Card = char;

fn fold_cmp_card(order: &str, acc: Ordering, (a, b): (&Card, &Card)) -> Ordering {
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

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
}

fn get_type(hand: &Hand, with_joker: bool) -> HandType {
    let mut histogram = HashMap::new();
    for c in hand.cards.iter() {
        if with_joker && *c == 'J' {
            continue;
        }
        histogram
            .entry(c)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }
    match histogram.clone().into_iter().max_by_key(|(_k, v)| *v) {
        None => {
            // all jokers
            return HandType::FiveOfAKind;
        }
        Some(mut max_card) => {
            if with_joker {
                let number_of_jokers = hand.cards.iter().filter(|c| **c == 'J').count();
                max_card.1 += number_of_jokers;
            }

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

fn cmp_hands(order: &str, a: &Hand, b: &Hand) -> Option<std::cmp::Ordering> {
    let with_joker = order.starts_with("J");
    let at = get_type(a, with_joker);
    let bt = get_type(b, with_joker);
    match at.partial_cmp(&bt) {
        Some(std::cmp::Ordering::Equal) => Some(
            a.cards
                .iter()
                .zip(b.cards.iter())
                .fold(Ordering::Equal, |acc, x| fold_cmp_card(order, acc, x)),
        ),
        res => res,
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
    let order = "23456789TJQKA";
    let mut games = parse(input);
    games.sort_unstable_by(|(a, _), (b, _)| cmp_hands(order, a, b).unwrap());
    Some(
        games
            .iter()
            .enumerate()
            .map(|(ix, (_, bid))| (ix as u32 + 1) * bid)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let order = "J23456789TQKA";
    let mut games = parse(input);
    games.sort_unstable_by(|(a, _), (b, _)| cmp_hands(order, a, b).unwrap());
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
            get_type(&parse_hand("JJJJJ").unwrap(), true),
            HandType::FiveOfAKind
        );
        assert_eq!(
            get_type(&parse_hand("JJ958").unwrap(), false),
            HandType::OnePair
        );
        assert_eq!(
            get_type(&parse_hand("JJ958").unwrap(), true),
            HandType::ThreeOfAKind
        );
    }

    #[test]
    fn test_cmp_hand() {
        let order = "J23456789TQKA";
        let a = parse_hand("QQQJA").unwrap();
        let b = parse_hand("KTJJT").unwrap();
        assert_eq!(cmp_hands(order, &a, &b), Some(Ordering::Less));
    }
}
