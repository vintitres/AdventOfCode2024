use std::collections::{BTreeSet, HashMap, HashSet};

use itertools::Itertools;

const COMP_NAME_START: char = 't';

fn char_letter_hash(c: char) -> u64 {
    c as u64 - 'a' as u64
}

fn computer_hash(name: &str) -> u64 {
    name.chars()
        .map(char_letter_hash)
        .reduce(|n, d| n * 100 + d)
        .unwrap()
}

fn unhash_char(hash: u64) -> char {
    (hash + 'a' as u64) as u8 as char
}

fn unhash(hash: &u64) -> String {
    unhash_char(hash / 100).to_string() + &String::from(unhash_char(hash % 100))

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

fn is_cliqe(net: &HashMap<u64, BTreeSet<u64>>, nodes: &HashSet<u64>) -> bool {
    for &node in nodes {
        for &node2 in nodes {
            if node == node2 {
                continue;
            }
            if !net.get(&node).unwrap().contains(&node2) {
                return false;
            }
        }
    }
    true
}

pub fn subsets(set: &BTreeSet<u64>, size: usize) -> Vec<HashSet<u64>> {
    if size == 1 {
        return set.iter().map(|node| HashSet::from([*node])).collect_vec();
    }
    let mut ret = Vec::new();
    let mut set2 = set.clone();
    for &node in set {
        set2.remove(&node);
        for mut subset in subsets(&set2, size - 1) {
            subset.insert(node);
            ret.push(subset);
        }
    }
    // dbg!(ret.len());
    ret
}

pub fn part2(input: &str) -> String {
    let net = read_net(input);
    let mut best = HashSet::new();
    // dbg!(&net);
    // dbg!(net.len());
    for &comp1 in net.keys() {
        // dbg!(comp1);
        let comps = net.get(&comp1).unwrap();
        // dbg!(comps.len());
        for i in 1..=comps.len() {
            // dbg!(i);
            let mut found = false;
            let subs = subsets(comps, i);
            // dbg!(subs.len());
            for mut nodes in subs {
                // dbg!(&nodes, i);
                nodes.insert(comp1);
                if is_cliqe(&net, &nodes) {
                    found = true;
                    if nodes.len() > best.len() {
                        best = nodes;
                    }
                    break;
                }
            }
            if !found {
                break;
            }
        }


    }
    best.iter().map(unhash).sorted().join(",")
}

pub fn subsetss() {

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

    #[ignore = "slow"]
    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), String::from("am,aq,by,ge,gf,ie,mr,mt,rw,sn,te,yi,zb"));
    }
}
