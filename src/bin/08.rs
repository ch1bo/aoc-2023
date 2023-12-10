use std::{collections::HashMap, iter::Cycle};

advent_of_code::solution!(8);

#[derive(Debug)]
enum Step {
    R,
    L,
}

type Instructions = Vec<Step>;

type Node = String;

#[derive(Debug)]
struct Network {
    nodes: HashMap<Node, (Node, Node)>,
}

fn parse(input: &str) -> (Instructions, Network) {
    let (instructions, network) = input.split_once("\n\n").unwrap();
    let instructions = instructions
        .chars()
        .map(|c| match c {
            'R' => Step::R,
            'L' => Step::L,
            _ => panic!("invalid step: {}", c),
        })
        .collect();
    let mut nodes = HashMap::new();
    for s in network.split('\n') {
        if s == "" {
            continue;
        }
        let (n, (l, r)) = parse_node(s);
        nodes.insert(n, (l, r));
    }
    (instructions, Network { nodes })
}

fn parse_node(s: &str) -> (Node, (Node, Node)) {
    let (n, lr) = s.split_once('=').unwrap();
    let (l, r) = lr
        .trim()
        .strip_prefix("(")
        .unwrap()
        .strip_suffix(")")
        .unwrap()
        .split_once(", ")
        .unwrap();
    (n.trim().to_string(), (l.to_string(), r.to_string()))
}

pub fn part_one(input: &str) -> Option<u32> {
    let (ins, network) = parse(input);
    println!("{ins:?}");
    println!("{network:?}");
    Some(navigate(&network, ins.iter().cycle(), &String::from("AAA")))
}

fn navigate<'a, I: Iterator<Item = &'a Step>>(
    network: &Network,
    mut instructions: I,
    cur: &Node,
) -> u32 {
    if cur == "ZZZ" {
        return 0;
    }
    let (l, r) = match network.nodes.get(cur) {
        None => return 0,
        Some(x) => x,
    };
    match instructions.next() {
        None => return 0,
        Some(Step::L) => 1 + navigate(network, instructions, l),
        Some(Step::R) => 1 + navigate(network, instructions, r),
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input1 = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(part_one(input1), Some(2));
        let input2 = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(part_one(input2), Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
