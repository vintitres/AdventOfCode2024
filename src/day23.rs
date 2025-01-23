use std::collections::{BTreeSet, HashMap, HashSet};

use itertools::Itertools;

const COMP_NAME_START: char = 't';

fn char_letter_hash(c: char) -> u64 {
    match c {
        'a' => 0,
        'b' => 1,
        'c' => 2,
        'd' => 3,
        'e' => 4,
        'f' => 5,
        'g' => 6,
        'h' => 7,
        'i' => 8,
        'j' => 9,
        'k' => 10,
        'l' => 11,
        'm' => 12,
        'n' => 13,
        'o' => 14,
        'p' => 15,
        'q' => 16,
        'r' => 17,
        's' => 18,
        't' => 19,
        'u' => 20,
        'v' => 21,
        'w' => 22,
        'x' => 23,
        'y' => 24,
        'z' => 25,
        _ => unimplemented!(),
    }
}

fn computer_hash(name: &str) -> u64 {
    name.chars()
        .map(char_letter_hash)
        .reduce(|n, d| n * 100 + d)
        .unwrap()
}

fn read_net(input: &str) -> HashMap<u64, BTreeSet<u64>> {
    let mut ret = HashMap::new();
    input.lines().for_each(|line| {
        let (l, r) = line.split('-').map(computer_hash).collect_tuple().unwrap();
        ret.entry(l).or_insert(BTreeSet::new()).insert(r);
        ret.entry(r).or_insert(BTreeSet::new()).insert(l);
    });
    ret
}

pub fn part1(input: &str) -> usize {
    let mut triples: HashSet<(u64, u64, u64)> = HashSet::new();
    let net = read_net(input);
    for &comp1 in net.keys() {
        if comp1 / 100 != char_letter_hash(COMP_NAME_START) {
            continue;
        }
        for &comp2 in net.get(&comp1).unwrap() {
            if comp1 == comp2 {
                continue;
            }
            for &comp3 in net.get(&comp2).unwrap() {
                if comp2 == comp3 {
                    continue;
                }
                if net.get(&comp3).unwrap().contains(&comp1) {
                    triples.insert(
                        vec![comp1, comp2, comp3]
                            .into_iter()
                            .sorted()
                            .collect_tuple()
                            .unwrap(),
                    );
                }
            }
        }
    }
    triples.len()
}

pub fn part2(input: &str) -> u64 {
    input.lines().count() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2024/day23.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 1240);
    }

    #[ignore = "not implemented"]
    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 25574739);
    }
}
