use std::collections::HashMap;

const MOD: u64 = 16777216;

fn nth_secret(secret: u64, n: usize, mem: &mut HashMap<(u64, usize), u64>) -> u64 {
    if n == 0 {
        return secret;
    }
    match mem.get(&(secret, n)) {
        Some(s) => *s,
        None => {
            let mut new_secret = secret;

            new_secret = ((new_secret * 64) ^ new_secret) % MOD;
            new_secret = ((new_secret / 32) ^ new_secret) % MOD;
            new_secret = ((new_secret * 2048) ^ new_secret) % MOD;

            new_secret = nth_secret(new_secret, n - 1, mem);

            mem.insert((secret, n), new_secret);

            new_secret
        }
    }
}

pub fn part1(input: &str) -> u64 {
    let mut mem = HashMap::new();
    input
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .map(|secret| nth_secret(secret, 2000, &mut mem))
        .sum()
}

pub fn part2(input: &str) -> u64 {
    let mut mem = HashMap::new();
    input
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .map(|secret| nth_secret(secret, 1, &mut mem))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2024/day22.txt")
    }

    #[ignore = "not implemented"]
    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 13004408787); // ?
    }

    #[ignore = "not implemented"]
    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 25574739);
    }
}
