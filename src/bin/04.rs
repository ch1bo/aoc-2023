advent_of_code::solution!(4);

fn parse_numbers(input: &str) -> Vec<u32> {
    input
        .split(' ')
        .map(|s| s.parse::<u32>().ok())
        .flatten()
        .collect::<Vec<u32>>()
}

fn card_value(winning: Vec<u32>, numbers: Vec<u32>) -> u32 {
    let wins = numbers.iter().filter(|n| winning.contains(n)).count();
    if wins > 0 {
        2u32.pow(wins as u32 - 1)
    } else {
        0
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    input
        .lines()
        .map(|l| {
            let (_name, l) = l.split_once(':')?;
            let (winning_str, numbers_str) = l.split_once('|')?;
            let winning = parse_numbers(winning_str);
            let numbers = parse_numbers(numbers_str);
            println!("{winning:?} | {numbers:?}");
            Some(card_value(winning, numbers))
        })
        .sum()
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
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
