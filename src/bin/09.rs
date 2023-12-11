advent_of_code::solution!(9);

fn derivates(seq: &Vec<i32>) -> Vec<i32> {
    seq.windows(2)
        .flat_map(|slice| {
            let a = slice.first()?;
            let b = slice.last()?;
            Some(b - a)
        })
        .collect()
}

fn all_zeros(seq: &Vec<i32>) -> bool {
    seq.iter().all(|x| *x == 0)
}

fn extrapolate(seq: &Vec<i32>) -> i32 {
    let last = *seq.last().unwrap();
    let ds = derivates(seq);
    if all_zeros(&ds) {
        last
    } else {
        last + extrapolate(&ds)
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let sequences: Vec<Vec<i32>> = input
        .lines()
        .map(|l| l.split(' ').flat_map(|s| s.parse()).collect())
        .collect();
    println!("{sequences:?}");
    for s in sequences.iter() {
        println!("{:?}", extrapolate(&s));
    }
    Some(sequences.iter().map(extrapolate).sum())
}

pub fn part_two(input: &str) -> Option<i32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_derivatives() {
        assert_eq!(derivates(&vec![0, 3, 6, 9, 12, 15]), vec![3, 3, 3, 3, 3]);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
