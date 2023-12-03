use advent_of_code::ParseError;

advent_of_code::solution!(3);

#[derive(Debug)]
struct Plan {
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

impl Part {
    fn is_adjacent_to_symbol(self: &Self, symbols: &Vec<Symbol>) -> bool {
        symbols
            .iter()
            .filter(|s| (self.row.saturating_sub(1)..=self.row.saturating_add(1)).contains(&s.row))
            .find(|s| (self.start.saturating_sub(1)..=self.end.saturating_add(1)).contains(&s.col))
            .is_some()
    }
}

#[derive(Debug)]
struct Symbol {
    symbol: char,
    row: usize,
    col: usize,
}

impl Symbol {
    fn gear_ratio(self: &Self, parts: &Vec<Part>) -> Option<u32> {
        if self.symbol != '*' {
            return None;
        }

        let adjacent_parts: Vec<&Part> = parts
            .iter()
            .filter(|p| (p.row.saturating_sub(1)..=p.row.saturating_add(1)).contains(&self.row))
            .filter(|p| (p.start.saturating_sub(1)..=p.end.saturating_add(1)).contains(&self.col))
            .collect();

        if adjacent_parts.len() != 2 {
            return None;
        }

        Some(adjacent_parts.iter().map(|p| p.number).product())
    }
}

fn parse_plan(input: &str) -> Result<Plan, ParseError> {
    let mut parts = Vec::new();
    let mut symbols = Vec::new();
    for (y, r) in input.lines().enumerate() {
        let mut part_cur: Option<(usize, String)> = None;
        for (x, c) in r.char_indices() {
            if c.is_digit(10) {
                match part_cur {
                    None => {
                        part_cur = Some((x, String::from(c)));
                    }
                    Some((start, ref mut content)) => {
                        content.push(c);
                        if x + 1 >= r.len() {
                            let number = content.parse().map_err(|_| ParseError)?;
                            parts.push(Part {
                                number,
                                row: y,
                                start,
                                end: x,
                            });
                            part_cur = None;
                        }
                    }
                }
                continue;
            }

            // FIXME: the fact that we are identifying parts twice is hacky
            match part_cur {
                Some((start, ref mut content)) => {
                    let number = content.parse().map_err(|_| ParseError)?;
                    parts.push(Part {
                        number,
                        row: y,
                        start,
                        end: x - 1,
                    });
                    part_cur = None;
                }
                None => {}
            }

            if c != '.' {
                // symbol
                symbols.push(Symbol {
                    symbol: c,
                    row: y,
                    col: x,
                });
            }
        }
    }
    Ok(Plan { parts, symbols })
}

pub fn part_one(input: &str) -> Option<u32> {
    let plan = parse_plan(input).ok()?;
    let parts: Vec<Part> = plan
        .parts
        .into_iter()
        .filter(|p| p.is_adjacent_to_symbol(&plan.symbols))
        .collect();
    let sum = parts.iter().map(|p| p.number).sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let plan = parse_plan(input).ok()?;
    let gear_ratios: Vec<u32> = plan
        .symbols
        .into_iter()
        .map(|s| s.gear_ratio(&plan.parts))
        .flatten()
        .collect();
    Some(gear_ratios.iter().sum())
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
        assert_eq!(result, Some(467835));
    }
}
