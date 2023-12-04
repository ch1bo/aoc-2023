advent_of_code::solution!(4);

fn parse_numbers(input: &str) -> Vec<u32> {
    input
        .split(' ')
        .map(|s| s.parse::<u32>().ok())
        .flatten()
        .collect::<Vec<u32>>()
}

fn winning_numbers(winning: &Vec<u32>, numbers: &Vec<u32>) -> u32 {
    numbers.iter().filter(|n| winning.contains(n)).count() as u32
}

fn card_value(winning: &Vec<u32>, numbers: &Vec<u32>) -> u32 {
    let wins = winning_numbers(winning, numbers);
    if wins > 0 {
        2u32.pow(wins as u32 - 1)
    } else {
        0
    }
}

fn parse_cards(input: &str) -> Vec<Card> {
    input
        .lines()
        .map(|l| {
            let (_name, l) = l.split_once(':')?;
            let (winning_str, numbers_str) = l.split_once('|')?;
            let winning = parse_numbers(winning_str);
            let numbers = parse_numbers(numbers_str);
            Some((winning, numbers))
        })
        .flatten()
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    parse_cards(input)
        .iter()
        .map(|(winning, numbers)| Some(card_value(winning, numbers)))
        .sum()
}

type Card = (Vec<u32>, Vec<u32>);
type CardStack = Vec<(Card, u32)>;

fn process(cardstack: &mut CardStack, index: usize) -> () {
    if index >= cardstack.len() {
        return ();
    }
    let ((winning, numbers), count) = cardstack[index].clone();
    let new_cards = winning_numbers(&winning, &numbers) as usize;
    for i in 1..=new_cards {
        if let Some((_, next_count)) = cardstack.get_mut(index + i) {
            *next_count += count;
        }
    }
    process(cardstack, index + 1);
}

pub fn part_two(input: &str) -> Option<u32> {
    let cards = parse_cards(input);
    let mut cardstack = cards.iter().map(|c| (c.clone(), 1)).collect();
    process(&mut cardstack, 0);
    Some(cardstack.iter().map(|(_, count)| count).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
