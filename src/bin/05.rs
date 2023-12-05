advent_of_code::solution!(5);

fn map(dst_start: u32, src_start: u32, len: u32) -> impl Fn(u32) -> Option<u32> {
    move |x| {
        if x > src_start && x < src_start + len {
            Some(dst_start + (x - src_start))
        } else {
            None
        }
    }
}

fn compose_any(
    functions: Vec<impl Fn(u32) -> Option<u32> + 'static>,
) -> impl Fn(u32) -> u32 + 'static {
    move |x| match functions.iter().find_map(|f| f(x)) {
        None => x,
        Some(res) => res,
    }
}

fn compose_all(functions: Vec<impl Fn(u32) -> u32 + 'static>) -> impl Fn(u32) -> u32 + 'static {
    move |x| functions.iter().fold(x, |x, f| f(x))
}

fn parse_map(input: &str) -> Option<impl Fn(u32) -> u32> {
    let vec = input
        .lines()
        .flat_map(|l| {
            let nums: Vec<_> = l.split(' ').collect();
            let dst = nums[0].parse::<u32>().ok()?;
            let src = nums[1].parse::<u32>().ok()?;
            let len = nums[2].parse::<u32>().ok()?;
            Some(map(dst, src, len))
        })
        .collect::<Vec<_>>();
    Some(compose_any(vec))
}

pub fn part_one(input: &str) -> Option<u32> {
    let (seed_line, rest) = input.split_once("\n\n")?;

    let seeds: Vec<u32> = seed_line
        .split(' ')
        .flat_map(|s| s.parse::<u32>())
        .collect();

    let full_map = compose_all(
        rest.split("\n\n")
            .map(|block| parse_map(block).unwrap())
            .collect(),
    );

    seeds
        .iter()
        .map(|x| {
            println!("Seed {x}");
            full_map(*x)
        })
        .min()
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
