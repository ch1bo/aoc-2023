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

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
