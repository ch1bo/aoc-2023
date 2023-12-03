use std::cmp::{max, min};

use advent_of_code::ParseError;

advent_of_code::solution!(3);

#[derive(Debug)]
struct Plan {
    rows: Vec<Vec<char>>,
    parts: Vec<Part>,
    symbols: Vec<Symbol>,
}

#[derive(Debug)]
struct Part {
    number: u32,
    row: usize,
    start: usize,
    end: usize,
}

#[derive(Debug)]
struct Symbol {
    symbol: char,
    row: usize,
    col: usize,
}

type Pos = (usize, usize);

fn parse_plan(input: &str) -> Result<Plan, ParseError> {
    let rows = input.lines().map(|l| l.chars().collect()).collect();
    let mut parts = Vec::new();
    let mut symbols = Vec::new();
    for (y, r) in input.lines().enumerate() {
        let mut part_cur: Option<(usize, String)> = None;
        for (x, c) in r.char_indices() {
            match part_cur {
                Some((start, ref mut content)) => {
                    if c.is_digit(10) {
                        content.push(c);
                    } else {
                        let number = content.parse().map_err(|_| ParseError)?;
                        parts.push(Part {
                            number,
                            row: y,
                            start,
                            end: x,
                        });
                    }
                    continue;
                }
                None if c.is_digit(10) => {
                    part_cur = Some((x, String::from(c)));
                    continue;
                }
                _ => {}
            }

            if c == '.' {
                continue;
            } else {
                // symbol
                symbols.push(Symbol {
                    symbol: c,
                    row: y,
                    col: x,
                });
            }
        }
    }
    Ok(Plan {
        rows,
        parts,
        symbols,
    })
}

fn row_length(plan: &Plan) -> usize {
    plan.rows.len()
}

fn col_length(plan: &Plan) -> usize {
    plan.rows.iter().map(|r| r.len()).max().unwrap_or_default()
}

fn adjacent(plan: &Plan, pos: &Pos) -> Vec<Pos> {
    let (y, x) = pos;
    let row_above = max(y - 1, 0);
    let row_below = min(y + 1, row_length(plan));
    let col_left = max(x - 1, 0);
    let col_right = min(x + 1, col_length(plan));
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
    let plan = parse_plan(input).ok();
    println!("{plan:?}");
    // let test: Vec<Vec<Pos>> = s.iter().map(|x| adjacent(&plan, x)).collect();
    // println!("{test:?}");
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
