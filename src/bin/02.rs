use std::{num::ParseIntError, str::FromStr};

advent_of_code::solution!(2);

#[derive(Debug, PartialEq, Eq)]
struct ParseError;

#[derive(Debug)]
struct Game {
    id: u32,
    draws: Vec<Draw>,
}

impl FromStr for Game {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (gs, rs) = s.split_once(':').ok_or(ParseError)?;
        let id = gs
            .strip_prefix("Game ")
            .ok_or(ParseError)
            .and_then(|s| s.parse::<u32>().map_err(|_| ParseError))?;

        let draws: Result<Vec<Draw>, ParseError> = rs
            .split(';')
            .map(|s| s.trim().parse().map_err(|_| ParseError))
            .collect();
        // TODO: without type annotation and ? in let?
        Ok(Game { id, draws: draws? })
    }
}

#[derive(Debug)]
struct Draw {
    red: u32,
    green: u32,
    blue: u32,
}

impl FromStr for Draw {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(',').map(|s| s.trim()).collect::<Vec<&str>>();
        // TODO: avoid muts / for loop
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for part in parts {
            let (left, right) = part.split_once(' ').ok_or(ParseError)?;
            let value = left.parse().map_err(|_| ParseError)?;
            match right {
                "red" => red = value,
                "green" => green = value,
                "blue" => blue = value,
                _ => return Err(ParseError),
            }
        }
        return Ok(Draw { red, green, blue });
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let games: Vec<Game> = input.lines().map(|l| l.parse::<Game>()).flatten().collect();
    let x = games
        .into_iter()
        .filter(|g| {
            g.draws
                .iter()
                .all(|d| d.red <= 12 && d.green <= 13 && d.blue <= 14)
        })
        .map(|g| g.id)
        .sum();
    // TODO: pure in rust?
    Some(x)
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
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
