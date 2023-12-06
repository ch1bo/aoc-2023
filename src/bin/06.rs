advent_of_code::solution!(6);

#[derive(Debug)]
struct Game {
    time: u64,
    distance: u64,
}

fn parse_games(input: &str) -> Vec<Game> {
    let mut lines = input.lines();
    let times = lines
        .next()
        .expect("missing first line")
        .strip_prefix("Time:")
        .expect("no prefix")
        .split(' ')
        .flat_map(|s| s.trim().parse::<u64>());
    let distances = lines
        .next()
        .expect("missing first line")
        .strip_prefix("Distance:")
        .expect("no prefix")
        .split(' ')
        .flat_map(|s| s.trim().parse::<u64>());

    times
        .zip(distances)
        .map(|(time, distance)| Game { time, distance })
        .collect()
}

fn winning_times(game: &Game) -> Vec<u64> {
    (0..game.time)
        .filter(|ms| travel_distance(*ms, game.time - ms) > game.distance)
        .collect()
}

fn travel_distance(holding_time: u64, duration: u64) -> u64 {
    holding_time * duration
}

pub fn part_one(input: &str) -> Option<u64> {
    let games = parse_games(input);
    println!("{games:?}");
    Some(
        games
            .iter()
            .map(|g| winning_times(g).len())
            .product::<usize>()
            .try_into()
            .expect("truncated"),
    )
}

fn parse_game(input: &str) -> Game {
    let mut lines = input.lines();
    let time = lines
        .next()
        .expect("missing first line")
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse::<u64>()
        .ok()
        .expect("parse time failed");
    let distance = lines
        .next()
        .expect("missing first line")
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse::<u64>()
        .ok()
        .expect("parse distance failed");
    Game { time, distance }
}

pub fn part_two(input: &str) -> Option<u64> {
    let game = parse_game(input);
    println!("{game:?}");
    Some(winning_times(&game).len().try_into().expect("truncated"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
