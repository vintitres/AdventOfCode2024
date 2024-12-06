use std::collections::{HashMap, HashSet};

use itertools::Itertools;

type Rules = HashMap<u64, HashSet<u64>>;

fn read_rules(input: &str) -> Rules {
    let mut rules = Rules::new();
    input
        .lines()
        .take_while(|l| !l.is_empty())
        .map(|l| {
            l.split('|')
                .map(|num| num.parse::<u64>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .for_each(|(n1, n2)| {
            rules.entry(n2).or_default().insert(n1);
        });
    rules
}

fn read_update(line: &str) -> Vec<u64> {
    line.split(',').map(|n| n.parse::<u64>().unwrap()).collect()
}

pub fn part1(input: &str) -> u64 {
    let rules = read_rules(input);
    input
        .lines()
        .skip_while(|l| !l.is_empty())
        .skip(1)
        .map(|l| {
            let update = read_update(l);
            let mut all = HashSet::<u64>::from_iter(update.iter().cloned());
            let mid_i = update.len() / 2;
            let mut mid = 0;
            for (i, n) in update.iter().enumerate() {
                if i == mid_i {
                    mid = *n;
                }
                if let Some(deps) = rules.get(n) {
                    for dep in deps {
                        if all.contains(dep) {
                            return 0;
                        }
                    }
                }
                all.remove(n);
            }
            mid
        })
        .sum()
}

pub fn part2(input: &str) -> u64 {
    input.lines().count() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2024/day5.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 7365);
    }

    #[ignore = "not implemented"]
    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 5770);
    }
}
