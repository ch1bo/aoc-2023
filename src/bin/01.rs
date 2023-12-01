use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn calibration_value(line: &str) -> u32 {
    let mut cs = line.chars();

    let first = cs.find(|x| x.is_ascii_digit()).and_then(|c| c.to_digit(10));
    let last = cs
        .rfind(|x| x.is_ascii_digit())
        .and_then(|c| c.to_digit(10));

    let a = first.or(last);
    let b = last.or(first);

    a.unwrap() * 10 + b.unwrap()
}

pub fn part_one(input: &str) -> Option<u32> {
    let res = input.lines().map(calibration_value).sum();
    Some(res)
}

pub fn calibration_value2(line: &str) -> u32 {
    let numbers = HashMap::from([
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let mut first = None;
    let mut first_ix = line.len();
    for (key, value) in numbers.iter() {
        match line.find(key) {
            Some(i) if i <= first_ix => {
                first_ix = i;
                first = Some(value);
            }
            _ => {}
        }
        continue;
    }
    let mut last = None;
    let mut last_ix = 0;
    for (key, value) in numbers.iter() {
        match line.rfind(key) {
            Some(i) if i >= last_ix => {
                last_ix = i;
                last = Some(value);
            }
            _ => {}
        }
        continue;
    }
    first.unwrap() * 10 + last.unwrap()
}

pub fn part_two(input: &str) -> Option<u32> {
    let res = input.lines().map(|x| calibration_value2(x)).sum();
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        println!("{:?}", result);
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}
