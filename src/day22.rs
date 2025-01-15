const MOD: u64 = 16777216;

fn nth_secret(mut secret: u64, n: usize) -> u64 {
    for _ in 0..n {
        secret = ((secret * 64) ^ secret) % MOD;
        secret = ((secret / 32) ^ secret) % MOD;
        secret = ((secret * 2048) ^ secret) % MOD;
    }
    secret
}

pub fn part1(input: &str) -> u64 {
    input.lines().map(|line| line.parse::<u64>().unwrap()).map(|secret| nth_secret(secret, 2000)).sum()
}

pub fn part2(input: &str) -> u64 {
    input.lines().count() as u64
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
        assert_eq!(part1(input()), 13004408787);  // ?
    }

    #[ignore = "not implemented"]
    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 25574739);
    }
}
