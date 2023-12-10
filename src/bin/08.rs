use std::collections::HashMap;

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
    let mut it = ins.iter().cycle();
    let mut count = 0;
    let mut cur = &String::from("AAA");
    while let Some(step) = it.next() {
        let next = navigate_step(&network, step, &cur);
        count += 1;
        if next == "ZZZ" {
            return Some(count);
        }
        cur = next;
    }
    None
}

fn navigate_step<'a>(network: &'a Network, step: &'a Step, cur: &'a Node) -> &'a Node {
    let (l, r) = match network.nodes.get(cur) {
        None => panic!("not found: {}", cur),
        Some(x) => x,
    };
    match step {
        Step::L => l,
        Step::R => r,
    }
}

fn find_starts(network: &Network) -> Vec<&Node> {
    network
        .nodes
        .keys()
        .clone()
        .filter(|n| n.ends_with("A"))
        .collect()
}

pub fn part_two(input: &str) -> Option<u32> {
    let (ins, network) = parse(input);
    let mut curs = find_starts(&network);
    println!("{curs:?}");
    let mut it = ins.iter().cycle();
    let mut count = 0;
    while let Some(step) = it.next() {
        count += 1;
        let nexts = curs.iter().map(|n| navigate_step(&network, step, n));
        // TODO: why clone needed? why mut on all?
        if nexts.clone().all(|n| n.ends_with("Z")) {
            return Some(count);
        }
        curs = nexts.collect();
    }
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
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        let result = part_two(&input);
        assert_eq!(result, Some(6));
    }
}
