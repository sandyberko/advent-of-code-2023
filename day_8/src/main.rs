use std::collections::HashMap;

use num::Integer;

const INPUT: &str = include_str!("input.txt");

fn main() {
    println!("Step Count: {}", step_count(INPUT));
    println!("Ghost Step Count: {}", multi_path(INPUT));
}

fn step_count(input: &str) -> usize {
    let (inst, network) = input.split_once("\n\n").unwrap();

    let network = network
        .lines()
        .map(|line| {
            let (key, value) = line
                .split_once('=')
                .ok_or_else(|| format!("{line:?} does not have a '='"))
                .unwrap();
            let (left, right) = value
                .trim()
                .trim_start_matches('(')
                .trim_end_matches(')')
                .split_once(',')
                .unwrap();
            (key.trim(), (left.trim(), right.trim()))
        })
        .collect::<HashMap<_, _>>();

    let mut cur_node = "AAA";
    let mut step_count = 0;
    let mut insts = inst.chars().cycle();
    while cur_node != "ZZZ" {
        let inst = insts.next().unwrap();
        let (left, right) = network[cur_node];
        cur_node = match inst {
            'L' => left,
            'R' => right,
            inst => panic!("invalid instruction {inst:?}"),
        };
        step_count += 1;
    }
    step_count
}

fn multi_path(input: &str) -> usize {
    let (inst, network) = input.split_once("\n\n").unwrap();

    let network = network
        .lines()
        .map(|line| {
            let (key, value) = line
                .split_once('=')
                .ok_or_else(|| format!("{line:?} does not have a '='"))
                .unwrap();
            let (left, right) = value
                .trim()
                .trim_start_matches('(')
                .trim_end_matches(')')
                .split_once(',')
                .unwrap();
            (key.trim(), (left.trim(), right.trim()))
        })
        .collect::<HashMap<_, _>>();

    let mut insts = inst.chars().cycle();

    network
        .keys()
        .filter(|node| node.ends_with('A'))
        .copied()
        .map(|mut cur_node| {
            let mut step_count = 0;
            while !cur_node.ends_with('Z') {
                let inst = insts.next().unwrap();
                let (left, right) = network[cur_node];
                cur_node = match inst {
                    'L' => left,
                    'R' => right,
                    inst => panic!("invalid instruction {inst:?}"),
                };
                step_count += 1;
            }
            step_count
        })
        .reduce(|acc, cur| acc.lcm(&cur))
        .unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn navigate() {
        const INPUT: &str = r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"#;
        let step_count = super::step_count(INPUT);
        assert_eq!(step_count, 2)
    }

    #[test]
    fn navigate_2() {
        const INPUT: &str = r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"#;
        let step_count = super::step_count(INPUT);
        assert_eq!(step_count, 6)
    }

    #[test]
    fn multi_path() {
        const INPUT: &str = r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"#;

        let step_count = super::multi_path(INPUT);
        assert_eq!(step_count, 6)
    }
}
