use std::{
    cmp::{max, min},
    str::FromStr,
};

use advent_of_code::ParseError;

advent_of_code::solution!(3);

#[derive(Debug)]
struct Plan {
    rows: Vec<Vec<char>>,
    parts: Vec<Part>,
}

#[derive(Debug)]
struct Part {
    number: u32,
    pos: Pos,
}

impl FromStr for Part {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let number = s.parse().map_err(|_| ParseError)?;
        Ok(Part {
            number,
            pos: (0, 0),
        })
    }
}

type Pos = (usize, usize);

impl FromStr for Plan {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let rows = input.lines().map(|l| l.chars().collect()).collect();
        let parts: Vec<Part> = input
            .lines()
            .map(|l| {
                let chars_it = l.chars();

                let part_str: String = chars_it
                    .skip_while(|c| *c == '.')
                    .take_while(|c| *c != '.')
                    .collect();
                part_str.parse::<Part>()
            })
            .flatten()
            .collect();
        Ok(Plan { rows, parts })
    }
}

fn symbols(plan: &Plan) -> Vec<Pos> {
    plan.rows
        .iter()
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
    plan.rows.len()
}

fn cols(plan: &Plan) -> usize {
    plan.rows.iter().map(|r| r.len()).max().unwrap_or_default()
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
    let plan = input.parse().ok()?;
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
