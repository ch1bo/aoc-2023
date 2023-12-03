use std::cmp::{max, min};

advent_of_code::solution!(3);

type Plan = Vec<Vec<char>>;

type Pos = (usize, usize);

fn parse_plan(input: &str) -> Plan {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn symbols(plan: &Plan) -> Vec<Pos> {
    plan.iter()
        .enumerate()
        .filter_map(|(y, row)| {
            Some(
                row.iter()
                    .enumerate()
                    .filter_map(|(x, cell)| {
                        if cell.is_digit(10) || *cell == '.' {
                            None
                        } else {
                            Some((y, x))
                        }
                    })
                    .collect::<Vec<Pos>>(),
            )
        })
        .flatten()
        .collect()
}

fn rows(plan: &Plan) -> usize {
    plan.len()
}

fn cols(plan: &Plan) -> usize {
    plan.iter().map(|r| r.len()).max().unwrap_or_default()
}

fn adjacent(plan: &Plan, pos: &Pos) -> Vec<Pos> {
    let (y, x) = pos;
    let row_above = max(y - 1, 0);
    let row_below = min(y + 1, rows(plan));
    let col_left = max(x - 1, 0);
    let col_right = min(x + 1, cols(plan));
    vec![
        (row_above, col_left),
        (row_above, *x),
        (row_above, col_right),
        (*y, col_left),
        (*y, col_right),
        (row_below, col_left),
        (row_below, *x),
        (row_below, col_right),
    ]
}

pub fn part_one(input: &str) -> Option<u32> {
    let plan = parse_plan(input);
    println!("{plan:?}");
    let s = symbols(&plan);
    println!("{s:?}");
    let test: Vec<Vec<Pos>> = s.iter().map(|x| adjacent(&plan, x)).collect();
    println!("{test:?}");
    None
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
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
