use std::cmp::min;

use itertools::Itertools;

fn read_coords(line: &str, plus: u64) -> (u64, u64) {
    let (_, coords) = line.split_once(": ").unwrap();
    coords
        .split(", ")
        .map(|coord| coord[2..].parse::<u64>().unwrap() + plus)
        .collect_tuple()
        .unwrap()
}

fn min_tokens_1(button_a: (u64, u64), button_b: (u64, u64), prize: (u64, u64)) -> Option<u64> {
    let mut min_coins = None;
    for press_a in 0..=100 {
        let pos_after_a = (button_a.0 * press_a, button_a.1 * press_a);
        if pos_after_a.0 > prize.0 || pos_after_a.1 > prize.1 {
            break;
        }
        let press_b = (prize.0 - pos_after_a.0) / button_b.0;
        if pos_after_a.0 + press_b * button_b.0 == prize.0
            && pos_after_a.1 + press_b * button_b.1 == prize.1
        {
            dbg!(press_a);
            min_coins = Some(min(
                min_coins.unwrap_or(u64::MAX),
                press_a as u64 * 3 + press_b as u64,
            ))
        }
    }
    min_coins
}

fn doit(input: &str, plus: u64) -> u64 {
    input
        .lines()
        .chunks(4)
        .into_iter()
        .map(|mut lines| {
            let button_a = read_coords(lines.next().unwrap(), 0);
            let button_b = read_coords(lines.next().unwrap(), 0);
            let prize = read_coords(lines.next().unwrap(), plus);
            // dbg!(min_tokens_1(button_a, button_b, prize));
            // dbg!(min_tokens(button_a, button_b, prize));
            min_tokens_1(button_a, button_b, prize).unwrap_or(0)
        })
        .sum()
}

pub fn part1(input: &str) -> u64 {
    doit(input, 0)
}

pub fn part2(input: &str) -> u64 {
    doit(input, 10000000000000)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2024/day13.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 31761);
    }

    #[ignore = "not implemented"]
    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 25574739);
    }
}
