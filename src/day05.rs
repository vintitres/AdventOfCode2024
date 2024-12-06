use std::collections::{HashMap, HashSet, VecDeque};

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

fn check(update: &[u64], rules: &Rules) -> u64 {
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
}

pub fn part1(input: &str) -> u64 {
    doit(input, &check)
}

pub fn doit(input: &str, score_fn: &dyn Fn(&[u64], &Rules) -> u64) -> u64 {
    let rules = read_rules(input);
    input
        .lines()
        .skip_while(|l| !l.is_empty())
        .skip(1)
        .map(read_update)
        .map(|update| score_fn(&update, &rules))
        .sum()
}

fn fix(update: &[u64], rules: &Rules) -> u64 {
    if check(update, rules) > 0 {
        return 0;
    }
    let mut new_update = Vec::<u64>::new();
    let mut all = HashSet::<u64>::from_iter(update.iter().cloned());
    let mut stack = VecDeque::<u64>::from_iter(update.iter().rev().cloned());
    let mut done = HashSet::<u64>::new();
    while !stack.is_empty() {
        let num = *stack.front().unwrap();
        if done.contains(&num) {
            stack.pop_front();
            continue;
        }
        if let Some(deps) = rules.get(&num) {
            let mut any_dep_added = false;
            for dep in deps {
                if all.contains(dep) {
                    stack.push_front(*dep);
                    any_dep_added = true;
                }
            }
            if any_dep_added {
                continue;
            }
        }

        stack.pop_front();
        done.insert(num);
        all.remove(&num);
        new_update.push(num);
    }
    *new_update.get(new_update.len() / 2).unwrap()
}

pub fn part2(input: &str) -> u64 {
    doit(input, &fix)
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

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 5770);
    }
}
