use std::collections::HashMap;

use itertools::Itertools;

fn read_input(input: &str) -> (Vec<u64>, Vec<u64>) {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|n| n.parse::<u64>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .unzip()
}

pub fn part1(input: &str) -> u64 {
    let (mut left, mut right) = read_input(input);
    left.sort();
    right.sort();
    left.into_iter()
        .zip(right)
        .map(|(l, r)| if l > r { l - r } else { r - l })
        .sum()
}

pub fn part2(input: &str) -> u64 {
    let (left, right) = read_input(input);
    let mut right_count = HashMap::<u64, usize>::new();
    for n in right {
        *right_count.entry(n).or_insert(0) += 1;
    }
    left.iter()
        .map(|&n| *right_count.entry(n).or_insert(0) as u64 * n)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2024/day1.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 1603498);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 25574739);
    }
}
